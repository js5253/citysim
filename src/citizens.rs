use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};
use bevy_rand::prelude::{Entropy, WyRand};
use rand::Rng;

use crate::{asset_loader::SceneAssets, building::Home};

const CITIZEN_COUNT: usize = 500;

#[derive(Component)]
pub struct Citizen;

#[derive(Component)]
pub struct Happiness(f32);

#[derive(Component, Debug)]
pub struct LivesIn(Entity);

#[derive(Event)]
pub struct CitizenSpawnRequested;

fn spawn_initial_citizens(
    mut commands: Commands,
    scene_assets: Res<SceneAssets>,
    mut q_source: Single<&mut Entropy<WyRand>>,
    available_homes: Query<Entity, With<Home>>,
) {
    let available_homes: Vec<Entity> = available_homes.iter().collect();
    println!("Spawning individuals");
    //TODO global rng
    for _ in 0..=CITIZEN_COUNT {
        let pos = Vec3::new(q_source.random(), 5., q_source.random());
        commands.spawn((
            Citizen,
            LivesIn(
                *available_homes
                    .get(q_source.random_range(0..available_homes.len()))
                    .unwrap(),
            ),
            Happiness(100.),
            SceneRoot(scene_assets.citizen_one.clone()),
            Transform {
                translation: 40. * pos,
                ..default()
            },
        ));
    }
}
fn spawn_citizen(mut reader: EventReader<CitizenSpawnRequested>, scene_assets: Res<SceneAssets>, mut commands: Commands, available_homes: Query<Entity, With<Home>>,     mut q_source: Single<&mut Entropy<WyRand>>,

) {
    let available_homes: Vec<Entity> = available_homes.iter().collect();
    for event in reader.read() {
        let pos = Vec3::new(q_source.random(), 5., q_source.random());
        commands.spawn((
            Citizen,
            LivesIn(
                *available_homes
                    .get(q_source.random_range(0..available_homes.len()))
                    .unwrap(),
            ),
            Happiness(100.),
            SceneRoot(scene_assets.citizen_one.clone()),
            Transform {
                translation: 40. * pos,
                ..default()
            },
        ));
    }
}
fn print_time(time: Res<Time>, citizens: Query<(Entity, &LivesIn), With<Citizen>>) {
    for (citizen, lives_in) in citizens.iter() {
        println!("Lives in: {:?}", lives_in);
    }
}

pub struct PopulationPlugin;

impl Plugin for PopulationPlugin {
    fn build(&self, app: &mut App) {
        println!("Initializing population");
        app.add_event::<CitizenSpawnRequested>();
        app.add_systems(PostStartup, spawn_initial_citizens);
        app.add_systems(Update, spawn_citizen);
        app.add_systems(Update, print_time.run_if(on_timer(Duration::from_secs(5))));
        app.insert_resource(Time::<Virtual>::from_max_delta(Duration::from_secs(10)));
    }
}
