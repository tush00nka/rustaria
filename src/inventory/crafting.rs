use std::{fs::File, io::Read};

use bevy::prelude::*;
use serde_json::{Map, Value};

pub struct CraftingPlugin;

impl Plugin for CraftingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_database);
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