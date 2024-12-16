use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{mouse_position::MousePosition, world::BreakBlockAtPosition, BLOCK_SIZE_PX};

use super::Player;

pub struct BlockInteractionPlugin;

impl Plugin for BlockInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedBlockPosition>();
        
        app
            .add_systems(Startup, spawn_selection_box)
            .add_systems(Update, (update_selected_position, move_selection_box, break_blocks));
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
        MeshMaterial2d(materials.add(Color::WHITE.with_alpha(0.75))),
        Transform::from_scale(Vec3::splat(BLOCK_SIZE_PX + 4.)),
        BlockSelectionBox,
    ));
}

#[derive(Resource, Default)]
struct SelectedBlockPosition(Vec2);

fn update_selected_position(
    q_player: Query<&Transform, (With<Player>, Without<BlockSelectionBox>)>, 
    q_rapier_context: Query<&RapierContext>,
    mouse_position: Res<MousePosition>,
    mut selected_position: ResMut<SelectedBlockPosition>,
) {
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

    selected_position.0 = 
        ((hit.point - hit.normal * BLOCK_SIZE_PX/2.)
        / BLOCK_SIZE_PX).floor() * BLOCK_SIZE_PX;
}

fn move_selection_box(
    mut q_selection: Query<&mut Transform, (With<BlockSelectionBox>, Without<Player>)>,
    selected_position: Res<SelectedBlockPosition>,
) {
    let Ok(mut selection_transform) = q_selection.get_single_mut() else { return };
    selection_transform.translation = (selected_position.0 + Vec2::splat(BLOCK_SIZE_PX/2.)).extend(2.0);
}

fn break_blocks(
    mouse_button: Res<ButtonInput<MouseButton>>,
    selected_position: Res<SelectedBlockPosition>,
    mut ev_break_block: EventWriter<BreakBlockAtPosition>,
) {
    if mouse_button.just_pressed(MouseButton::Left) {
        ev_break_block.send(BreakBlockAtPosition(selected_position.0));
    }
}