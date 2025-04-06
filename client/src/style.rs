use bevy::{color::Color, ecs::system::{Commands, Resource}};


#[derive(Resource, Debug)]
pub struct Theme {
    pub canvas: Color
}

pub fn light_mode() -> Theme {
    Theme { canvas: Color::WHITE }
}

pub fn dark_mode() -> Theme {
    Theme { canvas: Color::BLACK }
}

pub fn setup_theme(mut commands: Commands) {
    // TODO: read theme settings from persistent state and set it!
    let theme = light_mode();
    commands.insert_resource(theme);
}