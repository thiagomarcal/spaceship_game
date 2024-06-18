use bevy::prelude::*;

use crate::health::Health;
use crate::schedule::InGameSet;

const DESPAWN_DISTANCE: f32 = 100.0;

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (despawn_entities, despawn_dead_entities).in_set(InGameSet::DespawnEntities));
    }
}

fn despawn_entities(
    mut commands: Commands,
    query: Query<(Entity, &GlobalTransform)>,
) {
    for (entity, transform) in query.iter() {
        if transform.translation().distance(Vec3::ZERO) > DESPAWN_DISTANCE {
            // if entity exists in the query, despawn it
            if let Ok(result) = query.get(entity) {
                let (entity, _) = result;
                commands.entity(entity).despawn();
            }
        }
    }
}

fn despawn_dead_entities(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Health)>,
) {
    for (entity, health) in query.iter_mut() {
        if health.value <= 0.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

