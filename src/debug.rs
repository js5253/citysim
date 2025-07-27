use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPlugin, EguiPrimaryContextPass, egui};

use crate::{citizens::{Citizen, CitizenSpawnRequested, Happiness}, GameState};
pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(EguiPrimaryContextPass, create_ui);
    }
}

fn create_ui(mut contexts: EguiContexts,
    mut state: ResMut<GameState>,
    citizens: Query<&Citizen>,
    happiness: Query<&Happiness>,
    mut citizen_writer: EventWriter<CitizenSpawnRequested>
) -> Result {
        egui::Window::new("Debug").show(contexts.ctx_mut()?, |ui| {
            ui.horizontal(|ui| {
            ui.label(format!("Happiness: {}", happiness.iter())); /// TODO: add happiness meter here :/
            ui.label(format!("Citizens: {}", citizens.iter().len()));

            });
            if ui.button("Add School").clicked() {

            }
            if ui.button("Add Workplace").clicked() {

            }
            if ui.button("Add Citizen").clicked() {
                citizen_writer.write(CitizenSpawnRequested);
            }
         });
        Ok(())
}