use std::{fs::File, io::Read};

use bevy::prelude::*;
use serde::Deserialize;
use serde_json::Value;

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_block_database);
    }
}

#[derive(Clone, Copy)]
pub enum BlockLayer {
    Background,
    Foreground,
}

pub const MAX_LIGHT_LEVEL: u8 = 15;

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Block {
    pub id: u32,
    pub is_solid: bool,
    pub durability: u8,
    pub drop_item: u32,
    pub light_emission: u8,
    pub light: u8,
}

impl Block {
    pub const AIR: Block = Block {
        id: 0,
        is_solid: false,
        durability: 0,
        drop_item: 0,
        light_emission: 0,
        light: 0,
    };
}

#[derive(Resource, Deserialize)]
pub struct BlockDatabase {
    pub blocks: Vec<Value>,
}

impl BlockDatabase {
    pub fn get_by_id(&self, id: u32) -> Block {
        let block_data = self.blocks[id as usize].as_object().unwrap();

        let is_solid = block_data.get("is_solid").unwrap().as_bool().unwrap();
        let durability = block_data.get("durability").unwrap().as_u64().unwrap() as u8;
        let drop_item = block_data.get("drop_item").unwrap().as_u64().unwrap() as u32;
        let light_emission = block_data.get("light_emission").unwrap().as_u64().unwrap() as u8;

        Block {
            id,
            is_solid,
            durability,
            drop_item,
            light_emission, 
            light: 0,
        }
    }
}

fn init_block_database(
    mut commands: Commands,
) {
    let mut file = File::open("assets/block_data.json").unwrap();

    let mut string = String::new();
    let _ = file.read_to_string(&mut string);

    let map = serde_json::from_str::<serde_json::Map<String, Value>>(&string).unwrap();
    let blocks: Vec<Value> = map.get("blocks").unwrap().as_array().unwrap().to_vec();

    commands.insert_resource(BlockDatabase{blocks});
}