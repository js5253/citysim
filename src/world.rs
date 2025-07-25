use bevy::color::palettes::basic::PURPLE;
use bevy::prelude::*;
pub struct WorldPlugin;

pub const WORLD_SIZE: f32 = 100.0;

#[derive(Component)]
pub struct Ground;

fn spawn_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(WORLD_SIZE, WORLD_SIZE))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Ground,
    ));

    // light
    commands.spawn((
        Mesh2d(meshes.add(Mesh::from(Circle::new(1.0)))),
        DirectionalLight::default(),
        Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_world);
    }
}
