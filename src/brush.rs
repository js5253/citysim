use crate::{asset_loader::SceneAssets, building::Building, roads::{Road, RoadGrid}, ui::GridMarker, world::Ground, GameState};
use bevy::{input::mouse::MouseButtonInput, picking::window, prelude::*};
#[derive(Clone, PartialEq)]
pub enum BrushType {
    Road,
    Building,
    Zone,
}

#[derive(Event)]
pub struct BrushChanged(pub BrushType);

pub struct BrushPlugin;
#[derive(Component)]
pub struct Brush;

fn spawn_brush(mut commands: Commands) {
    commands.spawn((Brush, Transform::default()));
}
fn on_brush_change(mut reader: EventReader<BrushChanged>, mut state: ResMut<GameState>) {
    for event in reader.read() {
        state.brush_type = event.0.clone();
    }
}
fn handle_brush_press(
    mouse: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
    scene_assets: Res<SceneAssets>,
    windows: Query<&Window>,
    ground: Single<&GlobalTransform, With<Ground>>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
    state: Res<GameState>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let Ok(window) = windows.single() else {
        return;
    };
    let Some(cursor_position) = window.cursor_position() else {
        return;
    };
    let (camera, camera_transform) = *camera_query;

    // Calculate a ray pointing from the camera into the world based on the cursor's position.
    let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    // Calculate if and where the ray is hitting the ground plane.
    let Some(distance) =
        ray.intersect_plane(ground.translation(), InfinitePlane3d::new(ground.up()))
    else {
        return;
    };
    let point = ray.get_point(distance);
    if mouse.just_pressed(MouseButton::Left) {
        println!("Spawning at: {}", cursor_position);
        match state.brush_type {
            BrushType::Road => {
                commands.spawn((
                    SceneRoot(scene_assets.road.clone()),
                    Road,
                    Transform {
                        translation: point,
                        ..default()
                    },
                    RoadGrid {
                        mesh: Mesh2d(meshes.add(Rectangle::new(250., 250.))),
                        material: MeshMaterial2d(materials.add(Color::linear_rgb(0.5, 0.5, 0.2))),
                        marker: GridMarker
                    }
                ));
            }
            BrushType::Building => {
                commands.spawn((
                    SceneRoot(scene_assets.building_one.clone()),
                    Building,
                    Transform {
                        translation: point,
                        ..default()
                    },
                ));
            }
            _ => {}
        }
    }
}
fn move_brush(
    camera_query: Single<(&Camera, &GlobalTransform)>,
    ground: Single<&GlobalTransform, With<Ground>>,
    windows: Query<&Window>,
    mut gizmos: Gizmos,
) {
    let Ok(windows) = windows.single() else {
        return;
    };

    let (camera, camera_transform) = *camera_query;

    let Some(cursor_position) = windows.cursor_position() else {
        return;
    };

    // Calculate a ray pointing from the camera into the world based on the cursor's position.
    let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    // Calculate if and where the ray is hitting the ground plane.
    let Some(distance) =
        ray.intersect_plane(ground.translation(), InfinitePlane3d::new(ground.up()))
    else {
        return;
    };
    let point = ray.get_point(distance);
    // make sure the point is actually on the world's bounds
    if point.x.abs() > 100.0 || point.z.abs() > 100.0 {
        return;
    }
    // Draw a circle just above the ground plane at that position.
    gizmos.circle(
        Isometry3d::new(
            point + ground.up() * 0.01,
            Quat::from_rotation_arc(Vec3::Z, ground.up().as_vec3()),
        ),
        0.2,
        Color::WHITE,
    );
}

impl Plugin for BrushPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_brush);
        app.add_systems(Update, (move_brush, handle_brush_press, on_brush_change));
    }
}
