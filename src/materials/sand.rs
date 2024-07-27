use bevy::{
    pbr::{ExtendedMaterial, MaterialExtension},
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
};

pub type SandMaterial = ExtendedMaterial<StandardMaterial, SandMaterialExtension>;

#[derive(Default, Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct SandMaterialExtension {
    #[uniform(100)]
    color: Vec3,
}

impl MaterialExtension for SandMaterialExtension {
    fn fragment_shader() -> ShaderRef {
        "shaders/sand.wgsl".into()
    }

    fn deferred_fragment_shader() -> ShaderRef {
        "shaders/sand.wgsl".into()
    }
}
