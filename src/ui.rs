use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiPrimaryContextPass};

use crate::brush::BrushChanged;
pub struct UiPlugin;
fn create_ui(mut contexts: EguiContexts,  mut writer: EventWriter<BrushChanged>) -> Result {
    egui::Window::new("Hello").show(contexts.ctx_mut()?, |ui| {
        if ui.button("Road").clicked() {
            writer.write(BrushChanged(crate::brush::BrushType::Road));
        };
        if ui.button("Building").clicked() {
            writer.write(BrushChanged(crate::brush::BrushType::Building));
        }
    });
    Ok(())
}
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(EguiPrimaryContextPass, create_ui);
    }
}