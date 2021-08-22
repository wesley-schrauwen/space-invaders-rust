#![allow(unused)]

use bevy::prelude::*;

const GAME_WINDOW: (i32, i32) = (800, 600);
const PLAYER_SPRITE: &str = "player_1.png";

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

    let window_height = window.height() as i32;
    let window_width = window.width() as i32;

    window.set_position(
            IVec2::new(
                (window_height / 2) - (GAME_WINDOW.0 / 2),
                (window_width / 2) - (GAME_WINDOW.1 / 2)
            )
        );

    // player
    commands.spawn_bundle(SpriteBundle {
        material: assets.add(asset_server.load(PLAYER_SPRITE).into()),
        // transform: Transform {
        //     translation: Vec3::new(),
        //     rotation: Default::default(),
        //     scale: Default::default()
        // },
        ..Default::default()
    });
}
