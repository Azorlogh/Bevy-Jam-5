use avian3d::prelude::*;
use bevy::prelude::*;
use bevy::render::{
    mesh::Indices, render_asset::RenderAssetUsages, render_resource::PrimitiveTopology,
};

use noise::{NoiseFn, Perlin, Turbulence};

// Terrain Constants
const MAP_CHUNK_SIZE: usize = 240;
const SIZE: Vec2 = Vec2::new(MAP_CHUNK_SIZE as f32, MAP_CHUNK_SIZE as f32);
const AMPLITUDE: f64 = 10.0;
const LOD: usize = 0;
const SCALE: f32 = 0.005;

pub struct TerrainPlugin;
impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<TerrainParams>()
            .add_systems(Startup, create_terrain)
            .add_systems(Update, build_terrain);
    }
}

#[derive(Component, Reflect)]
pub struct TerrainParams {
    size: Vec2,
    amplitude: f64,
    lod: usize,
    frequency: f64,
    power: f64,
    roughness: usize,
    scale: f32,
}

fn build_terrain(
    mut cmds: Commands,
    // asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    q_terrain_params: Query<(Entity, &TerrainParams), Changed<TerrainParams>>,
) {
    for (entity, tp) in q_terrain_params.iter() {
        let perlin = Perlin::new(548);

        let noise: Turbulence<Perlin, Perlin> = Turbulence::new(perlin)
            .set_frequency(tp.frequency)
            .set_power(tp.power)
            .set_roughness(tp.roughness);

        let mesh = create_cube_mesh(tp, noise);
        // Render the mesh with the custom texture using a PbrBundle, add the marker.
        cmds.entity(entity).insert((
            Name::new("Terrain"),
            Collider::trimesh_from_mesh(&mesh).unwrap(),
            CollisionMargin(0.1),
            RigidBody::Static,
            PbrBundle {
                mesh: meshes.add(mesh),
                material: materials.add(StandardMaterial::default()),
                ..default()
            },
        ));
    }
}

fn create_terrain(mut cmds: Commands) {
    cmds.spawn(TerrainParams {
        size: SIZE,
        amplitude: AMPLITUDE,
        lod: LOD,
        frequency: 0.2,
        power: 10.0,
        roughness: 4,
        scale: SCALE,
    });
}

fn create_cube_mesh(tp: &TerrainParams, noise: impl NoiseFn<f64, 2>) -> Mesh {
    // Keep the mesh data accessible in future frames to be able to mutate it in toggle_texture.
    let (vertex_grid, vertex_indices) =
        create_vertex_grid(tp.size, tp.amplitude, tp.lod, noise, tp.scale);
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertex_grid)
    .with_inserted_indices(vertex_indices)
    .with_computed_normals()
}

fn create_vertex_grid(
    size: Vec2,
    amplitude: f64,
    lod: usize,
    noise: impl NoiseFn<f64, 2>,
    scale: f32,
) -> (Vec<Vec3>, Indices) {
    let xlen = size.x as u32;
    let ylen = size.y as u32;

    let mesh_simplification_increment = match lod {
        0 => 1,
        _ => lod,
    };
    // let mesh_simplification_increment = 1;

    let vertices_per_line = ((size.x - 1.0) / mesh_simplification_increment as f32 + 1.0) as u32;
    // let vertices_per_line = size.x as u32;

    let mut grid = vec![];
    let mut indices = vec![];
    let mut vidx = 0;

    dbg!(vertices_per_line);
    for iy in (0..=ylen).step_by(mesh_simplification_increment) {
        for ix in (0..=xlen).step_by(mesh_simplification_increment) {
            // create vertices
            let x = (ix as f32) - (size.x / 2.0);
            let y = (iy as f32) - (size.y / 2.0);
            let z = noise.get([(x * scale) as f64, (y * scale) as f64]) as f32;
            grid.push(Vec3::new(x, z * amplitude as f32, y));

            // create indices
            if ix < xlen && iy < ylen {
                dbg!(vidx);
                indices.push(vidx);
                indices.push(vidx + vertices_per_line + 1);
                indices.push(vidx + 1);
                indices.push(vidx + vertices_per_line + 1);
                indices.push(vidx + vertices_per_line + 2);
                indices.push(vidx + 1);
            }
            vidx += 1;
        }
    }

    (grid, Indices::U32(indices))
}
