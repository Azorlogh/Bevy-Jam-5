use avian3d::prelude::*;
use bevy::prelude::*;
use bevy::render::{
    mesh::Indices, render_asset::RenderAssetUsages, render_resource::PrimitiveTopology,
};

use noise::{NoiseFn, Perlin, Turbulence};
const SIZE: Vec2 = Vec2::new(500.0, 500.0);
const AMPLITUDE: f64 = 10.0;
const RES: f32 = 1.0;
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
    res: f32,
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
        let perlin = Perlin::new(rand::random());

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
        res: RES,
        frequency: 0.2,
        power: 10.0,
        roughness: 4,
        scale: SCALE,
    });
}

fn create_cube_mesh(tp: &TerrainParams, noise: impl NoiseFn<f64, 2>) -> Mesh {
    // Keep the mesh data accessible in future frames to be able to mutate it in toggle_texture.
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        create_vertex_grid(tp.size, tp.amplitude, tp.res, noise, tp.scale),
    )
    .with_inserted_indices(create_indices_grid(tp.size, tp.res))
    .with_computed_normals()
}

fn create_vertex_grid(
    size: Vec2,
    amplitude: f64,
    res: f32,
    noise: impl NoiseFn<f64, 2>,
    scale: f32,
) -> Vec<Vec3> {
    let xlen = (size.x * (1.0 / res)) as u32;
    let ylen = (size.y * (1.0 / res)) as u32;

    let mut grid = vec![];
    for iy in 0..=ylen as usize {
        for ix in 0..=xlen as usize {
            let x = (ix as f32 * res) - (size.x / 2.0);
            let y = (iy as f32 * res) - (size.y / 2.0);
            let z = noise.get([(x * scale) as f64, (y * scale) as f64]) as f32;
            grid.push(Vec3::new(x, z * amplitude as f32, y));
        }
    }
    return grid;
}

fn create_indices_grid(size: Vec2, res: f32) -> Indices {
    let xlen = (size.x * (1.0 / res)) as u32;
    let ylen = (size.y * (1.0 / res)) as u32;

    let mut indices = vec![];
    for x in 0..xlen {
        for y in 0..ylen {
            let idx = y * (xlen + 1) + x;
            indices.push(idx);
            indices.push(idx + xlen + 1);
            indices.push(idx + 1);
            indices.push(idx + xlen + 1);
            indices.push(idx + xlen + 2);
            indices.push(idx + 1);
        }
    }
    Indices::U32(indices)
}
