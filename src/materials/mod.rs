use bevy::{asset::load_internal_asset, pbr::ExtendedMaterial, prelude::*};

pub mod sand;

pub const COMMON: Handle<Shader> = Handle::weak_from_u128(2484523442896896);
pub const SIMPLEX_VEC3F: Handle<Shader> = Handle::weak_from_u128(7152507656601600);

pub struct BuiltinMaterialsPlugin;

impl Plugin for BuiltinMaterialsPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            COMMON,
            "../../assets/shaders/common.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            SIMPLEX_VEC3F,
            "../../assets/shaders/simplex_vec3f.wgsl",
            Shader::from_wgsl
        );
        app.add_plugins(MaterialPlugin::<
            ExtendedMaterial<StandardMaterial, sand::SandMaterialExtension>,
        >::default());
    }
}
