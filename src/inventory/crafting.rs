use std::{fs::File, io::Read};

use bevy::prelude::*;
use serde_json::{Map, Value};

use crate::player::Player;

use super::{item::ItemDatabase, Inventory};

pub struct CraftingPlugin;

impl Plugin for CraftingPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CraftItem>();
        app
            .add_systems(Startup, init_database)
            .add_systems(Update, (craft_item, debug));
    }
}

#[derive(Clone)]
pub struct CraftingRecipe {
    pub inputs: Vec<u32>,
    pub inputs_amount: Vec<u32>,
    pub output: u32,
    pub output_amount: u32
}

#[derive(Resource)]
pub struct CraftingRecipeDatabase {
    pub recipes: Vec<CraftingRecipe> 
} 

impl CraftingRecipeDatabase {
    pub fn get_by_output_id(&self, id: u32) -> Option<CraftingRecipe> {
        for recipe in self.recipes.iter() {
            if recipe.output == id {
                return Some(recipe.clone());
            }
        }

        None
    }
}

fn init_database(
    mut commands: Commands,
) {
    let mut file = File::open("assets/crafting_recipes_data.json").unwrap();

    let mut string = String::new();
    let _ = file.read_to_string(&mut string);

    let arr = serde_json::from_str::<Map<String, Value>>(&string).unwrap();
    let arr = arr.get("recipes").unwrap().as_array().unwrap();

    let mut recipes = vec![];
    for recipe in arr.iter() {
        let inputs: Vec<u32> = recipe.get("inputs").unwrap().as_array().unwrap().iter().map(|id| id.as_u64().unwrap() as u32).collect();
        let inputs_amount: Vec<u32> = recipe.get("inputs_amount").unwrap().as_array().unwrap().iter().map(|id| id.as_u64().unwrap() as u32).collect();
        let output = recipe.get("output").unwrap().as_u64().unwrap() as u32;
        let output_amount = recipe.get("output_amount").unwrap().as_u64().unwrap() as u32;

        recipes.push(CraftingRecipe {
            inputs,
            inputs_amount,
            output,
            output_amount,
        });
    }

    commands.insert_resource(CraftingRecipeDatabase { recipes });
}

#[derive(Event)]
pub struct CraftItem(pub u32);

fn debug(
    mut ev_craft: EventWriter<CraftItem>,
    keyboard: Res<ButtonInput<KeyCode>>
) {
    if keyboard.just_pressed(KeyCode::KeyT) {
        ev_craft.send(CraftItem(7));
    }
}

fn craft_item(
    mut ev_craft: EventReader<CraftItem>,
    recipe_database: Res<CraftingRecipeDatabase>,
    item_database: Res<ItemDatabase>,
    mut inventory: Single<&mut Inventory, With<Player>>,
) {
    for ev in ev_craft.read() {
        let Some(recipe) = recipe_database.get_by_output_id(ev.0) else { return };

        for i in 0..recipe.inputs.len() {
            if !inventory.has_item(item_database.get_by_id(recipe.inputs[i]), recipe.inputs_amount[i]) {
                return;
            }
        }

        for i in 0..recipe.inputs.len() {
            inventory.remove_item(item_database.get_by_id(recipe.inputs[i]), recipe.inputs_amount[i]);
        }

        // todo: rewrite this workaround so i can add amount of items by one call
        for _ in 0..recipe.output_amount {
            inventory.add_item(item_database.get_by_id(recipe.output));
        }
    }
}