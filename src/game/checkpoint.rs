use bevy::prelude::*;

pub struct CheckpointPlugin;
impl Plugin for CheckpointPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Checkpoint>();
    }
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct Checkpoint {
    pos: Vec3,
}
