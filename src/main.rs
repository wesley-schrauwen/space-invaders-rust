#![allow(unused)]

use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy::ecs::system::EntityCommands;
use bevy::sprite::QUAD_HANDLE;
use bevy::utils::Uuid;
use bevy::asset::{HandleId, AssetPathId};

const GAME_WINDOW: (i32, i32) = (800, 600);
const PLAYER_SPRITE: &str = "player_1.png";

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(WindowDescriptor {
            width: 800.0,
            height: 600.0,
            title: "Space Invaders Rust".to_string(),
            vsync: false,
            resizable: false,
            decorations: false,
            cursor_visible: false,
            cursor_locked: false,
            mode: WindowMode::Windowed,
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(startup.system())
        .run();
}

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut assets: ResMut<Assets<ColorMaterial>>,
    mut windows: ResMut<Windows>,
) {
    // camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    // reposition window to center
    windows
        .get_primary_mut()?
        .set_position(IVec2::new(GAME_WINDOW.0, GAME_WINDOW.1));

    // player
    commands.spawn_bundle(SpriteBundle {
        material: assets.add(asset_server.load(PLAYER_SPRITE).into()),
        ..DefaultPlugins::default()
    })
}
