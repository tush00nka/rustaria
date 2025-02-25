use std::{fs::File, io::Read};

use bevy::prelude::*;
use serde_json::Value;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_database);
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum ItemType {
    Block(u32),
    Tool,
    #[default]
    Miscellaneous,
}

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct Item {
    pub id: u32,
    pub item_type: ItemType,
    pub max_stack: u32,
}

#[derive(Resource)]
pub struct ItemDatabase {
    pub items: Vec<Value>,
}

impl ItemDatabase {
    pub fn get_by_id(&self, id: u32) -> Item {
        let item_data = self.items[id as usize].as_object().unwrap();
        let mut item_type = ItemType::Miscellaneous;
        
        let max_stack = item_data.get("max_stack").unwrap().as_u64().unwrap() as u32;

        if let Some(item_type_map) = item_data.get("item_type").unwrap().as_object() {
            if let Some(val) = item_type_map.get("Block") {
                item_type = ItemType::Block(val.as_object().unwrap().get("id").unwrap().as_u64().unwrap() as u32);
            }
        }
        else if let Some(item_type_str) = item_data.get("item_type").unwrap().as_str() {
            match item_type_str {
                "Tool" => { item_type = ItemType::Tool },
                "Miscellaneous" => { item_type = ItemType::Miscellaneous },
                _ => {}
            }
        }

        Item {
            id,
            item_type,
            max_stack,
        }
    }

    pub fn get_texture_by_id(&self, id: u32) -> String {
        let item_data = self.items[id as usize].as_object().unwrap();

        item_data.get("texture").unwrap().as_str().unwrap().to_string()
    }
}

fn init_database(
    mut commands: Commands,
) {
    let mut file = File::open("assets/item_data.json").unwrap();

    let mut string = String::new();
    let _ = file.read_to_string(&mut string);

    let map = serde_json::from_str::<serde_json::Map<String, Value>>(&string).unwrap();
    let items: Vec<Value> = map.get("items").unwrap().as_array().unwrap().to_vec();

    commands.insert_resource(ItemDatabase{items});
}