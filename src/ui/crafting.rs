use bevy::prelude::*;

use crate::inventory::{
    crafting::{CraftItem, CraftingRecipeDatabase},
    item::ItemDatabase
};

use super::mode_manager::UiState;

pub struct CraftingPlugin;

impl Plugin for CraftingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(UiState::Management), spawn_crafting_menu) 
            .add_systems(Update, craft_items);
    }
}

#[derive(Component)]
struct CraftingSlot(u32);

fn spawn_crafting_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    recipe_database: Res<CraftingRecipeDatabase>,
    item_database: Res<ItemDatabase>,
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
        justify_self: JustifySelf::End,
        ..default()
    }).id();

    let mut slots = vec![];

    for i in 0..recipe_database.recipes.len() {
        let recipe = &recipe_database.recipes[i];

        let slot = commands.spawn((
            ImageNode::solid_color(Color::BLACK.with_alpha(0.5)),
            Node {
                justify_content: JustifyContent::Center,
                align_content: AlignContent::Center,
                ..default()
            },
            CraftingSlot(recipe.output),
            Button
        )).id();
        let slot_item = commands.spawn((
            ImageNode::new(asset_server.load(item_database.get_texture_by_id(recipe.output))),
            Node {
                width: Val::Percent(80.),
                height: Val::Percent(80.),
                align_self: AlignSelf::Center,
                position_type: PositionType::Absolute,
                ..default()
            },
        )).id();

        commands.entity(slot).add_child(slot_item);
        slots.push(slot);
    }

    commands.entity(canvas)
    .add_children(&slots)
    .insert(StateScoped(UiState::Management));
}

fn craft_items(
    mut ev_craft: EventWriter<CraftItem>,
    q_button: Query<(&Interaction, &CraftingSlot), Changed<Interaction>>,
) {
    for (interaction, slot) in q_button.iter() {
        if *interaction == Interaction::Pressed {
            ev_craft.send(CraftItem(slot.0));
        }
    } 
}