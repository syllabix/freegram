use bevy::prelude::*;
use camera::CameraPlugin;
use widget::WidgetPlugin;

mod camera;
mod style;
pub mod widget;

pub struct FreegramPlugin;
impl Plugin for FreegramPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Freegram | Diagram Freely".to_string(),
                fit_canvas_to_parent: true,
                prevent_default_event_handling: true,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(CameraPlugin)
        .add_plugins(WidgetPlugin)
        .add_systems(PreStartup, style::setup_theme);
    }
}
