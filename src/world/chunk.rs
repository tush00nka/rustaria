use std::hash::{Hash, Hasher};

use bevy_rapier2d::prelude::*;
use bevy::{asset::RenderAssetUsages, prelude::*, render::mesh::{Indices, PrimitiveTopology}};
use noise::{NoiseFn, Perlin, Simplex};

pub mod block;
use block::*;

use crate::CHUNK_SIZE;
use crate::BLOCK_SIZE_PX;

pub struct ChunkPlugin;

impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DrawChunk>();

        app.add_systems(Update, draw_chunk);
    }
}

#[derive(Clone, Copy)]
pub struct Chunk {
    pub position: (i32, i32),
    pub data: [[Block<'static>; CHUNK_SIZE]; CHUNK_SIZE],
}

impl Chunk {
    pub fn new(_x: i32, _y: i32) -> Self {
        let mut data = [[Block::new(0); CHUNK_SIZE]; CHUNK_SIZE];

        let mut hasher = std::hash::DefaultHasher::new();

        "FUCK MY LIFE".hash(&mut hasher);
        let seed = hasher.finish() as u32;
        let simplex = Simplex::new(seed);
        let perlin = Perlin::new(seed);

        for y in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {

                let xf = x as f64 + _x as f64 * CHUNK_SIZE as f64;
                let yf = y as f64 + _y as f64 * CHUNK_SIZE as f64;

                let spread = 0.05;
                let oct1 = perlin.get([xf * spread]);
                let oct2 = perlin.get([xf * spread * 0.25]);
                let oct3 = perlin.get([xf * spread * 2.]);

                let height = (oct1+oct2+oct3 + CHUNK_SIZE as f64 / 2.).floor();

                let oct1 = simplex.get([xf * spread, yf * spread]);
                let oct2 = simplex.get([xf * spread * 0.25, yf * spread * 0.25]);
                let oct3 = simplex.get([xf * spread * 1.5, yf * spread * 1.5]);
                let oct4 = simplex.get([xf * spread * 2.5, yf * spread * 2.5]);

                let density = oct1+oct2+oct3+oct4;

                if yf < height {

                    let density_check;

                    if yf > height/3. {
                        density_check = -0.9;
                    }
                    else {
                        density_check = -0.1;
                    }

                    if density > density_check {
                        data[x][y] = Block::new(1); // dirt

                        if density > 0.6 && yf < height / 3. {
                            data[x][y] = Block::new(3); // stone
                        }
                    }
                }

                // grass
                if yf == height
                && density > -0.8 {
                    data[x][y] = Block::new(2);
                }
            }
        }

        Self {
            position: (_x,_y),
            data
        }
    }

    pub fn create_mesh(&self) -> (Mesh, Collider) {

        let mut vertices: Vec<[f32; 3]> = vec![];
        let mut collider_vertices: Vec<Vec2> = vec![];

        let mut indices: Vec<u32> = vec![];
        let mut collider_indices: Vec<[u32; 3]> = vec![];
        
        let mut colors: Vec<[f32; 4]> = vec![];

        for y in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                if self.data[x][y].id != 0 {
                    vertices.extend([
                        [x as f32 * BLOCK_SIZE_PX + BLOCK_SIZE_PX, y as f32 * BLOCK_SIZE_PX + BLOCK_SIZE_PX, 0.0],
                        [x as f32 * BLOCK_SIZE_PX, y as f32 * BLOCK_SIZE_PX + BLOCK_SIZE_PX, 0.0],
                        [x as f32 * BLOCK_SIZE_PX, y as f32 * BLOCK_SIZE_PX, 0.0],
                        [x as f32 * BLOCK_SIZE_PX + BLOCK_SIZE_PX, y as f32 * BLOCK_SIZE_PX, 0.0],
                    ]);
                        
                    collider_vertices.extend([
                        Vec2::new(x as f32 * BLOCK_SIZE_PX + BLOCK_SIZE_PX, y as f32 * BLOCK_SIZE_PX + BLOCK_SIZE_PX),
                        Vec2::new(x as f32 * BLOCK_SIZE_PX, y as f32 * BLOCK_SIZE_PX + BLOCK_SIZE_PX),
                        Vec2::new(x as f32 * BLOCK_SIZE_PX, y as f32 * BLOCK_SIZE_PX),
                        Vec2::new(x as f32 * BLOCK_SIZE_PX + BLOCK_SIZE_PX, y as f32 * BLOCK_SIZE_PX),
                    ]);

                    let color = match self.data[x][y].id {
                        1 => [0.8, 0.2, 0.2, 1.0],
                        2 => [0.2, 0.8, 0.2, 1.0],
                        3 => [0.4, 0.4, 0.5, 1.0],
                        _ => [1.0; 4]
                    };

                    colors.extend([color; 4]);

                    let base_index = vertices.len() as u32 - 4;
                    indices.extend([base_index, base_index + 1, base_index + 2]);
                    collider_indices.push([base_index, base_index + 1, base_index + 2]);

                    indices.extend([base_index, base_index + 2, base_index + 3]);
                    collider_indices.push([base_index, base_index + 2, base_index + 3]);

                }
            }
        }

        (Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD)
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
            .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colors)
            .with_inserted_indices(Indices::U32(indices)), Collider::trimesh(collider_vertices, collider_indices))
    }
}

#[derive(Event)]
pub struct DrawChunk {
    pub chunk: Chunk,
}

fn draw_chunk(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut ev_draw_chunk: EventReader<DrawChunk>,
    mut world: ResMut<super::World>,
) {
    for ev in ev_draw_chunk.read() {
        let (mesh, collider) = ev.chunk.create_mesh();

        let chunk_entity = commands.spawn((
            Mesh2d(meshes.add(mesh)),
            MeshMaterial2d(materials.add(ColorMaterial::default())),
            Transform::default().with_translation(Vec3::new(
                ev.chunk.position.0 as f32 * CHUNK_SIZE as f32 * BLOCK_SIZE_PX,
                ev.chunk.position.1 as f32 * CHUNK_SIZE as f32 * BLOCK_SIZE_PX, 0.0
            )),
            collider,
            Friction::coefficient(0.0)
        )).id();

        if world.chunk_entites.contains_key(&ev.chunk.position) {
            commands.entity(*world.chunk_entites.get(&ev.chunk.position).unwrap()).despawn_recursive();
            world.chunk_entites.remove(&ev.chunk.position);
        }
        world.chunk_entites.insert(ev.chunk.position, chunk_entity);
    }
}