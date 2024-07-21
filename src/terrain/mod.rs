use avian3d::math::Vector3;
use bevy::prelude::*;
use bevy::render::{
    mesh::{Indices, VertexAttributeValues},
    render_asset::RenderAssetUsages,
    render_resource::PrimitiveTopology,
};

use noise::{NoiseFn, Perlin, Turbulence};
const XSIZE: usize = 200;
const YSIZE: usize = 200;
const SCALE: f64 = 15.0;
const RES: f32 = 5.0;

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
    xsize: usize,
    ysize: usize,
    scale: f64,
    frequency: f64,
    power: f64,
    roughness: usize,
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

        let cube_mesh_handle = meshes.add(create_cube_mesh(tp.xsize, tp.ysize, tp.scale, noise));
        // Render the mesh with the custom texture using a PbrBundle, add the marker.
        cmds.entity(entity).insert((PbrBundle {
            mesh: cube_mesh_handle,
            material: materials.add(StandardMaterial::default()),
            ..default()
        },));
    }
}

fn create_terrain(mut cmds: Commands) {
    cmds.spawn(TerrainParams {
        xsize: XSIZE,
        ysize: YSIZE,
        scale: SCALE,
        frequency: 0.35,
        power: 15.0,
        roughness: 5,
    });
}

fn create_cube_mesh(xsize: usize, ysize: usize, scale: f64, noise: impl NoiseFn<f64, 2>) -> Mesh {
    // Keep the mesh data accessible in future frames to be able to mutate it in toggle_texture.
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        create_vertex_grid(xsize, ysize, scale, noise),
    )
    .with_inserted_indices(create_indices_grid(xsize, ysize))
    .with_computed_normals()
}

fn create_vertex_grid(
    xsize: usize,
    ysize: usize,
    scale: f64,
    noise: impl NoiseFn<f64, 2>,
) -> Vec<Vec3> {
    let mut grid = vec![];
    for ix in 0..=xsize {
        for iy in 0..=ysize {
            let x = ix as f32 - xsize as f32 / 2.0;
            let y = iy as f32 - ysize as f32 / 2.0;
            let z = noise.get([(x / xsize as f32) as f64, (y / ysize as f32) as f64]) as f32;
            grid.push(Vec3::new(x * RES, z * scale as f32, y * RES));
        }
    }
    return grid;
}

fn create_indices_grid(xsize: usize, ysize: usize) -> Indices {
    let xsize = xsize as u32;
    let ysize = ysize as u32;

    let mut indices = vec![];
    for x in 0..xsize {
        for y in 0..ysize {
            let idx = y * (xsize + 1) + x;
            indices.push(idx);
            indices.push(idx + 1);
            indices.push(idx + xsize + 1);
            indices.push(idx + xsize + 1);
            indices.push(idx + 1);
            indices.push(idx + xsize + 2);
        }
    }
    Indices::U32(indices)
}
