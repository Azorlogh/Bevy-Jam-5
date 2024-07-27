use bevy::{audio::SpatialScale, prelude::*};

pub struct TowerPlugin;
impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RingBell>()
            .add_systems(Startup, setup)
            .add_systems(Update, ring_bell);
    }
}

#[derive(Resource)]
pub struct BellSounds([Handle<AudioSource>; 4]);

#[derive(Component)]
pub struct TowerBell;

fn setup(mut cmds: Commands, asset_server: Res<AssetServer>, mut meshes: ResMut<Assets<Mesh>>) {
    cmds.spawn((
        Name::new("Clocktower"),
        SceneBundle {
            scene: asset_server.load("levels/Hub.glb#Scene0"),
            transform: Transform::from_xyz(100.0, -1.0, 0.0)
                .with_rotation(Quat::from_rotation_x(0.1) * Quat::from_rotation_y(0.2)),
            ..default()
        },
    ))
    .with_children(|cmds| {
        cmds.spawn((
            // SpatialBundle::from_transform(Transform::from_xyz(0.0, 100.0, 0.0)),
            PbrBundle {
                mesh: meshes.add(Sphere::new(3.0)),
                transform: Transform::from_xyz(0.0, 100.0, 0.0),
                ..default()
            },
            TowerBell,
            // AudioEmitter::default(),
        ));
    });

    cmds.insert_resource(BellSounds([
        asset_server.load("audio/sfx/tower_bells_1.ogg"),
        asset_server.load("audio/sfx/tower_bells_2.ogg"),
        asset_server.load("audio/sfx/tower_bells_3.ogg"),
        asset_server.load("audio/sfx/tower_bells_long.ogg"),
    ]))
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
                            .with_spatial_scale(SpatialScale::new(0.002))
                            .with_volume(bevy::audio::Volume::new(1.0)),
                    },
                ));
            });
        }
    }
}
