use bevy::prelude::*;

use crate::player::{Inventory, Player};

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
    inventory: Inventory,
}

pub fn save_checkpoint(world: &mut World) {
    let (transform, inventory) = world
        .query_filtered::<(&Transform, &Inventory), With<Player>>()
        .single(world);
    let checkpoint = Checkpoint {
        pos: transform.translation,
        inventory: inventory.clone(),
    };
    world.insert_resource(checkpoint);
}

pub fn load_checkpoint(world: &mut World) {
    world.resource_scope(|world: &mut World, checkpoint: Mut<Checkpoint>| {
        let (mut transform, mut inventory) = world
            .query_filtered::<(&mut Transform, &mut Inventory), With<Player>>()
            .single_mut(world);
        transform.translation = checkpoint.pos;
        *inventory = checkpoint.inventory.clone();
    });
}
