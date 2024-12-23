use std::hash::{Hash, Hasher};

use bevy_rapier2d::prelude::*;
use bevy::{asset::RenderAssetUsages, prelude::*, render::mesh::{Indices, PrimitiveTopology}};
use noise::{NoiseFn, Perlin, Simplex};
use rand::Rng;

pub mod block;
use block::*;

mod block_structure;
use block_structure::*;

use crate::{CHUNK_SIZE, SEED};
use crate::BLOCK_SIZE_PX;
pub struct ChunkPlugin;

impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BlockPlugin);
        app
            .add_event::<GenerateChunkData>()
            .add_event::<DrawChunk>()
            .add_event::<UpdateChunkLight>();
        app.add_systems(Update, (
            generate_chunk_data,
            update_light.after(generate_chunk_data),
            draw_chunk.after(update_light),
        ));
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Chunk {
    pub position: (i32, i32),
    pub data: [[Block; CHUNK_SIZE]; CHUNK_SIZE],
    pub background_data: [[Block; CHUNK_SIZE]; CHUNK_SIZE],
}

impl Chunk {
    pub const PLACEHOLDER: Chunk = Self {
        position: (i32::MAX, i32::MAX),
        data: [[Block::AIR; CHUNK_SIZE]; CHUNK_SIZE],
        background_data: [[Block::AIR; CHUNK_SIZE]; CHUNK_SIZE]
    };

    pub fn new(_x: i32, _y: i32) -> Self {
        Self {
            position: (_x,_y),
            data: [[Block::AIR; CHUNK_SIZE]; CHUNK_SIZE],
            background_data: [[Block::AIR; CHUNK_SIZE]; CHUNK_SIZE]
        }
    }

    pub fn create_mesh(&self) -> (Mesh, Mesh, Mesh, Collider) {

        let mut vertices: Vec<[f32; 3]> = vec![];
        let mut not_solid_vertices: Vec<[f32; 3]> = vec![];
        let mut bg_vertices: Vec<[f32; 3]> = vec![];
        let mut collider_vertices: Vec<Vec2> = vec![];

        let mut indices: Vec<u32> = vec![];
        let mut not_solid_indices: Vec<u32> = vec![];
        let mut bg_indices: Vec<u32> = vec![];
        let mut collider_indices: Vec<[u32; 3]> = vec![];
        
        let mut colors: Vec<[f32; 4]> = vec![];
        let mut not_solid_colors: Vec<[f32; 4]> = vec![];
        let mut bg_colors: Vec<[f32; 4]> = vec![];

        let mut uvs: Vec<[f32; 2]> = vec![];
        let mut not_solid_uvs: Vec<[f32; 2]> = vec![];
        let mut bg_uvs: Vec<[f32; 2]> = vec![];

        for y in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                let uv_block_size = BLOCK_SIZE_PX/256.;

                if self.data[x][y].is_solid {
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

                    let light = self.data[x][y].light as f32 / 15.;
                    let color = [light, light, light, 1.0];
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
                    indices.extend([
                        base_index, base_index + 1, base_index + 2,
                        base_index, base_index + 2, base_index + 3
                    ]);

                    collider_indices.extend([
                        [base_index, base_index + 1, base_index + 2],
                        [base_index, base_index + 2, base_index + 3]
                    ]);
                }
                else if self.data[x][y].id != 0 {
                    not_solid_vertices.extend([
                        [x as f32 * BLOCK_SIZE_PX + BLOCK_SIZE_PX, y as f32 * BLOCK_SIZE_PX + BLOCK_SIZE_PX, -0.5],
                        [x as f32 * BLOCK_SIZE_PX, y as f32 * BLOCK_SIZE_PX + BLOCK_SIZE_PX, -0.5],
                        [x as f32 * BLOCK_SIZE_PX, y as f32 * BLOCK_SIZE_PX, -0.5],
                        [x as f32 * BLOCK_SIZE_PX + BLOCK_SIZE_PX, y as f32 * BLOCK_SIZE_PX, -0.5],
                    ]);

                    let light = self.data[x][y].light as f32 / 15.;
                    let color = [light, light, light, 1.0];
                    not_solid_colors.extend([color; 4]);

                    let uv_offset_x = (self.data[x][y].id as f32 * BLOCK_SIZE_PX) / 256.;
                    let uv_offset_y = ((self.data[x][y].id / 16) as f32 * 16. * BLOCK_SIZE_PX) / 256.;

                    not_solid_uvs.extend([
                        [uv_offset_x, uv_offset_y],
                        [uv_offset_x + uv_block_size, uv_offset_y],
                        [uv_offset_x + uv_block_size, uv_offset_y + uv_block_size],
                        [uv_offset_x, uv_offset_y + uv_block_size],
                    ]);

                    let base_index = not_solid_vertices.len() as u32 - 4;
                    not_solid_indices.extend([
                        base_index, base_index + 1, base_index + 2,
                        base_index, base_index + 2, base_index + 3
                    ]);
                } 
                else if self.background_data[x][y].id != 0 {
                    bg_vertices.extend([
                        [x as f32 * BLOCK_SIZE_PX + BLOCK_SIZE_PX, y as f32 * BLOCK_SIZE_PX + BLOCK_SIZE_PX, -1.0],
                        [x as f32 * BLOCK_SIZE_PX, y as f32 * BLOCK_SIZE_PX + BLOCK_SIZE_PX, -1.0],
                        [x as f32 * BLOCK_SIZE_PX, y as f32 * BLOCK_SIZE_PX, -1.0],
                        [x as f32 * BLOCK_SIZE_PX + BLOCK_SIZE_PX, y as f32 * BLOCK_SIZE_PX, -1.0],
                    ]);

                    let light = self.background_data[x][y].light as f32 / 15.;
                    let color = [light/10., light/10., light/10., 1.0];
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
                    bg_indices.extend([
                        base_index, base_index + 1, base_index + 2,
                        base_index, base_index + 2, base_index + 3
                    ]);
                } 
            }
        }

        let mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD)
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
            .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colors)
            .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
            .with_inserted_indices(Indices::U32(indices));

        let not_solid_mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD)
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, not_solid_vertices)
            .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, not_solid_colors)
            .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, not_solid_uvs)
            .with_inserted_indices(Indices::U32(not_solid_indices));

        let bg_mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD)
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, bg_vertices)
            .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, bg_colors)
            .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, bg_uvs)
            .with_inserted_indices(Indices::U32(bg_indices));

        (mesh, not_solid_mesh, bg_mesh, Collider::trimesh(collider_vertices, collider_indices))
    }
}

#[derive(Event)]
pub struct GenerateChunkData {
    pub position: (i32, i32),
}

fn generate_chunk_data(
    mut ev_generate: EventReader<GenerateChunkData>,
    mut ev_update_light: EventWriter<UpdateChunkLight>,
    mut world: ResMut<super::World>,
    block_database: Res<BlockDatabase>,
) {
    for ev in ev_generate.read() {
        let (_x, _y) = ev.position;
        let mut chunk = Chunk::new(_x, _y);

        let mut hasher = std::hash::DefaultHasher::new();
        SEED.hash(&mut hasher);
        let seed = hasher.finish() as u32;
        let simplex = Simplex::new(seed);
        let perlin = Perlin::new(seed);

        let mut block_structures: Vec<((usize, usize), BlockStructure)> = vec![];

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
                        chunk.background_data[x][y] = block_database.get_by_id(1); // dirt
                    }
                    else {
                        chunk.background_data[x][y] = block_database.get_by_id(3); // stone
                    }                    

                    if cave_density > density_check {
                        if block_density > density_check {
                            chunk.data[x][y] = block_database.get_by_id(1); // dirt
                            continue;
                        }
                        
                        chunk.data[x][y] = block_database.get_by_id(3); // stone
                    }
                }

                // grass
                if yf == height
                && cave_density > -0.8{
                    chunk.data[x][y] = block_database.get_by_id(2);
                    chunk.background_data[x][y] = block_database.get_by_id(1); // dirt

                    let mut rng = rand::thread_rng();

                    if x & 2 == 0 {
                        chunk.data[x][y+1] = block_database.get_by_id(6); // thread
                    }

                    // trees
                    if x % 9 == 0 {
                        let structure = BlockStructure::new_tree(rng.gen_range(2..6));
                        block_structures.push(((x,y), structure));
                    }
                }
            }
        }

        for ((x, y), structure) in block_structures.iter() {
            for j in 0..structure.height() {
                for i in 0..structure.width() {
                    let block_id = structure.data[j][i];
                    let bg_block_id = structure.bg_data[j][i];
    
                    if block_id == 0 && !structure.fill_air { continue; };
                    // todo: make it not to fill bg aswell

                    if x+i < CHUNK_SIZE {
                        if y+j < CHUNK_SIZE {
                            chunk.data[x+i][y+j] = block_database.get_by_id(block_id);
                            chunk.background_data[x+i][y+j] = block_database.get_by_id(bg_block_id);
                        }
                        else {
                            if let Some(neighbour) = world.get_chunk_mut(_x, _y+1) {
                                neighbour.data[x+i][j] = block_database.get_by_id(block_id);
                                chunk.background_data[x+i][j] = block_database.get_by_id(bg_block_id);
                                ev_update_light.send(UpdateChunkLight { chunk: *neighbour });
                            }
                        }
                    }
                    else {
                        if y+j < CHUNK_SIZE {
                            if let Some(neighbour) = world.get_chunk_mut(_x+1, _y) {
                                neighbour.data[i][y+j] = block_database.get_by_id(block_id);
                                chunk.background_data[i][y+j] = block_database.get_by_id(bg_block_id);
                                ev_update_light.send(UpdateChunkLight { chunk: *neighbour });
                            }
                        }
                        else {
                            if let Some(neighbour) = world.get_chunk_mut(_x+1, _y+1) {
                                neighbour.data[i][j] = block_database.get_by_id(block_id);
                                chunk.background_data[i][j] = block_database.get_by_id(bg_block_id);
                                ev_update_light.send(UpdateChunkLight { chunk: *neighbour });
                            }
                        }
                    }
                }
            }
        }

        world.chunks.insert((_x, _y), chunk);
        ev_update_light.send(UpdateChunkLight { chunk });
    }
}

#[derive(Event)]
pub struct UpdateChunkLight {
    pub chunk: Chunk,
}

fn update_light(
    mut world: ResMut<super::World>,
    mut ev_update_light: EventReader<UpdateChunkLight>,
    mut ev_draw_chunk: EventWriter<DrawChunk>,
) {
    for ev in ev_update_light.read() {
        let mut chunk = ev.chunk;
        let (_x, _y) = chunk.position; 

        let mut block_light_queue = vec![];
        // let mut sun_light_queue = vec![];

        let default_chunk = Chunk::PLACEHOLDER;
        let top_chunk = world.get_chunk(_x, _y+1).unwrap_or(&default_chunk);
        let left_chunk = world.get_chunk(_x-1, _y).unwrap_or(&default_chunk);
        let right_chunk = world.get_chunk(_x+1, _y).unwrap_or(&default_chunk);
        let bottom_chunk = world.get_chunk(_x, _y-1).unwrap_or(&default_chunk);

        // block light
        for y in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {                
                chunk.data[x][y].light = 0;
                chunk.background_data[x][y].light = 0;

                if chunk.data[x][y].light_emission > 0 {
                    chunk.data[x][y].light = chunk.data[x][y].light_emission;
                    block_light_queue.push(((x,y), chunk.data[x][y].light_emission));
                }

                if top_chunk.data[x][0].light > chunk.data[x][CHUNK_SIZE-1].light {
                    block_light_queue.push(((x,CHUNK_SIZE-1), top_chunk.data[x][0].light - 1));
                }
                if bottom_chunk.data[x][CHUNK_SIZE-1].light > chunk.data[x][0].light {
                    block_light_queue.push(((x,0), bottom_chunk.data[x][CHUNK_SIZE-1].light - 1));
                }
            }

            if left_chunk.data[CHUNK_SIZE-1][y].light > chunk.data[0][y].light {
                block_light_queue.push(((0,y), left_chunk.data[CHUNK_SIZE-1][y].light - 1));
            }
            if right_chunk.data[0][y].light > chunk.data[CHUNK_SIZE-1][y].light {
                block_light_queue.push(((CHUNK_SIZE-1,y), right_chunk.data[0][y].light - 1));
            }
        }


        // sun light
        // if _y == WORLD_HEIGHT {
        //     for x in 0..CHUNK_SIZE {
        //         chunk.data[x][CHUNK_SIZE-1].light = 15;
        //         sun_light_queue.push(((x,CHUNK_SIZE-1), 15));
        //     }
        // }
        // else {
        //     for x in 0..CHUNK_SIZE {
        //         chunk.data[x][CHUNK_SIZE-1].light = top_chunk.data[x][0].light;  
        //         sun_light_queue.push(((x,CHUNK_SIZE-1), top_chunk.data[x][0].light));
        //     }
        // }

        // while !sun_light_queue.is_empty() {
        //     if let Some(((x, y), em)) = sun_light_queue.pop() {
        //         if em >= 3 {
        //             if y > 0 {
        //                 let emission = if chunk.data[x][y-1].is_solid {
        //                     em - 3
        //                 } else { em };

        //                 if chunk.data[x][y-1].light < emission {
        //                     chunk.data[x][y-1].light = emission;
        //                     chunk.background_data[x][y-1].light = emission;
        //                     sun_light_queue.push(((x,y-1), emission));
        //                 }
        //             }
        //         }
        //     }
        // }

        while !block_light_queue.is_empty() {
            if let Some(((x, y), emission)) = block_light_queue.pop() {
                if emission >= 1 {
                    if x+1 < CHUNK_SIZE {
                        if chunk.data[x+1][y].light < emission-1 {
                            chunk.data[x+1][y].light = emission-1;
                            chunk.background_data[x+1][y].light = emission-1;
                            block_light_queue.push(((x+1,y), emission-1));
                        }
                    }
    
                    if y+1 < CHUNK_SIZE {
                        if chunk.data[x][y+1].light < emission-1 {
                            chunk.data[x][y+1].light = emission-1;
                            chunk.background_data[x][y+1].light = emission-1;
                            block_light_queue.push(((x,y+1), emission-1));
                        }
                    }

                    if x > 0 {
                        if chunk.data[x-1][y].light < emission-1 {
                            chunk.data[x-1][y].light = emission-1;
                            chunk.background_data[x-1][y].light = emission-1;
                            block_light_queue.push(((x-1,y), emission-1));
                        }
                    }

                    if y > 0 {
                        if chunk.data[x][y-1].light < emission-1 {
                            chunk.data[x][y-1].light = emission-1;
                            chunk.background_data[x][y-1].light = emission-1;
                            block_light_queue.push(((x,y-1), emission-1));
                        }
                    }
                }
            }
        }
    
        let chunk_to_edit= world.get_chunk_mut(_x, _y).unwrap();
        chunk_to_edit.data = chunk.data;
        chunk_to_edit.background_data = chunk.background_data;

        ev_draw_chunk.send(DrawChunk { chunk });
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
        let (mesh, not_solid_mesh, bg_mesh, collider) = ev.chunk.create_mesh();

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
            Mesh2d(meshes.add(not_solid_mesh)),
            MeshMaterial2d(materials.add(asset_server.load("blocks.png"))), 
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