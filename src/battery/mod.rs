use avian3d::spatial_query::{SpatialQuery, SpatialQueryFilter};
use bevy::{input::common_conditions::input_just_pressed, prelude::*, reflect};
use leafwing_input_manager::common_conditions::action_just_pressed;

use crate::{
    battery,
    camera::{follow::Eyes, MainCamera},
    input::Action,
    player::{Inventory, Player},
    shelter::SafeZoneText,
    tower::BatterySlot,
};

pub struct BatteryPlugin;

impl Plugin for BatteryPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Battery>()
            .add_systems(Startup, setup)
            .add_systems(Update, player_interact)
            .add_systems(
                Update,
                (take, place).run_if(action_just_pressed(Action::Interact)),
            );
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

#[derive(Component)]
pub struct BatteryTakeText;

fn player_interact(
    mut cmds: Commands,
    q_player: Query<&Transform, With<MainCamera>>,
    q_battery: Query<Entity, With<Battery>>,
    spatial_query: SpatialQuery,
    asset_server: Res<AssetServer>,
    q_text: Query<Entity, With<BatteryTakeText>>,
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
                cmds.spawn((
                    BatteryTakeText,
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            align_items: AlignItems::End,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Press <interact> to take the battery.",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 30.0,
                            color: Color::srgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
            } else {
                for e in &q_text {
                    cmds.entity(e).despawn_recursive();
                }
            }
        }
    } else {
        for e in &q_text {
            cmds.entity(e).despawn_recursive();
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
