use bevy::prelude::*;
use fake::Faker;
use crate::{asset_loader::SceneAssets};
pub struct BuildingPlugin;

#[derive(Component)]
pub struct Building;

#[derive(Component)]
pub struct Destination;

#[derive(Component)]
pub struct Alias(String);

#[derive(Component)]
pub struct School;

#[derive(Component)]
pub struct Workplace;

#[derive(Component)]
pub struct Home;

fn spawn_buildings(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((Destination, School, Transform::default(), Alias(String::from("1024 W Byte St"))));
    commands.spawn((Destination, Workplace, Transform::default(), Alias(String::from("4412 W Roanoke Ave"))));
    commands.spawn((Destination, School, Transform::default(), Alias(String::from("6235 W Boomer Drive"))));
    commands.spawn((Destination, Transform::default(), Alias(String::from("354 W South Shore Drive"))));
    commands.spawn((Destination, Transform::default(), Alias(String::from("7451 W Byte St"))));
    commands.spawn((Destination, Transform::default(), Alias(String::from("2005 W Penguin Av")), Home));
}

impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_buildings);
        // app.add_systems(Update, add_buildings);
    }
}