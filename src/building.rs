use bevy::prelude::*;

use crate::{asset_loader::SceneAssets};
pub struct BuildingPlugin;

#[derive(Component)]
pub struct Building;


fn spawn_buildings(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    println!("Spawning building!");
}

impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_buildings);
        // app.add_systems(Update, add_buildings);
    }
}