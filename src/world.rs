use std::collections::HashMap;

use bevy::prelude::*;

mod chunk;
use block::Block;
use chunk::*;

use crate::{BLOCK_SIZE_PX, CHUNK_SIZE};

pub struct WorldPlugin;
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ChunkPlugin);

        app.init_resource::<World>();
        app.add_event::<BreakBlockAtPosition>();

        app
            .add_systems(Startup, (generate_world_data, draw_world.after(generate_world_data)))
            .add_systems(Update, break_block_at_position);
    }
}

#[derive(Resource, Default)]
pub struct World {
    chunks: HashMap<(i32, i32), Chunk>,
    chunk_entites: HashMap<(i32, i32), Entity>,
}

impl World {
    pub fn generate_chunk_at_position(&mut self, x: i32, y: i32) {
        self.chunks.insert((x,y), Chunk::new(x,y));
    }

    pub fn get_chunk(&self, x: i32, y: i32) -> Option<&Chunk> {
        if self.chunks.contains_key(&(x,y)) {
            return Some(self.chunks.get(&(x,y)).unwrap());
        }

        None
    }

    pub fn get_chunk_mut(&mut self, x: i32, y: i32) -> Option<&mut Chunk> {
        if self.chunks.contains_key(&(x,y)) {
            return Some(self.chunks.get_mut(&(x,y)).unwrap());
        }

        None
    }
}

fn generate_world_data(mut world: ResMut<World>) {
    for y in -8..1 {
        for x in -8..8 {
            world.generate_chunk_at_position(x, y);
        }
    }
}

fn draw_world(
    world: Res<World>,
    mut ev_draw_chunk: EventWriter<DrawChunk>,
) {
    for (_, chunk) in world.chunks.iter() {
        ev_draw_chunk.send(DrawChunk { chunk: *chunk });
    }
}

#[derive(Event)]
pub struct BreakBlockAtPosition(pub Vec2);

fn break_block_at_position(
    mut ev_break_block: EventReader<BreakBlockAtPosition>,
    mut world: ResMut<World>,
    mut ev_draw_chunk: EventWriter<DrawChunk>,
) {
    for ev in ev_break_block.read() {
        let (chunk_x, chunk_y) = ((ev.0.x / CHUNK_SIZE as f32 / BLOCK_SIZE_PX).floor() as i32,
                                            (ev.0.y / CHUNK_SIZE as f32 / BLOCK_SIZE_PX).floor() as i32);

        println!("chunk: ({chunk_x},{chunk_y})");

        let Some(chunk) = world.get_chunk_mut(chunk_x, chunk_y) else { return; };

        let (block_x, block_y) = ((ev.0.x / BLOCK_SIZE_PX - (chunk_x as f32 * CHUNK_SIZE as f32)) as usize,
                                                (ev.0.y / BLOCK_SIZE_PX - (chunk_y as f32 * CHUNK_SIZE as f32)) as usize);

        println!("block: ({block_x},{block_y})");

        chunk.data[block_x][block_y] = Block::new(0);

        ev_draw_chunk.send(DrawChunk { chunk: *chunk });
    }
}