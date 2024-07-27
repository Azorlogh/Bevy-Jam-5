use bevy::prelude::*;

pub mod d2;
mod ring;

#[derive(Component, Reflect)]
pub enum ChunkVisibility {
    Visible,
    Hidden,
}

#[derive(Component, Reflect)]
pub struct ChunkReady;

// use bevy::prelude::*;
// use d2::Lod2dPlugin;
// use d3::Lod3dPlugin;

// pub struct LodPlugin;

// impl Plugin for LodPlugin {
// 	fn build(&self, app: &mut App) {
// 		app.add_plugins((Lod2dPlugin, Lod3dPlugin));
// 	}
// }
