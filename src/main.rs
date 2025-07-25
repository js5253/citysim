use bevy::{math::bool, prelude::*, text::TextPlugin};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, prelude::*, quick::WorldInspectorPlugin};

mod camera;
mod world;
mod brush;
mod building;
mod citizens;
mod asset_loader;
mod ui;
use crate::{asset_loader::AssetLoaderPlugin, brush::{BrushChanged, BrushPlugin, BrushType}, building::BuildingPlugin, camera::CameraPlugin, citizens::PopulationPlugin, ui::UiPlugin, world::WorldPlugin};

#[derive(Resource)]
pub struct GameState {
    brush_type: BrushType,
}

fn main() {
    App::new()
        .insert_resource(GameState {
            brush_type: BrushType::Building
        })
        .add_event::<BrushChanged>()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(PopulationPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(BrushPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(BuildingPlugin)
        .add_plugins(UiPlugin)
        .add_plugins(AssetLoaderPlugin)
        .run();
}
