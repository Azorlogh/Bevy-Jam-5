mod clock;

use bevy::{audio::SpatialScale, prelude::*};

pub struct TowerPlugin;
impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(clock::ClockPlugin)
            .register_type::<TowerBell>()
            .register_type::<BatterySlot>()
            .add_event::<RingBell>()
            .add_systems(Startup, setup)
            .add_systems(Update, ring_bell);
    }
}

#[derive(Resource)]
pub struct BellSounds([Handle<AudioSource>; 4]);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct TowerBell;

fn setup(mut cmds: Commands, asset_server: Res<AssetServer>) {
    cmds.spawn((
        Name::new("Clocktower"),
        SceneBundle {
            scene: asset_server.load("levels/Tower.glb#Scene0"),
            transform: Transform::from_xyz(100.0, -1.0, 0.0)
                .with_rotation(Quat::from_rotation_x(0.1) * Quat::from_rotation_y(0.2)),
            ..default()
        },
    ));

    cmds.insert_resource(BellSounds([
        asset_server.load("audio/sfx/tower_bells_1.ogg"),
        asset_server.load("audio/sfx/tower_bells_2.ogg"),
        asset_server.load("audio/sfx/tower_bells_3.ogg"),
        asset_server.load("audio/sfx/tower_bells_long.ogg"),
    ]));
}

/// Send this event to ring the bell n times.
#[derive(Event)]
pub struct RingBell(pub u8);

fn ring_bell(
    mut cmds: Commands,
    mut ev_bell: EventReader<RingBell>,
    q_bell: Query<Entity, With<TowerBell>>,
    sounds: Res<BellSounds>,
) {
    for ev in ev_bell.read() {
        for e in &q_bell {
            cmds.entity(e).with_children(|cmds| {
                cmds.spawn((
                    SpatialBundle::default(),
                    AudioBundle {
                        source: sounds.0[ev.0 as usize].clone(),
                        settings: PlaybackSettings::DESPAWN
                            .with_spatial(true)
                            .with_spatial_scale(SpatialScale::new(0.0005))
                            .with_volume(bevy::audio::Volume::new(0.01)),
                    },
                ));
            });
        }
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct BatterySlot {
    pub empty: bool,
    pub name: String,
}
