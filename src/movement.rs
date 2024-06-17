use bevy::prelude::*;

use crate::collision::Collision;
use crate::schedule::InGameSet;

#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3,
}

impl Velocity {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}

#[derive(Component, Debug)]
pub struct Acceleration {
    pub value: Vec3,
}

impl Acceleration {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}

#[derive(Component, Debug)]
pub struct Rotation {
    pub value: f32,
}

impl Rotation {
    pub fn new(value: f32) -> Self {
        Self { value }
    }
}

#[derive(Component, Debug)]
pub struct Roll {
    pub value: f32,
}

impl Roll {
    pub fn new(value: f32) -> Self {
        Self { value }
    }
}


#[derive(Bundle)]
pub struct MovingObjectBundle {
    pub velocity: Velocity,
    pub rotation: Rotation,
    pub roll: Roll,
    pub acceleration: Acceleration,
    pub collision: Collision,
    pub model: SceneBundle,
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_velocity, update_positions)
            .chain()
            .in_set(InGameSet::EntityUpdates),
        );
    }
}

fn update_velocity(mut query: Query<(&Acceleration, &mut Velocity)>, time: Res<Time>) {
    for (acceleration, mut velocity) in query.iter_mut() {
        velocity.value += acceleration.value * time.delta_seconds();
    }
}

fn update_positions(mut query: Query<(&Velocity, &Rotation, &Roll, &mut Transform)>, time: Res<Time>) {
    for (velocity, rotation, roll, mut transform) in query.iter_mut() {
        transform.translation += velocity.value * time.delta_seconds();
        transform.rotate_y(rotation.value * time.delta_seconds());
        transform.rotate_local_z(roll.value * time.delta_seconds());
    }
}
