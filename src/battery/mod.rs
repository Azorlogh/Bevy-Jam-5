use avian3d::{
    prelude::{Collider, Sensor},
    spatial_query::{SpatialQuery, SpatialQueryFilter},
};
use bevy::prelude::*;
use leafwing_input_manager::common_conditions::action_just_pressed;

use crate::{
    camera::{CameraRange, MainCamera},
    input::Action,
    player::{Inventory, Player},
};

pub struct BatteryPlugin;

impl Plugin for BatteryPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Battery>()
            .register_type::<BatterySlot>()
            .register_type::<PointingAtBattery>()
            .register_type::<PointingAtSlot>()
            .insert_resource(PointingAtBattery(None))
            .insert_resource(PointingAtSlot(None))
            .add_systems(
                Update,
                (
                    raycast_batteries,
                    (
                        (interact_text, interact_slot_text),
                        (take, place).run_if(action_just_pressed(Action::Interact)),
                    ),
                )
                    .chain(),
            );
    }
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct Battery;

#[derive(PartialEq, Resource, Reflect)]
#[reflect(Resource)]
pub struct PointingAtBattery(Option<Entity>);

#[derive(PartialEq, Resource, Reflect)]
#[reflect(Resource)]
pub struct PointingAtSlot(Option<Entity>);

#[derive(Component)]
pub struct BatteryTakeText;

#[derive(Component)]
pub struct BatteryPlaceText;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct BatterySlot {
    pub filled: bool,
}

fn raycast_batteries(
    mut pointing_at_battery: ResMut<PointingAtBattery>,
    mut pointing_at_slot: ResMut<PointingAtSlot>,
    q_camera: Query<(&GlobalTransform, &CameraRange), With<MainCamera>>,
    q_player: Query<(Entity, &Children), With<Player>>,
    q_battery: Query<(), With<Battery>>,
    q_slot: Query<&BatterySlot>,
    spatial_query: SpatialQuery,
    q_ignored: Query<Entity, (With<Sensor>, Without<Battery>, Without<BatterySlot>)>,
) {
    let Ok((player_e, player_children)) = q_player.get_single() else {
        return;
    };

    let Ok((cam_tr, range)) = q_camera.get_single() else {
        return;
    };

    if let Some(hit) = spatial_query.cast_ray_predicate(
        cam_tr.translation(), // Origin
        cam_tr.forward(),     // Direction
        range.0,              // Maximum time of impact (travel distance)
        true,                 // Does the ray treat colliders as "solid"
        SpatialQueryFilter::from_excluded_entities(vec![
            player_e,
            player_children[0],
            player_children[1],
        ]), // Query filter
        &|e| !q_ignored.contains(e),
    ) {
        pointing_at_battery.set_if_neq(PointingAtBattery(
            q_battery.contains(hit.entity).then_some(hit.entity),
        ));
        pointing_at_slot.set_if_neq(PointingAtSlot(
            q_slot
                .get(hit.entity)
                .is_ok_and(|slot| !slot.filled)
                .then_some(hit.entity),
        ));
    }
}

fn interact_text(
    pointing_at: Res<PointingAtBattery>,
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    q_text: Query<Entity, With<BatteryTakeText>>,
) {
    if pointing_at.0.is_some() && q_text.is_empty() {
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
    } else if pointing_at.0.is_none() {
        for e in &q_text {
            cmds.entity(e).despawn_recursive();
        }
    }
}

fn interact_slot_text(
    pointing_at: Res<PointingAtSlot>,
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    q_inventory: Query<&Inventory>,
    q_text: Query<Entity, With<BatteryPlaceText>>,
) {
    if pointing_at.0.is_some() && q_text.is_empty() {
        let inventory = q_inventory.single();
        cmds.spawn((
            BatteryPlaceText,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::End,
                    justify_content: JustifyContent::Center,
                    padding: UiRect::all(Val::Percent(5.0)),
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                if inventory.batteries.len() == 0 {
                    "You are not carrying any batteries."
                } else {
                    "Press <interact> to place a battery."
                },
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 30.0,
                    color: Color::srgb(0.9, 0.9, 0.9),
                },
            ));
        });
    } else if pointing_at.0.is_none() {
        for e in &q_text {
            cmds.entity(e).despawn_recursive();
        }
    }
}

fn take(
    mut cmds: Commands,
    pointing_at: Res<PointingAtBattery>,
    mut q_inventory: Query<&mut Inventory>,
) {
    if let Some(battery_e) = pointing_at.0 {
        q_inventory.single_mut().batteries.push(battery_e);
        cmds.entity(battery_e)
            .insert(Visibility::Hidden)
            .remove::<Collider>();
    }
}

fn place(
    pointing_at: Res<PointingAtSlot>,
    mut q_inventory: Query<&mut Inventory>,
    mut q_battery: Query<&mut Transform, With<Battery>>,
    mut q_slots: Query<(&Transform, &mut BatterySlot), (With<BatterySlot>, Without<Battery>)>,
) {
    let Some((slot_tr, mut slot)) = pointing_at.0.and_then(|e| q_slots.get_mut(e).ok()) else {
        info!("not pointing at slot");
        return;
    };

    let Ok(mut inventory) = q_inventory.get_single_mut() else {
        info!("no inventory");
        return;
    };

    let Some(battery_e) = inventory.batteries.pop() else {
        info!("no battery in inventory");
        return;
    };

    q_battery.get_mut(battery_e).unwrap().translation = slot_tr.translation;
    slot.filled = true;
}
