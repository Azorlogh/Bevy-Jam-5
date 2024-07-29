use std::f32::consts::TAU;

use avian3d::math::Scalar;
use avian3d::prelude::*;
use bevy::ecs::entity::EntityHashSet;
use bevy::pbr::ExtendedMaterial;
use bevy::prelude::*;
use bevy::render::{
    mesh::Indices, render_asset::RenderAssetUsages, render_resource::PrimitiveTopology,
};

use noise::{NoiseFn, Perlin, Turbulence};

mod loddy;

use loddy::{
    d2::{Chunk, Lod2dPlugin, Lod2dTree},
    ChunkReady, ChunkVisibility,
};

use crate::materials::sand::{SandMaterial, SandMaterialExtension};

// Makes the chunks slighly bigger so that they overlap and blend with neighboring chunks
// This helps blend between chunks of differing LODs
const SKIRT_RATIO: f32 = 1.2;

pub struct TerrainPlugin;
impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Lod2dPlugin)
            .init_resource::<TerrainParams>()
            .register_type::<TerrainParams>()
            .register_type::<ChunkVisibility>()
            .register_type::<ChunkReady>()
            .add_systems(Startup, setup)
            .add_systems(Update, build_terrain) // Change from Update to other
            .add_systems(Update, update_chunk_visibility)
            .add_systems(Update, update_lod_center.before(loddy::d2::update_lod));
    }
}

#[derive(Resource)]
pub struct TerrainMaterial(Handle<SandMaterial>);

fn setup(
    mut cmds: Commands,
    mut materials: ResMut<Assets<SandMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let material = materials.add(ExtendedMaterial {
        base: StandardMaterial {
            base_color: Color::srgb_u8(255, 208, 0),
            ..default()
        },
        extension: SandMaterialExtension::default(),
    });
    cmds.insert_resource(TerrainMaterial(material.clone()));
    cmds.spawn(MaterialMeshBundle {
        mesh: meshes.add(Rectangle::new(50000.0, 50000.0)),
        material,
        transform: Transform::from_xyz(0.0, -200.0, 0.0)
            .with_rotation(Quat::from_rotation_x(-TAU / 4.0)),
        ..default()
    });
}

fn update_chunk_visibility(mut q_chunk: Query<(&ChunkVisibility, &mut Visibility)>) {
    for (chunk_vis, mut vis) in q_chunk.iter_mut() {
        *vis = match chunk_vis {
            ChunkVisibility::Visible => Visibility::Visible,
            ChunkVisibility::Hidden => Visibility::Hidden,
        }
    }
}

fn update_lod_center(
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
    seed: u32,
    amplitude: f64,
    n_turb_frequency: f64,
    n_turb_power: f64,
    n_turb_roughness: usize,
    n_scale: f32,
    n_power: f32,
    n_skew: f32,
}

impl TerrainParams {
    pub fn get_height(&self, pos: Vec2) -> f32 {
        let perlin = Perlin::new(self.seed);

        let noise: Turbulence<Perlin, Perlin> = Turbulence::new(perlin)
            .set_frequency(self.n_turb_frequency)
            .set_power(self.n_turb_power)
            .set_roughness(self.n_turb_roughness);

        let n = noise.get((pos * self.n_scale).as_dvec2().to_array()) as f32;

        ((n + self.n_skew).powf(self.n_power) - self.n_skew) as f32 * self.amplitude as f32
    }
}

impl Default for TerrainParams {
    fn default() -> Self {
        TerrainParams {
            nb_vertices: 64,
            size: 512.0,
            seed: rand::random(),
            amplitude: 20.0,
            n_turb_frequency: 0.2,
            n_turb_power: 10.0,
            n_turb_roughness: 4,
            n_scale: 0.001,
            n_power: 2.0,
            n_skew: 1.0,
        }
    }
}

fn build_terrain(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    q_added_chunks: Query<Entity, Added<Chunk>>,
    q_chunks: Query<(Entity, &Chunk)>,
    tp: Res<TerrainParams>,
    material: Res<TerrainMaterial>,
) {
    let chunks_to_build: EntityHashSet = q_added_chunks
        .iter()
        .chain(
            tp.is_changed()
                .then_some(q_chunks.iter().map(|(e, _)| e))
                .into_iter()
                .flatten(),
        )
        .collect();

    for (entity, chunk) in chunks_to_build
        .into_iter()
        .filter_map(|e| q_chunks.get(e).ok())
    {
        let perlin = Perlin::new(tp.seed);

        let noise: Turbulence<Perlin, Perlin> = Turbulence::new(perlin)
            .set_frequency(tp.n_turb_frequency)
            .set_power(tp.n_turb_power)
            .set_roughness(tp.n_turb_roughness);

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
                Vec3::new(tp.size * SKIRT_RATIO, 1.0, tp.size * SKIRT_RATIO),
            ));
        };

        cmds.entity(entity).insert(MaterialMeshBundle {
            mesh: meshes.add(mesh),
            material: material.0.clone(),
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
    let scale = tp.n_scale;

    let lod = chunk.lod as usize;
    let offset = chunk.coord.as_vec2() * size;

    // let mesh_simplification_increment = match lod {
    //     0 => 1,
    //     _ => lod,
    // };
    let mesh_simplification_increment = 2usize.pow(lod as u32);
    // let mesh_simplification_increment = 1;

    let vertices_per_line =
        ((nb_vertices as f32 - 1.0) / mesh_simplification_increment as f32 + 1.0) as u32;
    // let vertices_per_line = size.x as u32;

    let mut grid = vec![];
    let mut indices = vec![];
    let mut vidx = 0;

    let mut heights = vec![];

    for iy in (0..=nb_vertices).step_by(mesh_simplification_increment) {
        let mut sub_height = vec![];
        for ix in (0..=nb_vertices).step_by(mesh_simplification_increment) {
            // create vertices
            let x = ((ix as f32) - (nb_vertices as f32 / 2.0)) / nb_vertices as f32
                * size
                * SKIRT_RATIO;
            let y = ((iy as f32) - (nb_vertices as f32 / 2.0)) / nb_vertices as f32
                * size
                * SKIRT_RATIO;

            let skirt = ((x.abs().max(y.abs()) / (size / 2.0) - 1.0).max(0.0)
                / (SKIRT_RATIO - 1.0))
                .powf(2.0);

            let n = noise.get([
                ((x + offset.x) * scale) as f64,
                ((y + offset.y) * scale) as f64,
            ]) as f32;

            // blend between the noise of the terrain, and the bottom of the skirt
            // let z = (n.abs().powf(tp.n_power as f64) * n.signum()) as f32 * amplitude as f32
            //     - 3.0 * skirt;
            let z = ((n + tp.n_skew).powf(tp.n_power) - tp.n_skew) as f32 * tp.amplitude as f32
                - 3.0 * skirt;

            sub_height.push(z);
            grid.push(Vec3::new(x, z, y));

            // create indices
            if ix < nb_vertices && iy < nb_vertices {
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
