use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    inventory::{item::ItemType, Inventory}, mouse_position::MousePosition, ui::mode_manager::UiState, world::{
        chunk::block::{Block, BlockDatabase, BlockLayer},
        SetBlock,
        World
    }, BLOCK_SIZE_PX};

use super::{hotbar::Hotbar, Player};

pub struct BlockInteractionPlugin;

impl Plugin for BlockInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedBlock>();
        
        app
            .add_systems(OnEnter(UiState::InGame), spawn_selection_box)
            .add_systems(Update, (
                toggle_selection_mode,
                update_selected_position,
                (move_selection_box, break_blocks, place_blocks)
                    .run_if(in_state(UiState::InGame))
            ));
    }
}

#[derive(Component)]
struct BlockSelectionBox;

fn spawn_selection_box(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(Color::WHITE.with_alpha(0.5))),
        Transform::from_scale(Vec3::splat(BLOCK_SIZE_PX + 4.)),
        BlockSelectionBox,
        StateScoped(UiState::InGame)
    ));
}

#[derive(PartialEq)]
enum BlockSelectionMode {
    Free,
    Raycasting,
}

impl Default for BlockSelectionMode {
    fn default() -> Self {
        Self::Raycasting
    }
}

#[derive(Resource, Default)]
struct SelectedBlock {
    position: Vec2,
    selection_mode: BlockSelectionMode,
}

fn toggle_selection_mode(
    mut selected: ResMut<SelectedBlock>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Tab) {
        match selected.selection_mode {
            BlockSelectionMode::Free => selected.selection_mode = BlockSelectionMode::Raycasting,
            BlockSelectionMode::Raycasting => selected.selection_mode = BlockSelectionMode::Free,
        }
    }
}

fn update_selected_position(
    q_player: Query<&Transform, (With<Player>, Without<BlockSelectionBox>)>, 
    q_rapier_context: Query<&RapierContext>,
    mouse_position: Res<MousePosition>,
    mut selected: ResMut<SelectedBlock>,
) {
    if selected.selection_mode == BlockSelectionMode::Raycasting {
        let Ok(rapier_context) = q_rapier_context.get_single() else { return };
        let Ok(player_transform) = q_player.get_single() else { return };
    
        let ray_dir = (mouse_position.0 - player_transform.translation.truncate()).normalize();
    
        let Some((_, hit)) = rapier_context.cast_ray_and_get_normal(
            player_transform.translation.truncate(),
            ray_dir,
            100.0,
            true,
            QueryFilter::exclude_dynamic())
        else { return };
    
        selected.position = 
            ((hit.point - hit.normal * BLOCK_SIZE_PX/2.)
            / BLOCK_SIZE_PX).floor() * BLOCK_SIZE_PX;
    }
    else {
        selected.position = (mouse_position.0 / BLOCK_SIZE_PX).floor() * BLOCK_SIZE_PX;
    }
}

fn move_selection_box(
    mut q_selection: Query<&mut Transform, (With<BlockSelectionBox>, Without<Player>)>,
    selected: Res<SelectedBlock>,
) {
    let Ok(mut selection_transform) = q_selection.get_single_mut() else { return };
    selection_transform.translation = (selected.position + Vec2::splat(BLOCK_SIZE_PX/2.)).extend(2.0);
}

fn break_blocks(
    mouse_button: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    selected: Res<SelectedBlock>,
    mut ev_break_block: EventWriter<SetBlock>,
) {
    if mouse_button.pressed(MouseButton::Left) {

        let layer;
        if keyboard.pressed(KeyCode::ShiftLeft) {
            layer = BlockLayer::Background;
        }
        else {
            layer = BlockLayer::Foreground;
        }

        ev_break_block.send(SetBlock {
            block: Block::AIR,
            position: selected.position,
            layer,
            can_overwrite: true,
        });
    }
}

fn place_blocks(
    mouse_button: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    selected: Res<SelectedBlock>,
    mut ev_set_block: EventWriter<SetBlock>,
    block_database: Res<BlockDatabase>,
    mut q_player: Query<&mut Inventory, With<Player>>,
    hotbar: Res<Hotbar>,
    world: Res<World>,
) {
    let Ok(mut inventory) = q_player.get_single_mut() else { return }; 

    if mouse_button.pressed(MouseButton::Right) {

        let Some(selected_item) = inventory.items[hotbar.selected_slot].item else { return };

        match selected_item.item_type {
            ItemType::Block(id) => {
                if inventory.has_item(selected_item, 1) {
                
                    let layer;
                    if keyboard.pressed(KeyCode::ShiftLeft) {
                        layer = BlockLayer::Background;
                    }
                    else {
                        layer = BlockLayer::Foreground;
                    }

                    let Some(block) = world.get_block(selected.position.x, selected.position.y, layer) else { return; };
                    if block.id != 0 { return; };

                    inventory.remove_item_from_slot(hotbar.selected_slot);

                    ev_set_block.send(SetBlock {
                        block: block_database.get_by_id(id),
                        position: selected.position,
                        layer,
                        can_overwrite: false,
                    });
                }
            },
            _ => return
        }
    }
}