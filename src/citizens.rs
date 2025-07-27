use bevy::prelude::*;
use bevy_rand::prelude::{Entropy, WyRand};
use rand::Rng;

use crate::{asset_loader::SceneAssets, building::Home};

const CITIZEN_COUNT: usize = 500;

#[derive(Component)]
pub struct Citizen;

#[derive(Component)]
pub struct Happiness(f32);

fn spawn_individuals(
    mut commands: Commands,
    scene_assets: Res<SceneAssets>,
    mut q_source: Single<&mut Entropy<WyRand>>,
) {
    //TODO global rng
    for _ in 0..=CITIZEN_COUNT {
        let pos = Vec3::new(q_source.random(), 5., q_source.random());
        commands.spawn((
            Citizen,
            Happiness(100.),
            SceneRoot(scene_assets.citizen_one.clone()),
            Transform {
                translation: 40. * pos,
                ..default()
            },
        ));
    }
}

fn calculate_happiness() {
    // let a = random_range(0..=10);
}
pub struct PopulationPlugin;

impl Plugin for PopulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_individuals);
    }
}
