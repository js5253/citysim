use bevy::prelude::*;

use crate::ui::GridMarker;

pub struct RoadPlugin;

// fn setup() {}#[derive(Component)]
#[derive(Component)]
pub struct Road;


#[derive(Bundle)]
pub struct RoadGrid {
    pub mesh: Mesh2d,
    pub material: MeshMaterial2d<ColorMaterial>,
    pub marker: GridMarker

}