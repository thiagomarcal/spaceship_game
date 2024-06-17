use std::ops::Range;

use bevy::prelude::*;
use rand::Rng;

use crate::asset_loader::SceneAssets;
use crate::collision::{Collision, CollisionDamage, CollisionType};
use crate::health::Health;
use crate::movement::{Acceleration, MovingObjectBundle, Roll, Rotation, Velocity};
use crate::schedule::InGameSet;

const ROCK_RANGE_X: Range<f32> = -25.0..25.0;
const ROCK_RANGE_Z: Range<f32> = 0.0..50.0;
const ROCK_VELOCITY_SCALAR: f32 = 5.0;
const ROCK_ACCELERATION_SCALAR: f32 = 1.0;
const ROCK_SPAWN_TIME_SECONDS: f32 = 1.0;

const ROCK_HEALTH: f32 = 30.0;

const ROCK_DAMAGE: f32 = 10.0;

const RADIUS: f32 = 2.5;

#[derive(Component, Debug)]
pub struct Rock;

pub struct RockPlugin;

impl Plugin for RockPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer {
            timer: Timer::from_seconds(ROCK_SPAWN_TIME_SECONDS, TimerMode::Repeating),
        })
            .add_systems(Update, (spawn_rock, rotate_rocks).in_set(InGameSet::EntityUpdates));
    }
}

#[derive(Resource, Debug)]
pub struct SpawnTimer {
    pub timer: Timer,
}

fn spawn_rock(
    mut commands: Commands,
    mut spawn_timer: ResMut<SpawnTimer>,
    time: Res<Time>,
    scene_assets: Res<SceneAssets>,
) {
    spawn_timer.timer.tick(time.delta());

    if !spawn_timer.timer.just_finished() {
        return;
    }

    let mut rng = rand::thread_rng();

    let translation = Vec3::new(rng.gen_range(ROCK_RANGE_X), 0.0, rng.gen_range(ROCK_RANGE_Z));

    let mut random_unit_vector =
        || Vec3::new(rng.gen_range(-1.0..1.0), 0.0, rng.gen_range(-1.0..1.0)).normalize_or_zero();

    let velocity = random_unit_vector() * ROCK_VELOCITY_SCALAR;
    let acceleration = random_unit_vector() * ROCK_ACCELERATION_SCALAR;

    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity::new(velocity),
            rotation: Rotation::new(0.0),
            roll: Roll::new(0.0),
            acceleration: Acceleration::new(acceleration),
            collision: Collision::new(RADIUS, CollisionType::Rock),
            model: SceneBundle {
                scene: scene_assets.rock.clone(),
                transform: Transform::from_translation(translation),
                ..Default::default()
            },
        },
        Rock,
        Health::new(ROCK_HEALTH),
        CollisionDamage::new(ROCK_DAMAGE),
    ));
}

const ROCK_ROTATION_SPEED: f32 = 2.5;

fn rotate_rocks(
    mut query: Query<(&mut Transform), With<Rock>>,
    time: Res<Time>,
) {
    for (mut transform) in query.iter_mut() {
        transform.rotate_local_z(ROCK_ROTATION_SPEED * time.delta_seconds());
    }
}

// fn handle_rock_collision(
//     mut commands: Commands,
//     mut query: Query<(Entity, &Transform, &mut Collision), With<Rock>>,
// ) {
//     for (entity, transform, collision) in query.iter_mut() {
//         let colliding_entity = collision.collisions.get(&entity);
//         if let Some(colliding_entity) = colliding_entity {
//             match colliding_entity {
//                 CollisionType::Spaceship => {
//                     // commands.entity(entity).despawn_recursive();
//                 }
//                 CollisionType::Missile => {
//                     commands.entity(entity).despawn_recursive();
//                 }
//                 _ => {}
//             }
//         }
//     }
// }
