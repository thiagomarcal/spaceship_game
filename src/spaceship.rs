use bevy::prelude::*;

use crate::asset_loader::SceneAssets;
use crate::collision::{Collision, CollisionDamage, CollisionType};
use crate::health::Health;
use crate::movement::{Acceleration, MovingObjectBundle, Roll, Rotation, Velocity};
use crate::schedule::InGameSet;

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const STARTING_VELOCITY: Vec3 = Vec3::new(0.0, 0.0, 1.0);

const SPACESHIP_SPEED: f32 = 25.0;
const SPACESHIP_ROTATION_SPEED: f32 = 2.5;
const SPACESHIP_ROLL_SPEED: f32 = 2.5;

const SPACESHIP_HEALTH: f32 = 100.0;
const SPACESHIP_DAMAGE: f32 = 10.0;

const MISSILE_SPAWN_SCALAR: f32 = 10.0;

const SPACESHIP_RADIUS: f32 = 5.5;

const MISSILE_SPEED: f32 = 40.0;
const MISSILE_RADIUS: f32 = 0.5;

const MISSILE_HEALTH: f32 = 10.0;
const MISSILE_DAMAGE: f32 = 10.0;

#[derive(Component, Debug)]
pub struct Spaceship;

#[derive(Component, Debug)]
pub struct SpaceshipShield;

#[derive(Component, Debug)]
pub struct SpaceshipMissile;

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_spaceship)
            .add_systems(Update, (spaceship_movement_controls, spaceship_weapon_controls, spaceship_shield_controls)
                .chain()
                .in_set(InGameSet::UserInput));
    }
}


fn spawn_spaceship(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity::new(Vec3::ZERO),
            rotation: Rotation::new(0.0),
            roll: Roll::new(0.0),
            acceleration: Acceleration::new(Vec3::ZERO),
            collision: Collision::new(SPACESHIP_RADIUS, CollisionType::Spaceship),
            model: SceneBundle {
                scene: scene_assets.spaceship.clone(),
                transform: Transform::from_translation(STARTING_TRANSLATION),
                ..Default::default()
            },
        },
        Spaceship,
        Health::new(SPACESHIP_HEALTH),
        CollisionDamage::new(SPACESHIP_DAMAGE),
    ));
}

fn spaceship_movement_controls(
    mut query: Query<(&Transform, &mut Velocity, &mut Rotation, &mut Roll), With<Spaceship>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let Ok((transform, mut velocity, mut rotation, mut roll)) = query.get_single_mut() else {
        return;
    };

    let mut movement_calc = 0.0;
    let mut rotation_calc = 0.0;
    let mut roll_calc = 0.0;

    if keyboard_input.pressed(KeyCode::KeyS) {
        movement_calc = -SPACESHIP_SPEED;
    } else if keyboard_input.pressed(KeyCode::KeyW) {
        movement_calc = SPACESHIP_SPEED;
    }

    if keyboard_input.pressed(KeyCode::KeyA) {
        rotation_calc = SPACESHIP_ROTATION_SPEED;
    } else if keyboard_input.pressed(KeyCode::KeyD) {
        rotation_calc = -SPACESHIP_ROTATION_SPEED;
    }

    if keyboard_input.pressed(KeyCode::KeyQ) {
        roll_calc = SPACESHIP_ROLL_SPEED;
    } else if keyboard_input.pressed(KeyCode::KeyE) {
        roll_calc = -SPACESHIP_ROLL_SPEED;
    }

    velocity.value = -transform.forward() * movement_calc;
    rotation.value = rotation_calc;
    roll.value = roll_calc;
}

fn spaceship_weapon_controls(
    mut commands: Commands,
    mut query: Query<(&Transform), With<Spaceship>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    asset_scene: Res<SceneAssets>,
    time: Res<Time>,
) {
    let Ok(transform) = query.get_single() else {
        return;
    };

    let mut missile_translation = transform.translation + (-transform.forward() * MISSILE_SPAWN_SCALAR);
    let mut missile_velocity = -transform.forward() * MISSILE_SPEED;
    let mut missile_acceleration = Vec3::ZERO;

    if keyboard_input.just_pressed(KeyCode::Space) {
        commands.spawn((
            MovingObjectBundle {
                velocity: Velocity::new(missile_velocity),
                rotation: Rotation::new(0.0),
                roll: Roll::new(0.0),
                acceleration: Acceleration::new(missile_acceleration),
                collision: Collision::new(MISSILE_RADIUS, CollisionType::Missile),
                model: SceneBundle {
                    scene: asset_scene.missile.clone(),
                    transform: Transform::from_translation(missile_translation),
                    ..Default::default()
                },
            },
            SpaceshipMissile,
            Health::new(MISSILE_HEALTH),
            CollisionDamage::new(MISSILE_DAMAGE),
        ));
    }
}

fn spaceship_shield_controls(
    mut commands: Commands,
    mut query: Query<Entity, With<Spaceship>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let Ok(entity) = query.get_single_mut() else {
        return;
    };

    if keyboard_input.pressed(KeyCode::KeyF) {
        commands.entity(entity).insert(SpaceshipShield);
    }
}
