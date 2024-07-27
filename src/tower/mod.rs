use bevy::prelude::*;
use bevy_kira_audio::{prelude::*, AudioSource};

pub struct TowerPlugin;
impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RingBell>()
            .add_systems(Startup, setup)
            .add_systems(Update, ring_bell);
    }
}

#[derive(Resource)]
pub struct BellSounds(Handle<AudioSource>);

#[derive(Component)]
pub struct TowerBell;

fn setup(mut cmds: Commands, asset_server: Res<AssetServer>) {
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
            SpatialBundle::from_transform(Transform::from_xyz(0.0, 100.0, 0.0)),
            TowerBell,
            AudioEmitter::default(),
        ));
    });

    cmds.insert_resource(BellSounds(asset_server.load("audio/sfx/bells_3.ogg")))
}

/// Send this event to ring the bell n times.
#[derive(Event)]
pub struct RingBell(pub u8);

fn ring_bell(
    mut ev_bell: EventReader<RingBell>,
    mut q_bell: Query<&mut AudioEmitter, With<TowerBell>>,
    sounds: Res<BellSounds>,
    audio: Res<Audio>,
) {
    for ev in ev_bell.read() {
        println!("ring!");
        for mut emitter in &mut q_bell {
            let instance = audio.play(sounds.0.clone()).with_volume(0.0).handle();
            emitter.instances.push(instance);
        }
    }
}
