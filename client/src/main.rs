use bevy::prelude::*;
use client::FreegramPlugin;

fn main() {
    App::new().add_plugins(FreegramPlugin).run();
}