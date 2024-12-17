use std::{fs::File, io::Read};

use bevy::prelude::*;
use serde_json::Value;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_item_database);
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ItemType {
    Block(u32),
    Tool,
    Miscellaneous,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Item {
    pub id: u32,
    pub item_type: ItemType,
}

impl Item {
    // todo: сделать так, чтобы при разных id
    // блока и предмета можно было сопоставить,
    // какой предмет к какому блоку относится
    pub fn from_block_id(id: u32) -> Self {
        Self {
            id,
            item_type: ItemType::Block(id),
        }
    }
}

#[derive(Resource)]
pub struct ItemDatabase {
    pub items: Vec<Value>,
}

impl ItemDatabase {
    pub fn get_by_id(&self, id: u32) -> Item {
        let item_data = self.items[id as usize].as_object().unwrap();
        let item_type_map = item_data.get("item_type").unwrap().as_object().unwrap();
        let item_type;
        
        // КОД ГОВА todo: разобраться с десериализацией enum'ов
        if  item_type_map.get("Tool").is_some() {
            item_type = ItemType::Tool;
        }
        else if let Some(val) = item_type_map.get("Block") {
            item_type = ItemType::Block(val.as_object().unwrap().get("id").unwrap().as_u64().unwrap() as u32);
        }
        else {
            item_type = ItemType::Miscellaneous;
        }

        Item {
            id,
            item_type
        }
    }

    pub fn get_texture_by_id(&self, id: u32) -> String {
        let item_data = self.items[id as usize].as_object().unwrap();

        item_data.get("texture").unwrap().as_str().unwrap().to_string()
    }
}

fn init_item_database(
    mut commands: Commands,
) {
    let mut file = File::open("assets/item_data.json").unwrap();

    let mut string = String::new();
    let _ = file.read_to_string(&mut string);

    let map = serde_json::from_str::<serde_json::Map<String, Value>>(&string).unwrap();
    let items: Vec<Value> = map.get("items").unwrap().as_array().unwrap().to_vec();

    commands.insert_resource(ItemDatabase{items});
}