#![allow(unused)]

use bevy::prelude::*;

// 64x64
const PLAYER_SPRITE: &str = "player_1.png";

struct ASSETS {
}

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(WindowDescriptor {
            width: 800.0,
            height: 600.0,
            title: "Space Invaders Rust".to_string(),
            ..Default::default()
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
    let mut window = windows.get_primary_mut().unwrap();

    let window_height = window.height();
    let window_width = window.width();

    window.set_position(IVec2::new(0, 0));

    // player
    // point of origin is the center of the screen not top left
    commands.spawn_bundle(SpriteBundle {
        material: assets.add(asset_server.load(PLAYER_SPRITE).into()),
        transform: Transform {
            // 32.0 is half the size of the raw sprite
            // 5.0 for padding purposes
            translation: Vec3::new(0.0, -window_height / 2.0 + 32.0 + 5.0, 0.0),
            ..Default::default()
        },
        ..Default::default()
    });
}
