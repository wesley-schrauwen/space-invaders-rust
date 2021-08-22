#![allow(unused)]

use bevy::prelude::*;
use bevy::window::WindowMode;

const PLAYER_SPRITE: &str = "player_1.png";

fn main() {

    App::build()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(WindowDescriptor {
            width: 800.0,
            height: 600.0,
            resize_constraints: Default::default(),
            scale_factor_override: None,
            title: "Space Invaders Rust".to_string(),
            vsync: false,
            resizable: false,
            decorations: false,
            cursor_visible: false,
            cursor_locked: false,
            mode: WindowMode::Windowed
        })
        .add_plugins(DefaultPlugins)
        .run();

}
