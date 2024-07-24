use avian3d::math::Scalar;
use avian3d::prelude::*;
use bevy::prelude::*;
use bevy::render::{
    mesh::Indices, render_asset::RenderAssetUsages, render_resource::PrimitiveTopology,
};

use noise::{NoiseFn, Perlin, Turbulence};

use loddy::d2::{Lod2dPlugin, Lod2dTree};

use crate::loddy::d2::Chunk;
use crate::loddy::{self, ChunkReady, ChunkVisibility};

pub struct TerrainPlugin;
impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Lod2dPlugin)
            .init_resource::<TerrainParams>()
            .register_type::<TerrainParams>()
            .register_type::<ChunkVisibility>()
            .register_type::<ChunkReady>()
            .add_systems(Update, build_terrain) // Change from Update to other
            .add_systems(Update, update_chunk_visibility)
            .add_systems(Update, update_cursor.before(loddy::d2::update_lod));
    }
}

fn update_chunk_visibility(mut q_chunk: Query<(&ChunkVisibility, &mut Visibility)>) {
    for (chunk_vis, mut vis) in q_chunk.iter_mut() {
        *vis = match chunk_vis {
            ChunkVisibility::Visible => Visibility::Visible,
            ChunkVisibility::Hidden => Visibility::Hidden,
        }
    }
}

fn update_cursor(
    q_cam: Query<&GlobalTransform, With<Camera>>,
    tp: Res<TerrainParams>,
    mut lod: ResMut<Lod2dTree>,
) {
    let Ok(pos) = q_cam.get_single() else { return };
    lod.pos = pos.translation().xz() / tp.size;
}

#[derive(Resource, Reflect, Clone)]
#[reflect(Resource)]
pub struct TerrainParams {
    nb_vertices: usize,
    size: f32,
    amplitude: f64,
    n_frequency: f64,
    n_power: f64,
    n_roughness: usize,
    n_scale: f32,
}

impl Default for TerrainParams {
    fn default() -> Self {
        TerrainParams {
            nb_vertices: 12,
            size: 256.0,
            amplitude: 10.0,
            n_frequency: 0.2,
            n_power: 10.0,
            n_roughness: 4,
            n_scale: 0.005,
        }
    }
}

fn build_terrain(
    mut cmds: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    q_chunk: Query<(Entity, &Chunk), Added<Chunk>>,
    tp: Res<TerrainParams>,
) {
    let seed = rand::random();
    for (entity, chunk) in q_chunk.iter() {
        let perlin = Perlin::new(seed);

        let noise: Turbulence<Perlin, Perlin> = Turbulence::new(perlin)
            .set_frequency(tp.n_frequency)
            .set_power(tp.n_power)
            .set_roughness(tp.n_roughness);

        let (mesh, heights) = create_cube_mesh(&tp, &chunk, noise);
        // Render the mesh with the custom texture using a PbrBundle, add the marker.
        cmds.entity(entity).insert((
            Name::new("Terrain"),
            CollisionMargin(0.1),
            RigidBody::Static,
            ChunkReady,
        ));

        if chunk.lod == 0 {
            cmds.entity(entity).insert(Collider::heightfield(
                heights,
                Vec3::new(tp.size, 1.0, tp.size),
            ));
        };

        cmds.entity(entity).insert(PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(StandardMaterial::default()),
            transform: Transform::from_translation(
                chunk.coord.as_vec2().extend(0.0).xzy() * tp.size as f32,
            ),
            ..default()
        });
    }
}

fn create_cube_mesh(
    tp: &TerrainParams,
    chunk: &Chunk,
    noise: impl NoiseFn<f64, 2>,
) -> (Mesh, Vec<Vec<Scalar>>) {
    // Keep the mesh data accessible in future frames to be able to mutate it in toggle_texture.
    let (vertex_grid, vertex_indices, heights) = create_vertex_grid(&tp, chunk, noise);
    let mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertex_grid)
    .with_inserted_indices(vertex_indices)
    .with_computed_normals();

    (mesh, heights)
}

fn create_vertex_grid(
    tp: &TerrainParams,
    chunk: &Chunk,
    noise: impl NoiseFn<f64, 2>,
) -> (Vec<Vec3>, Indices, Vec<Vec<Scalar>>) {
    let nb_vertices = tp.nb_vertices;
    let size = tp.size;
    let amplitude = tp.amplitude;
    let scale = tp.n_scale;

    let lod = chunk.lod as usize;
    let offset = chunk.coord.as_vec2() * size;

    let xlen = nb_vertices as u32;
    let ylen = nb_vertices as u32;

    let mesh_simplification_increment = match lod {
        0 => 1,
        _ => lod,
    };
    // let mesh_simplification_increment = 1;

    let vertices_per_line =
        ((nb_vertices as f32 - 1.0) / mesh_simplification_increment as f32 + 1.0) as u32;
    // let vertices_per_line = size.x as u32;

    let mut grid = vec![];
    let mut indices = vec![];
    let mut vidx = 0;

    let mut heights = vec![];

    for iy in (0..=ylen).step_by(mesh_simplification_increment) {
        let mut sub_height = vec![];
        for ix in (0..=xlen).step_by(mesh_simplification_increment) {
            // create vertices
            let x = ((ix as f32) - (nb_vertices as f32 / 2.0)) / nb_vertices as f32 * size;
            let y = ((iy as f32) - (nb_vertices as f32 / 2.0)) / nb_vertices as f32 * size;
            let z = noise.get([
                ((x + offset.x) * scale) as f64,
                ((y + offset.y) * scale) as f64,
            ]) as f32
                * amplitude as f32;

            sub_height.push(z);
            grid.push(Vec3::new(x, z, y));

            // create indices
            if ix < xlen && iy < ylen {
                indices.push(vidx);
                indices.push(vidx + vertices_per_line + 1);
                indices.push(vidx + 1);
                indices.push(vidx + vertices_per_line + 1);
                indices.push(vidx + vertices_per_line + 2);
                indices.push(vidx + 1);
            }
            vidx += 1;
        }
        heights.push(sub_height);
    }

    let rows = heights.len();
    let cols = heights[0].len();

    let transposed: Vec<Vec<_>> = (0..rows)
        .map(|col| (0..cols).map(|row| heights[row][col]).collect())
        .collect();

    (grid, Indices::U32(indices), transposed)
}
