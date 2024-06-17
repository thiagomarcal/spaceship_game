use std::collections::HashMap;

use bevy::prelude::*;

use crate::health::Health;
use crate::rock::Rock;
use crate::schedule::InGameSet;
use crate::spaceship::{Spaceship, SpaceshipMissile};

#[derive(Debug, Copy, Clone)]
pub enum CollisionType {
    Spaceship,
    Rock,
    Missile,
}

#[derive(Component, Debug)]
pub struct Collision {
    pub collisions: HashMap<Entity, (Entity, CollisionType)>,
    pub radius: f32,
    pub collision_type: CollisionType,
}

impl Collision {
    pub fn new(radius: f32, collision_type: CollisionType) -> Self {
        Self {
            collisions: HashMap::new(),
            radius,
            collision_type,
        }
    }
}

#[derive(Component, Debug)]
pub struct CollisionDamage {
    amount: f32,
}

impl CollisionDamage {
    pub fn new(amount: f32) -> Self {
        Self { amount }
    }
}

#[derive(Event, Debug)]
pub struct CollisionEvent {
    pub entity: Entity,
    pub collided_with: Entity,
    pub collided_type: CollisionType,
}

impl CollisionEvent {
    pub fn new(entity: Entity, collided_with: Entity, collided_type: CollisionType) -> Self {
        Self {
            entity,
            collided_with,
            collided_type,
        }
    }
}


pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, check_collisions.in_set(InGameSet::CollisionDetection))
            .add_systems(Update,
                         (
                             (
                                 handle_collisions::<Rock>,
                                 handle_collisions::<Spaceship>,
                                 handle_collisions::<SpaceshipMissile>
                             ),
                             apply_collision_damage,
                         )
                             .chain()
                             .in_set(InGameSet::EntityUpdates),
            )
            .add_event::<CollisionEvent>();
    }
}


fn check_collisions(
    mut query: Query<(Entity, &Transform, &mut Collision)>,
) {
    let mut collisions: HashMap<Entity, (Entity, CollisionType)> = HashMap::new();

    for (entity, transform, collision) in query.iter() {
        for (other_entity, other_transform, other_collision) in query.iter() {
            if entity == other_entity {
                continue;
            }
            let distance = transform.translation.distance(other_transform.translation);
            if distance < collision.radius + other_collision.radius {
                collisions.insert(entity, (other_entity, other_collision.collision_type));
            }
        }
    }

    for (_, _, mut collision) in query.iter_mut() {
        collision.collisions = collisions.clone();
    }
}

fn handle_collisions<T: Component>(
    query: Query<(Entity, &Collision), With<T>>,
    mut event_writer: EventWriter<CollisionEvent>,
) {
    for (entity, collision) in query.iter() {
        if let Some((collided_entity, collided_type)) = collision.collisions.get(&entity) {
            event_writer.send(CollisionEvent::new(entity, *collided_entity, *collided_type));
        }
    }
}

fn apply_collision_damage(
    mut event_reader: EventReader<CollisionEvent>,
    mut heath_query: Query<&mut Health>,
    collision_damage_query: Query<&CollisionDamage>,
) {
    for event in event_reader.read() {
        let Ok(mut health) = heath_query.get_mut(event.entity) else {
            continue;
        };

        let Ok(collision_damage) = collision_damage_query.get(event.entity) else {
            continue;
        };

        health.value -= collision_damage.amount;
    }
}