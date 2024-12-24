use bevy::prelude::*;

use crate::player::{hotbar::Hotbar, Player};

use super::{
    inventory::{
        update_inventory_of, InventorySlot
    },
    mode_manager::UiState
};

pub struct HotbarPlugin;

impl Plugin for HotbarPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(UiState::InGame), spawn_hotbar)
            .add_systems(Update, (update_inventory_of::<Player>, update_hotbar_selection)
                .run_if(in_state(UiState::InGame)));
    }
}

fn spawn_hotbar(
    mut commands: Commands,
) {
    let canvas = commands.spawn(Node {
        display: Display::Grid,
        grid_auto_flow: GridAutoFlow::Row,
        grid_template_columns: RepeatedGridTrack::flex(9, 1.0),
        width: Val::Percent(30.),
        height: Val::Percent(6.),
        column_gap: Val::Px(4.),
        justify_content: JustifyContent::Start,
        ..default()
    }).id();

    let mut item_slots = vec![];

    for i in 0..9 {
        let slot = commands.spawn((
            ImageNode::solid_color(Color::BLACK),
            Node {
                justify_content: JustifyContent::Center,
                align_content: AlignContent::Center,
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
                position_type: PositionType::Absolute,
                ..default()
            },
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
        )).id();

        commands.entity(slot).add_children(&[slot_item, slot_amount]);
        item_slots.push(slot);
    }

    commands.entity(canvas)
    .add_children(&item_slots)
    .insert(StateScoped(UiState::InGame));
}

fn update_hotbar_selection(
    mut q_slot: Query<(&mut ImageNode, &InventorySlot)>,
    hotbar: Res<Hotbar>,
) {
    let mut slots: Vec<(Mut<'_, ImageNode>, _)> = q_slot
    .iter_mut()
    .sort_by::<&InventorySlot>(|item1, item2| {
        item1.0.partial_cmp(&item2.0).unwrap()
    })
    .collect();

    for i in 0..9 {
        slots[i].0.color = Color::BLACK.with_alpha(0.5);
        if hotbar.selected_slot == i {
            slots[i].0.color = Color::WHITE;
        }
    }
}
