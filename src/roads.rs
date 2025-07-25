use bevy::prelude::*;

pub struct RoadPlugin;

fn setup() {}

impl Plugin for RoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, check_for_road);
    }
}
