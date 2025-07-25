use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPlugin, EguiPrimaryContextPass, egui};

use crate::{
    GameState,
    brush::{BrushChanged, BrushType},
    zoning::ZoneType,
};
pub struct UiPlugin;
fn create_ui(
    mut contexts: EguiContexts,
    mut writer: EventWriter<BrushChanged>,
    mut state: ResMut<GameState>,
) -> Result {
    egui::Window::new("Hello").show(contexts.ctx_mut()?, |ui| {
        ui.horizontal(|ui| {
            ui.radio_value(&mut state.brush_type, BrushType::Building, "Building");
            ui.radio_value(&mut state.brush_type, BrushType::Road, "Road");
            ui.radio_value(&mut state.brush_type, BrushType::Zone, "Zone");
        });
        ui.horizontal(|ui| {
            ui.radio_value(&mut state.zone_type, ZoneType::Commercial, "Commercial");
            ui.radio_value(&mut state.zone_type, ZoneType::Industrial, "Industrial");
            ui.radio_value(&mut state.zone_type, ZoneType::Residential, "Residential");
        });
        ui.checkbox(&mut state.grid_enabled, "Grid");
    });
    // egui::Window::new("Serialization").show(contexts.ctx_mut()?, |ui| {
    //     ui.horizontal(|ui| {
    //         ui.button("Save");
    //         ui.button("Load");
    //     });
    // });

    Ok(())
}
fn create_vfx(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shape = Circle::new(50.0);
    commands.spawn((
        Mesh2d(meshes.add(shape)),
        MeshMaterial2d(materials.add(Color::linear_rgb(210., 55., 66.))),
        Transform::from_xyz(50.0, 50.0, 50.0),
    ));
    println!("Spawned something that should appear in 2d... idk")
}
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(EguiPrimaryContextPass, create_ui);
        app.add_systems(Startup, create_vfx);
    }
}
