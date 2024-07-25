use bevy::prelude::*;

use crate::player::Player;

pub struct MonolithPlugin;
impl Plugin for MonolithPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Monolith>()
            .add_systems(Update, (monolith_collect, monolith_light_up));
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Monolith;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct CollectedMonolith;

fn monolith_collect(
    mut cmds: Commands,
    q_player: Query<&GlobalTransform, With<Player>>,
    q_monolith: Query<(Entity, &GlobalTransform), (With<Monolith>, Without<CollectedMonolith>)>,
) {
    let Ok(player_tr) = q_player.get_single() else {
        return;
    };
    for (monolith_e, tr) in &q_monolith {
        if tr.translation().distance(player_tr.translation()) < 3.0 {
            cmds.entity(monolith_e).insert(CollectedMonolith);
        }
    }
}

fn monolith_light_up(
    q_monolith: Query<Entity, Added<CollectedMonolith>>,
    q_children: Query<&Children>,
    q_name: Query<&Name>,
    q_material: Query<&Handle<StandardMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for e in &q_monolith {
        for e in q_children.iter_descendants(e) {
            if q_name.get(e).map(|n| n.as_str()) == Ok("Sigil.Mesh") {
                let Ok(mat_handle) = q_material.get(e) else {
                    continue;
                };
                materials.get_mut(mat_handle).unwrap().emissive = LinearRgba::WHITE * 100.0;
            }
        }
    }
}
