use avian3d::spatial_query::{SpatialQuery, SpatialQueryFilter};
use bevy::{input::common_conditions::input_just_pressed, prelude::*, reflect};

use crate::{
    battery,
    camera::{follow::Eyes, MainCamera},
    player::{Inventory, Player},
    tower::BatterySlot,
};

pub struct BatteryPlugin;

impl Plugin for BatteryPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Battery>()
            .add_systems(Startup, setup)
            .add_systems(Update, player_interact)
            .add_systems(Update, take.run_if(input_just_pressed(KeyCode::KeyT)))
            .add_systems(Update, place.run_if(input_just_pressed(KeyCode::KeyY)));
    }
}

fn setup(mut cmds: Commands, asset_server: Res<AssetServer>) {
    cmds.spawn((
        Name::new("Battery"),
        SceneBundle {
            scene: asset_server.load("levels/Battery.glb#Scene0"),
            transform: Transform::from_xyz(105.0, 0.0, 0.0),
            ..default()
        },
    ));
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct Battery {
    pub name: String,
}

fn player_interact(
    q_player: Query<&Transform, With<MainCamera>>,
    q_battery: Query<Entity, With<Battery>>,
    spatial_query: SpatialQuery,
) {
    let Ok(player) = q_player.get_single() else {
        return;
    };

    if let Some(hit) = spatial_query.cast_ray(
        player.translation,            // Origin
        player.forward(),              // Direction
        100.0,                         // Maximum time of impact (travel distance)
        true,                          // Does the ray treat colliders as "solid"
        SpatialQueryFilter::default(), // Query filter
    ) {
        for battery in q_battery.iter() {
            if hit.entity == battery {
                // Show text for the player
                dbg!("Battery : {:?} hit", battery);
            }
        }
    }
}

fn take(
    mut cmds: Commands,
    mut q_inventory: Query<&mut Inventory>,
    mut q_battery: Query<(&Battery, Entity), With<Battery>>,
) {
    let Ok((battery, b_entity)) = q_battery.get_single_mut() else {
        return;
    };
    let Ok(mut inventory) = q_inventory.get_single_mut() else {
        return;
    };

    dbg!(&inventory);

    inventory.batteries.push(battery.name.clone());
    cmds.entity(b_entity).despawn_recursive();

    // add ui indicator
    dbg!(&inventory);
}

fn place(
    mut cmds: Commands,
    mut q_inventory: Query<&mut Inventory>,
    mut q_battery: Query<&Battery>,
    mut q_slots: Query<&mut BatterySlot>,
) {
    for mut slot in q_slots.iter_mut() {
        let Ok(battery) = q_battery.get_single_mut() else {
            return;
        };

        let Ok(mut inventory) = q_inventory.get_single_mut() else {
            return;
        };

        dbg!(&inventory);

        let obj = inventory.batteries.pop();

        if obj.is_none() {
            dbg!("No object");
            continue;
        }

        if slot.name == obj.unwrap() {
            slot.empty = false;
            // place battery in slot
        }

        // add ui indicator
        dbg!(&inventory);
    }
}
