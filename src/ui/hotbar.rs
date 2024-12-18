use bevy::prelude::*;

use crate::{
    inventory::{item::ItemDatabase, Inventory},
    player::hotbar::Hotbar
};

pub struct HotbarPlugin;

impl Plugin for HotbarPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_hotbar)
            .add_systems(Update, (update_hotbar, update_hotbar_selection));
    }
}

#[derive(Component)]
pub struct HotbarSlot(u32);

#[derive(Component)]
pub struct HotbarSlotImage(u32);

#[derive(Component)]
pub struct HotbarSlotText(u32);

fn spawn_hotbar(
    mut commands: Commands,
) {
    let canvas = commands.spawn(Node {
        display: Display::Flex,
        flex_direction: FlexDirection::Row,
        width: Val::Percent(100.),
        height: Val::Percent(100.),
        column_gap: Val::Px(4.),
        ..default()
    }).id();

    let mut item_slots = vec![];

    for i in 0..9 {
        let slot = commands.spawn((
            ImageNode::solid_color(Color::WHITE),
            Node {
                width: Val::Percent(50.) / 16.,
                height: Val::Percent(50.) / 9.,
                justify_content: JustifyContent::Center,
                ..default()
            },
            HotbarSlot(i),
        )).id();
        let slot_item = commands.spawn((
            ImageNode::solid_color(Color::WHITE),
            Node {
                width: Val::Percent(80.),
                height: Val::Percent(80.),
                align_self: AlignSelf::Center,
                ..default()
            },
            HotbarSlotImage(i),
        )).id();
        let slot_amount = commands.spawn((
            Text::new("0"),
            TextColor::WHITE,
            TextFont {
                font_size: 14.,
                ..default()
            },
            TextLayout::new_with_no_wrap(),
            HotbarSlotText(0),
        )).id();

        commands.entity(slot_item).add_child(slot_amount);
        commands.entity(slot).add_child(slot_item);
        item_slots.push(slot);
    }

    commands.entity(canvas).add_children(&item_slots);
}

fn update_hotbar_selection(
    mut q_slot: Query<(&mut ImageNode, &HotbarSlot)>,
    hotbar: Res<Hotbar>,
) {
    let mut slots: Vec<(Mut<'_, ImageNode>, _)> = q_slot
    .iter_mut()
    .sort_by::<&HotbarSlot>(|item1, item2| {
        item1.0.partial_cmp(&item2.0).unwrap()
    })
    .collect();

    for i in 0..9 {
        slots[i].0.color = Color::WHITE.with_alpha(0.5);
        if hotbar.selected_slot == i {
            slots[i].0.color = Color::srgb(0.0, 1.0, 0.0);
        }
    }

}

fn update_hotbar(
    q_player: Query<&Inventory, Changed<Inventory>>,
    mut q_slot_images: Query<(&mut ImageNode, &HotbarSlotImage)>,
    mut q_slot_texts: Query<(&mut Text, &HotbarSlotText)>,
    item_database: Res<ItemDatabase>,
    assets_server: Res<AssetServer>,
) {
    let Ok(inventory) = q_player.get_single() else { return };

    let mut slot_images: Vec<(Mut<'_, ImageNode>, _)> = q_slot_images
        .iter_mut()
        .sort_by::<&HotbarSlotImage>(|item1, item2| {
            item1.0.partial_cmp(&item2.0).unwrap()
        })
        .collect();

    let mut slot_texts: Vec<(Mut<'_, Text>, _)> = q_slot_texts
        .iter_mut()
        .sort_by::<&HotbarSlotText>(|item1, item2| {
            item1.0.partial_cmp(&item2.0).unwrap()
        })
        .collect();

    for i in 0..9 {
        if inventory.items[i].item.is_some() {
            let slot = inventory.items[i];
            slot_images[i].0.color = Color::WHITE.with_alpha(1.0);
            slot_images[i].0.image = assets_server.load(item_database.get_texture_by_id(slot.item.unwrap().id));
            if slot.amount > 1 {
                slot_texts[i].0.0 = slot.amount.to_string();
            }
            else {
                slot_texts[i].0.0 = "".to_string();
            }
        }
        else {
            slot_images[i].0.color = Color::WHITE.with_alpha(0.0);
            slot_texts[i].0.0 = "".to_string();
        }
    }
}