use bevy::prelude::*;

use asset_loader::AssetLoaderPlugin;
use camera::CameraPlugin;
use debug::DebugPlugin;
use movement::MovementPlugin;
use rock::RockPlugin;
use spaceship::SpaceshipPlugin;

use crate::collision::CollisionPlugin;
use crate::despawn::DespawnPlugin;
use crate::schedule::SchedulePlugin;
use crate::state::GameStatePlugin;

mod debug;
mod movement;
mod spaceship;
mod camera;
mod rock;
mod asset_loader;
mod collision;
mod despawn;
mod schedule;
mod state;
mod health;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 750.0,
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(SpaceshipPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(RockPlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(CollisionPlugin)
        .add_plugins(DespawnPlugin)
        .add_plugins(SchedulePlugin)
        .add_plugins(GameStatePlugin)
        .run();
}
