use bevy::prelude::*;

use crate::{
    inventory::{
        item::ItemDatabase,
        Inventory
    },
    player::Player
};

use super::mode_manager::UiState;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(UiState::Management), spawn_player_inventory)
            .add_systems(Update, update_inventory_of::<Player, 45>
                .run_if(in_state(UiState::Management)));
    }
}

#[derive(Component)]
pub struct InventorySlot(pub u32);

#[derive(Component)]
pub struct InventorySlotImage(pub u32);

#[derive(Component)]
pub struct InventorySlotText(pub u32);

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
                ..default()
            },
            InventorySlot(i),
        )).id();
        let slot_item = commands.spawn((
            ImageNode::solid_color(Color::WHITE),
            Node {
                width: Val::Percent(80.),
                height: Val::Percent(80.),
                align_self: AlignSelf::Center,
                ..default()
            },
            InventorySlotImage(i),
        )).id();
        let slot_amount = commands.spawn((
            Text::new("0"),
            TextColor::WHITE,
            TextFont {
                font_size: 14.,
                ..default()
            },
            TextLayout {
                justify: JustifyText::Left,
                linebreak: LineBreak::NoWrap,
            },
            InventorySlotText(0),
        )).id();

        commands.entity(slot_item).add_child(slot_amount);
        commands.entity(slot).add_child(slot_item);
        item_slots.push(slot);
    }

    commands.entity(canvas)
    .add_children(&item_slots)
    .insert(StateScoped(UiState::Management));
}

fn update_inventory_of<S: Component, const LENGTH: usize>(
    q_player: Query<&Inventory, With<S>>, // todo: add some <Changed> implementation
    mut q_slot_images: Query<(&mut ImageNode, &InventorySlotImage)>,
    mut q_slot_texts: Query<(&mut Text, &InventorySlotText)>,
    item_database: Res<ItemDatabase>,
    asset_server: Res<AssetServer>,
) {
    let Ok(inventory) = q_player.get_single() else { return };

    let mut slot_images: Vec<(Mut<'_, ImageNode>, _)> = q_slot_images
        .iter_mut()
        .sort_by::<&InventorySlotImage>(|item1, item2| {
            item1.0.partial_cmp(&item2.0).unwrap()
        })
        .collect();

    let mut slot_texts: Vec<(Mut<'_, Text>, _)> = q_slot_texts
        .iter_mut()
        .sort_by::<&InventorySlotText>(|item1, item2| {
            item1.0.partial_cmp(&item2.0).unwrap()
        })
        .collect();

    for i in 0..LENGTH {
        if inventory.items[i].item.is_some() {
            let item_image = asset_server.load(item_database.get_texture_by_id(inventory.items[i].item.unwrap().id));
            update_slot(i, inventory, &mut slot_images[i].0, &mut slot_texts[i].0, item_image);
        }
        else {
            reset_slot(&mut slot_images[i].0, &mut slot_texts[i].0);
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
        slot_text.0 = "".to_string();
    }
}

pub fn reset_slot(slot_image: &mut ImageNode, slot_text: &mut Text) {
    slot_image.color = Color::WHITE.with_alpha(0.0);
    slot_text.0 = "".to_string();
}