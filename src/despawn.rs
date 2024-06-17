use bevy::prelude::*;

use crate::schedule::InGameSet;

const DESPAWN_DISTANCE: f32 = 100.0;

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, despawn_entities.in_set(InGameSet::DespawnEntities));
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

