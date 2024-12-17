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

#[derive(Clone, Copy)]
pub struct Block {
    pub id: u32,
    pub layer: BlockLayer,
    is_solid: bool,
    durability: u8,
}

impl Block {
    pub const AIR: Block = Block {
        id: 0,
        layer: BlockLayer::Foreground,
        is_solid: false,
        durability: 0
    };

    pub fn with_layer(&mut self, layer: BlockLayer) -> Self {
        self.layer = layer;
        *self
    }
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

        Block {
            id,
            layer: BlockLayer::Foreground,
            is_solid,
            durability,
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