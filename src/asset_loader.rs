use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub spaceship: Handle<Scene>,
    pub rock: Handle<Scene>,
    pub missile: Handle<Scene>,
}

pub struct AssetLoaderPlugin;


impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>()
            .add_systems(Startup, load_assets);
    }
}

fn load_assets(mut scene_assets: ResMut<SceneAssets>, asset_server: Res<AssetServer>) {
    scene_assets.spaceship = asset_server.load("Spaceship.glb#Scene0");
    scene_assets.rock = asset_server.load("Rock.glb#Scene0");
    scene_assets.missile = asset_server.load("Missile.glb#Scene0");
}
