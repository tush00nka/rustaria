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
        app.add_plugins(BlockPlugin);
        app.add_event::<DrawChunk>();
        app.add_systems(Update, draw_chunk);
    }
}

#[derive(Clone, Copy)]
pub struct ChunkBlockPool {
    pub grass: Block,
    pub dirt: Block,
    pub stone: Block,
}

#[derive(Clone, Copy)]
pub struct Chunk {
    pub position: (i32, i32),
    pub data: [[Block; CHUNK_SIZE]; CHUNK_SIZE],
    pub background_data: [[Block; CHUNK_SIZE]; CHUNK_SIZE],
}

impl Chunk {
    pub fn new(_x: i32, _y: i32) -> Self {
        Self {
            position: (_x,_y),
            data: [[Block::AIR; CHUNK_SIZE]; CHUNK_SIZE],
            background_data: [[Block::AIR; CHUNK_SIZE]; CHUNK_SIZE]
        }
    }

    pub fn fill_block_data(&mut self, block_pool: ChunkBlockPool) {
        let (_x, _y) = self.position;

        let mut hasher = std::hash::DefaultHasher::new();

        "I LOVE LIKA".hash(&mut hasher);
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
                let cave_density = oct1+oct2+oct3+oct4;

                let offset = 1000.0;
                let oct1 = simplex.get([(xf + offset) * spread, (yf + offset) * spread]);
                let oct2 = simplex.get([(xf + offset) * spread * 0.25, (yf + offset) * spread * 0.25]);
                let oct3 = simplex.get([(xf + offset) * spread * 1.5, (yf + offset) * spread * 1.5]);
                let block_density = oct1+oct2+oct3;

                if yf < height {

                    let density_check;

                    if yf > height/3. {
                        density_check = -0.9;
                    }
                    else {
                        density_check = -0.1;
                    }

                    if block_density > density_check {
                        self.background_data[x][y] = block_pool.dirt; // dirt
                    }
                    else {
                        self.background_data[x][y] = block_pool.stone; // stone
                    }                    

                    if cave_density > density_check {
                        if block_density > density_check {
                            self.data[x][y] = block_pool.dirt; // dirt
                            continue;
                        }
                        
                        self.data[x][y] = block_pool.stone; // stone
                    }
                }

                // grass
                if yf == height
                && cave_density > -0.8 {
                    self.data[x][y] = block_pool.grass;
                    self.background_data[x][y] = block_pool.dirt; // dirt
                }
            }
        }
    }

    pub fn create_mesh(&self) -> (Mesh, Mesh, Collider) {

        let mut vertices: Vec<[f32; 3]> = vec![];
        let mut bg_vertices: Vec<[f32; 3]> = vec![];
        let mut collider_vertices: Vec<Vec2> = vec![];

        let mut indices: Vec<u32> = vec![];
        let mut bg_indices: Vec<u32> = vec![];
        let mut collider_indices: Vec<[u32; 3]> = vec![];
        
        let mut colors: Vec<[f32; 4]> = vec![];
        let mut bg_colors: Vec<[f32; 4]> = vec![];

        let mut uvs: Vec<[f32; 2]> = vec![];
        let mut bg_uvs: Vec<[f32; 2]> = vec![];

        for y in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                let uv_block_size = BLOCK_SIZE_PX/256.;

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

                    let color = [1.0; 4];
                    colors.extend([color; 4]);

                    let uv_offset_x = (self.data[x][y].id as f32 * BLOCK_SIZE_PX) / 256.;
                    let uv_offset_y = ((self.data[x][y].id / 16) as f32 * 16. * BLOCK_SIZE_PX) / 256.;

                    uvs.extend([
                        [uv_offset_x, uv_offset_y],
                        [uv_offset_x + uv_block_size, uv_offset_y],
                        [uv_offset_x + uv_block_size, uv_offset_y + uv_block_size],
                        [uv_offset_x, uv_offset_y + uv_block_size],
                    ]);

                    let base_index = vertices.len() as u32 - 4;
                    indices.extend([base_index, base_index + 1, base_index + 2]);
                    collider_indices.push([base_index, base_index + 1, base_index + 2]);

                    indices.extend([base_index, base_index + 2, base_index + 3]);
                    collider_indices.push([base_index, base_index + 2, base_index + 3]);
                }
                else if self.background_data[x][y].id != 0 {
                    bg_vertices.extend([
                        [x as f32 * BLOCK_SIZE_PX + BLOCK_SIZE_PX, y as f32 * BLOCK_SIZE_PX + BLOCK_SIZE_PX, -1.0],
                        [x as f32 * BLOCK_SIZE_PX, y as f32 * BLOCK_SIZE_PX + BLOCK_SIZE_PX, -1.0],
                        [x as f32 * BLOCK_SIZE_PX, y as f32 * BLOCK_SIZE_PX, -1.0],
                        [x as f32 * BLOCK_SIZE_PX + BLOCK_SIZE_PX, y as f32 * BLOCK_SIZE_PX, -1.0],
                    ]);

                    let color = [0.3, 0.3, 0.3, 1.0];
                    bg_colors.extend([color; 4]);

                    let uv_offset_x = (self.background_data[x][y].id as f32 * BLOCK_SIZE_PX) / 256.;
                    let uv_offset_y = ((self.background_data[x][y].id / 16) as f32 * 16. * BLOCK_SIZE_PX) / 256.;

                    bg_uvs.extend([
                        [uv_offset_x, uv_offset_y],
                        [uv_offset_x + uv_block_size, uv_offset_y],
                        [uv_offset_x + uv_block_size, uv_offset_y + uv_block_size],
                        [uv_offset_x, uv_offset_y + uv_block_size],
                    ]);

                    let base_index = bg_vertices.len() as u32 - 4;
                    bg_indices.extend([base_index, base_index + 1, base_index + 2]);
                    bg_indices.extend([base_index, base_index + 2, base_index + 3]);
                }
            }
        }

        let mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD)
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
            .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colors)
            .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
            .with_inserted_indices(Indices::U32(indices));

        let bg_mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD)
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, bg_vertices)
            .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, bg_colors)
            .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, bg_uvs)
            .with_inserted_indices(Indices::U32(bg_indices));

        (mesh, bg_mesh, Collider::trimesh(collider_vertices, collider_indices))
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
    asset_server: Res<AssetServer>,
    mut ev_draw_chunk: EventReader<DrawChunk>,
    mut world: ResMut<super::World>,
) {
    for ev in ev_draw_chunk.read() {
        let (mesh, bg_mesh, collider) = ev.chunk.create_mesh();

        let chunk_entity = commands.spawn((
            Mesh2d(meshes.add(mesh)),
            MeshMaterial2d(materials.add(asset_server.load("blocks.png"))),
            Transform::default().with_translation(Vec3::new(
                ev.chunk.position.0 as f32 * CHUNK_SIZE as f32 * BLOCK_SIZE_PX,
                ev.chunk.position.1 as f32 * CHUNK_SIZE as f32 * BLOCK_SIZE_PX, 0.0
            )),
            collider,
            CollisionGroups::new(Group::GROUP_2, Group::GROUP_1 | Group::GROUP_3),
            Friction::coefficient(0.0),
            Restitution::coefficient(0.0),
        ))
        .with_child((
            Mesh2d(meshes.add(bg_mesh)),
            MeshMaterial2d(materials.add(asset_server.load("blocks.png"))),
        ))
        .id();

        if world.chunk_entites.contains_key(&ev.chunk.position) {
            commands.entity(*world.chunk_entites.get(&ev.chunk.position).unwrap()).despawn_recursive();
            world.chunk_entites.remove(&ev.chunk.position);
        }
        world.chunk_entites.insert(ev.chunk.position, chunk_entity);
    }
}