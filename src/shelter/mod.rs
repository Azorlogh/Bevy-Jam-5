use std::f32::consts::TAU;

use avian3d::prelude::CollidingEntities;
use bevy::prelude::*;

use crate::{player::Player, terrain::TerrainParams, util::poisson_disc_sampling};

pub struct ShelterPlugin;
impl Plugin for ShelterPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ShelterSafeZone>()
            .insert_resource(PlayerIsSafe(false))
            .add_systems(Startup, setup)
            .add_systems(Update, (check_safe_zones, safe_text));
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ShelterSafeZone;

fn setup(mut cmds: Commands, asset_server: Res<AssetServer>, terrain_params: Res<TerrainParams>) {
    let region = 4000.0;
    for p in poisson_disc_sampling(1000.0, region, 30000) {
        let p = p - region / 2.0;
        let height = terrain_params.get_height(p) + 3.0;
        cmds.spawn((
            Name::new("Shelter"),
            SceneBundle {
                scene: asset_server.load("levels/Shelter.glb#Scene0"),
                transform: Transform::from_translation(p.extend(height).xzy())
                    .with_rotation(Quat::from_rotation_y(rand::random::<f32>() * TAU)),
                ..default()
            },
        ));
    }
}

#[derive(Resource)]
pub struct PlayerIsSafe(pub bool);

fn check_safe_zones(
    q_player: Query<&CollidingEntities, With<Player>>,
    q_safe_zone: Query<(), With<ShelterSafeZone>>,
    mut player_is_safe: ResMut<PlayerIsSafe>,
) {
    let Ok(colliding_entities) = q_player.get_single() else {
        return;
    };
    player_is_safe.0 = colliding_entities.iter().any(|e| q_safe_zone.contains(*e));
}

#[derive(Component)]
pub struct SafeZoneText;

fn safe_text(
    mut cmds: Commands,
    player_is_safe: Res<PlayerIsSafe>,
    mut prev_safe: Local<bool>,
    asset_server: Res<AssetServer>,
    q_text: Query<Entity, With<SafeZoneText>>,
) {
    if !*prev_safe && player_is_safe.0 {
        cmds.spawn((
            SafeZoneText,
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
                "You are safe from the storm.",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 30.0,
                    color: Color::srgb(0.9, 0.9, 0.9),
                },
            ));
        });
    } else if *prev_safe && !player_is_safe.0 {
        for e in &q_text {
            cmds.entity(e).despawn_recursive();
        }
    }

    *prev_safe = player_is_safe.0;
}
