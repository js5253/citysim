use bevy::{
    input::{keyboard::KeyboardInput, mouse::MouseWheel},
    prelude::*,
};

pub struct CameraPlugin;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        // commands.spawn(Camera2d {}),
        // Camera2d::default(),
        Transform::from_xyz(15.0, 5.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
fn handle_camera_movement(
    mut camera_transform: Query<&mut Transform, With<Camera3d>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    windows: Query<&Window>,
) {
    let mut camera_transform = camera_transform.single_mut().unwrap();
    //TODO scale by delta time
    let camera_speed = 0.25;
    // camera_transform.translation.x += if (keyboard.pressed(KeyCode::ArrowUp)): camera_speed else 0.;
    camera_transform.translation.x += if keyboard.pressed(KeyCode::ArrowUp) {
        -camera_speed
    } else {
        0.
    };
    camera_transform.translation.x += if keyboard.pressed(KeyCode::ArrowDown) {
        camera_speed
    } else {
        0.
    };
    camera_transform.translation.z += if keyboard.pressed(KeyCode::ArrowRight) {
        -camera_speed
    } else {
        0.
    };
    camera_transform.translation.z += if keyboard.pressed(KeyCode::ArrowLeft) {
        camera_speed
    } else {
        0.
    };
    let max_translation = 40. * Vec3::ONE;
    camera_transform.translation = (camera_transform.translation + 0.5 * max_translation)
        .clamp(Vec3::ZERO, max_translation)
        - 0.5 * max_translation;

    // for mouse_wheel_event in mouse_wheel_events.read() {
    //     let cursor_position = windows.single().unwrap().cursor_position();

    //     if let Some(mut pos) = cursor_position {
    //         // brush_position.translation = Vec3::new(pos.x, pos.y, 0.);
    //         if mouse_wheel_event.y.is_sign_negative() {
    //         camera_transform.translation.x += (camera_transform.translation.x - pos.x);
    //         camera_transform.translation.y += (camera_transform.translation.y - pos.y);

    //         } else {
    //         camera_transform.translation.x -= (camera_transform.translation.x / pos.x) * 10.0;
    //         camera_transform.translation.y -= (camera_transform.translation.y / pos.y) * 10.0;

    //         }
    //     }

    // }
}
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
        app.add_systems(Update, handle_camera_movement);
    }
}
