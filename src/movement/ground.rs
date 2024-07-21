use avian3d::prelude::*;
use bevy::prelude::*;

#[derive(Bundle)]
pub struct GroundSensorBundle {
    ground_sensor: GroundSensor,
    transform: TransformBundle,
    collider: Collider,
    sensor: Sensor,
    colliding_entities: CollidingEntities,
}

impl GroundSensorBundle {
    pub fn new(radius: f32, y_offset: f32) -> Self {
        Self {
            ground_sensor: default(),
            transform: TransformBundle::from_transform(Transform::from_xyz(0.0, y_offset, 0.0)),
            collider: Collider::cylinder(radius, 0.1),
            sensor: Sensor,
            colliding_entities: default(),
        }
    }
}

#[derive(Debug, Component, Reflect)]
pub struct OnGround(pub bool);

/// Put this on a collider of the agentt to control OnGround
#[derive(Default, Component)]
pub struct GroundSensor;

pub fn detect_ground(
    q_sensor: Query<(&Parent, &CollidingEntities), With<GroundSensor>>,
    mut q_agent: Query<&mut OnGround>,
) {
    for (parent, sensor) in &q_sensor {
        if let Ok(mut on_ground) = q_agent.get_mut(parent.get()) {
            on_ground.0 = !sensor.is_empty();
        }
    }
}
