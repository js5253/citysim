use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPlugin, EguiPrimaryContextPass, egui};

use crate::{citizens::Citizen, GameState};
pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(EguiPrimaryContextPass, create_ui);
    }
}

fn create_ui(    mut contexts: EguiContexts,
    mut state: ResMut<GameState>,
    mut citizens: Query<&Citizen>
) -> Result {
        egui::Window::new("Debug").show(contexts.ctx_mut()?, |ui| {
            ui.label(format!("Citizens: {}", citizens.iter().len()));
            if ui.button("Add School").clicked() {

            }
            if ui.button("Add Workplace").clicked() {

            }
         });
        Ok(())
}