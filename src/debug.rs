use avian3d::debug_render::{PhysicsDebugPlugin, PhysicsGizmos};
use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WorldInspectorEnabled(false))
            .add_plugins((
                PhysicsDebugPlugin::new(Update),
                WorldInspectorPlugin::new().run_if(resource_equals(WorldInspectorEnabled(true))),
            ))
            .add_systems(Startup, |mut store: ResMut<GizmoConfigStore>| {
                let cfg = store.config_mut::<PhysicsGizmos>().0;
                cfg.enabled = !cfg.enabled;
            })
            .add_systems(
                Update,
                (
                    (|mut store: ResMut<GizmoConfigStore>| {
                        let cfg = store.config_mut::<PhysicsGizmos>().0;
                        cfg.enabled = !cfg.enabled
                    })
                    .run_if(input_just_pressed(KeyCode::KeyP)),
                    (|mut time: ResMut<Time<Virtual>>| {
                        let s = time.relative_speed();
                        time.set_relative_speed(s * 1.3);
                    })
                    .run_if(input_just_pressed(KeyCode::ArrowLeft)),
                    (|mut time: ResMut<Time<Virtual>>| {
                        let s = time.relative_speed();
                        time.set_relative_speed(s / 1.3);
                    })
                    .run_if(input_just_pressed(KeyCode::ArrowRight)),
                    (|mut enabled: ResMut<WorldInspectorEnabled>| enabled.0 = !enabled.0)
                        .run_if(input_just_pressed(KeyCode::KeyI)),
                ),
            );
    }
}

#[derive(PartialEq, Resource)]
struct WorldInspectorEnabled(bool);
