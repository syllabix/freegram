use bevy::prelude::*;
use bevy_pancam::{DirectionKeys, PanCam, PanCamPlugin};

#[derive(Resource, Default)]
pub struct PanningLock {
    pub is_locked: bool,
}

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PanningLock>()
            .add_plugins(PanCamPlugin)
            .add_systems(Startup, setup)
            .add_systems(Update, (toggle_zoom, update_panning_state));
    }
}

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        PanCam {
            move_keys: DirectionKeys::NONE,
            max_scale: 40.,
            min_scale: 1.,
            zoom_to_cursor: true,
            enabled: true,
            ..default()
        },
    ));
}

pub fn toggle_zoom(mut query: Query<&mut PanCam>, keys: Res<ButtonInput<KeyCode>>) {
    for mut pan_cam in query.iter_mut() {
        // Keep panning enabled but only allow zooming when SuperLeft is pressed
        if keys.pressed(KeyCode::SuperLeft) {
            // Enable both panning and zooming with normal scale limits
            pan_cam.min_scale = 1.0;
            pan_cam.max_scale = 40.0;
        } else {
            // Disable zooming by setting min and max scale to the same value
            // This effectively disables zooming while keeping panning enabled
            pan_cam.min_scale = 1.0;
            pan_cam.max_scale = 1.0;
        }
    }
}

pub fn update_panning_state(
    mut query: Query<&mut PanCam>,
    panning_lock: Res<PanningLock>,
) {
    for mut pan_cam in query.iter_mut() {
        pan_cam.enabled = !panning_lock.is_locked;
    }
}
