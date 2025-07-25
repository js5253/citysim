use bevy::{math::bool, prelude::*, sprite::Material2dPlugin, text::TextPlugin};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, prelude::*, quick::WorldInspectorPlugin};

mod asset_loader;
mod brush;
mod building;
mod camera;
mod citizens;
mod ui;
mod world;
mod zoning;
use crate::{
    asset_loader::AssetLoaderPlugin,
    brush::{BrushChanged, BrushPlugin, BrushType},
    building::BuildingPlugin,
    camera::CameraPlugin,
    citizens::PopulationPlugin,
    ui::UiPlugin,
    world::WorldPlugin,
    zoning::ZoneType,
};

#[derive(Resource)]
pub struct GameState {
    brush_type: BrushType,
    zone_type: ZoneType,
    grid_enabled: bool,
}

fn main() {
    App::new()
        .insert_resource(GameState {
            brush_type: BrushType::Building,
            zone_type: ZoneType::Residential,
            grid_enabled: false,
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
