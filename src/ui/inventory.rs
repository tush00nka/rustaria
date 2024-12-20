use bevy::prelude::*;

use crate::{
    inventory::{
        item::{Item, ItemDatabase}, Inventory
    },
    player::Player
};

use super::mode_manager::UiState;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentDragItem>();

        app
            .add_systems(OnEnter(UiState::Management), spawn_player_inventory)
            .add_systems(Update, (
                update_inventory_of::<Player>,
                move_items_of::<Player>
            ).run_if(in_state(UiState::Management)))
            .add_systems(OnExit(UiState::Management), return_taken_item::<Player>);
    }
}

#[derive(Component)]
pub struct InventorySlot(pub usize);

#[derive(Resource, Default)]
pub struct CurrentDragItem {
    pub item: Option<Item>,
    pub amount: u32,
    pub slot_id: usize,
}

impl CurrentDragItem {
    fn clear(&mut self) {
        self.item = None;
        self.amount = 0;
        self.slot_id = 0;
    }
}

fn spawn_player_inventory(
    mut commands: Commands,
) {
    let canvas = commands.spawn(Node {
        display: Display::Grid,
        grid_auto_flow: GridAutoFlow::Row,
        grid_template_rows: RepeatedGridTrack::flex(5, 1.0),
        grid_template_columns: RepeatedGridTrack::flex(9, 1.0),
        width: Val::Percent(30.),
        height: Val::Percent(30.),
        column_gap: Val::Px(4.),
        row_gap: Val::Px(4.),
        justify_content: JustifyContent::Start,
        ..default()
    }).id();

    let mut item_slots = vec![];

    for i in 0..45 {
        let slot = commands.spawn((
            ImageNode::solid_color(Color::BLACK.with_alpha(0.5)),
            Node {
                justify_content: JustifyContent::Center,
                align_content: AlignContent::Center,
                ..default()
            },
            InventorySlot(i),
            Button
        )).id();
        let slot_item = commands.spawn((
            ImageNode::solid_color(Color::WHITE),
            Node {
                width: Val::Percent(80.),
                height: Val::Percent(80.),
                align_self: AlignSelf::Center,
                ..default()
            },
        )).id();
        let slot_amount = commands.spawn((
            Text::new("  "),
            TextColor::WHITE,
            TextFont {
                font_size: 14.,
                ..default()
            },
            TextLayout::new_with_no_wrap(),
        )).id();

        commands.entity(slot_item).add_child(slot_amount);
        commands.entity(slot).add_child(slot_item);
        item_slots.push(slot);
    }

    commands.entity(canvas)
    .add_children(&item_slots)
    .insert(StateScoped(UiState::Management));
}

pub fn update_inventory_of<S: Component>(
    q_player: Query<&Inventory, With<S>>, // todo: add some <Changed> implementation
    mut q_slot: Query<(&Children, &InventorySlot)>,
    mut q_slot_images: Query<(&Children, &mut ImageNode)>,
    mut q_slot_texts: Query<&mut Text>,
    item_database: Res<ItemDatabase>,
    asset_server: Res<AssetServer>,
) {
    let Ok(inventory) = q_player.get_single() else { return };

    for (children, slot_id) in q_slot.iter_mut() {
        let image_entity = *children.get(0).unwrap();
        let (image_children, mut slot_image) = q_slot_images.get_mut(image_entity).unwrap();

        let text_entity = *image_children.get(0).unwrap();
        let mut slot_text= q_slot_texts.get_mut(text_entity).unwrap();

        let id = slot_id.0;

        if inventory.items[id].item.is_some() {
            let item_image = asset_server.load(item_database.get_texture_by_id(inventory.items[id].item.unwrap().id));
            update_slot(id, inventory, &mut slot_image, &mut slot_text, item_image);
        }
        else {
            reset_slot(&mut slot_image, &mut slot_text);
        }
    }
}

pub fn update_slot(id: usize, inventory: &Inventory, slot_image: &mut ImageNode, slot_text: &mut Text, item_image: Handle<Image>) {
    let slot = inventory.items[id];
    slot_image.color = Color::WHITE.with_alpha(1.0);
    slot_image.image = item_image;
    if slot.amount > 1 {
        slot_text.0 = slot.amount.to_string();
    }
    else {
        slot_text.0 = "  ".to_string();
    }
}

pub fn reset_slot(slot_image: &mut ImageNode, slot_text: &mut Text) {
    slot_image.color = Color::WHITE.with_alpha(0.0);
    slot_text.0 = "".to_string();
}

fn move_items_of<S: Component>(
    mut current_drag_item: ResMut<CurrentDragItem>,
    q_slots: Query<(&Interaction, &InventorySlot), Changed<Interaction>>,
    mut q_inventory: Query<&mut Inventory, With<S>>,
) {
    let Ok(mut inventory) = q_inventory.get_single_mut() else { return };

    for (interaction, slot) in q_slots.iter() {
        if *interaction == Interaction::Pressed {
            if current_drag_item.item.is_none() {
                current_drag_item.item = inventory.items[slot.0].item;
                current_drag_item.amount = inventory.items[slot.0].amount;
                current_drag_item.slot_id = slot.0;

                inventory.items[slot.0].clear();
            }
            else {
                if inventory.items[slot.0].item.is_none() {
                    inventory.items[slot.0].item = current_drag_item.item;
                    inventory.items[slot.0].amount = current_drag_item.amount;

                    current_drag_item.clear();
                }
                else {
                    inventory.items[current_drag_item.slot_id] = inventory.items[slot.0];
                    inventory.items[slot.0].item = current_drag_item.item;
                    inventory.items[slot.0].amount = current_drag_item.amount;
                    current_drag_item.clear();
                }
            }
        }
    }
}

fn return_taken_item<S: Component>(
    mut current_drag_item: ResMut<CurrentDragItem>,
    mut q_inventory: Query<&mut Inventory, With<S>>,
) {
    let Ok(mut inventory) = q_inventory.get_single_mut() else { return };

    if current_drag_item.item.is_some() {
        inventory.items[current_drag_item.slot_id].item = current_drag_item.item;
        inventory.items[current_drag_item.slot_id].amount = current_drag_item.amount;
        current_drag_item.clear();
    }
}