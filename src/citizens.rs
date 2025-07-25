use bevy::prelude::*;
use rand::{Rng, random_range};

use crate::asset_loader::SceneAssets;

#[derive(Component)]
pub struct Citizen;

#[derive(Component)]
pub struct Happiness(f32);

fn spawn_individuals(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    //TODO global rng
    let mut rng = rand::rng();
    for _ in 0..=500 {
        let pos = Vec3::new(rng.random(), 5., rng.random());
        commands.spawn((
            Citizen,
            Happiness(100.),
            SceneRoot(
                scene_assets.citizen_one.clone()),
                Transform {
                    translation: 40.*pos,
                    ..default()
                },
        ));
    }
}

fn calculate_happiness() {
    let a = random_range(0..=10);
}
pub struct PopulationPlugin;

impl Plugin for PopulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_individuals);
    }
}
