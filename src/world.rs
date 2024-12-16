use std::collections::HashMap;

use bevy::prelude::*;

mod chunk;
use chunk::*;

pub struct WorldPlugin;
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ChunkPlugin);

        app.init_resource::<World>();

        app.add_systems(Startup, (generate_world_data, draw_world.after(generate_world_data)));
    }
}

#[derive(Resource, Default)]
pub struct World {
    chunks: HashMap<(i32, i32), Chunk>,
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
    world: ResMut<World>,
    mut ev_draw_chunk: EventWriter<DrawChunkEvent>,
) {
    for (_, chunk) in world.chunks.iter() {
        ev_draw_chunk.send(DrawChunkEvent { chunk: *chunk });
    }
}