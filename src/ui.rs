use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPlugin, EguiPrimaryContextPass, egui};

use crate::{
    GameState,
    brush::{BrushChanged, BrushType},
    world::{Ground, WORLD_SIZE},
    zoning::ZoneType,
};
#[derive(Component)]
pub struct GridMarker;

pub struct UiPlugin;
fn create_ui(
    mut contexts: EguiContexts,
    mut writer: EventWriter<BrushChanged>,
    mut state: ResMut<GameState>,
) -> Result {
    egui::Window::new("Brush Properties").show(contexts.ctx_mut()?, |ui| {
        ui.horizontal(|ui| {
            ui.radio_value(&mut state.brush_type, BrushType::Building, "Building");
            ui.radio_value(&mut state.brush_type, BrushType::Road, "Road");
            ui.radio_value(&mut state.brush_type, BrushType::Zone, "Zone");
        });
        ui.horizontal(|ui| {
            // ui.
            ui.menu_button("Change Asset", |ui| {
        })

        });
        ui.horizontal(|ui| {
            ui.radio_value(&mut state.zone_type, ZoneType::Commercial, "Commercial");
            ui.radio_value(&mut state.zone_type, ZoneType::Industrial, "Industrial");
            ui.radio_value(&mut state.zone_type, ZoneType::Residential, "Residential");
        });
        ui.checkbox(&mut state.grid_enabled, "Grid");
    });
    Ok(())
}
fn create_grid(
    state: Res<GameState>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut items: Query<&Transform, With<GridMarker>>
    // ground: Query<&Transform, With<Ground>>
) {
    return;
    let grid_material = MeshMaterial3d(materials.add(Color::srgba(0.6, 0.6, 0.6, 0.2)));
    for x in 0..=WORLD_SIZE.floor() as i32 {
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(WORLD_SIZE, 2., 2.))),
            grid_material.clone(),
            GridMarker,
            Transform {
                translation: Vec3::new(((20 * x)) as f32, 0., 0.),
                ..default()
            },
        ));
    }
}
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(EguiPrimaryContextPass, create_ui);
        app.add_systems(Update, create_grid);
    }
}
