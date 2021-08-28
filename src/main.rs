#![allow(unused)]

mod player;
mod laser;
mod enemy;
mod collision;

use bevy::prelude::*;
use bevy::input::keyboard::KeyboardInput;
use crate::player::PlayerPlugin;
use crate::laser::{LaserPlugin, Laser};
use crate::enemy::{EnemyPlugin, Enemy, EnemyDimensions};
use bevy::sprite::collide_aabb::collide;
use bevy::math::Vec3Swizzles;
use crate::collision::CollisionPlugin;

// 64x64
const PLAYER_SPRITE: &str = "player_1.png";
const PLAYER_LASER: &str = "player_laser_1.png";
const ENEMY_SPRITE: &str = "alien_1.png";
// width * height
const WINDOW_SIZE: (f32, f32) = (800.0, 600.0);
// one tick every second
const ENGINE_POLL_RATE: f32 = 1.0 / 60.0;

struct MaterialManifest {
    player_material: Handle<ColorMaterial>,
    player_laser: Handle<ColorMaterial>,
    enemy_material: Handle<ColorMaterial>
}

struct GameWindowSize {
    width: f32,
    height: f32
}

struct ActiveEnemies(u8);
struct Speed(f32);

impl Default for Speed {
    fn default() -> Self {
        Self(500.0)
    }
}

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(WindowDescriptor {
            width: WINDOW_SIZE.0,
            height: WINDOW_SIZE.1,
            title: "Space Invaders Rust".to_string(),
            ..Default::default()
        })
        .insert_resource(ActiveEnemies(0))
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(LaserPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(CollisionPlugin)
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

    commands.insert_resource(MaterialManifest {
        player_material: assets.add(asset_server.load(PLAYER_SPRITE).into()),
        player_laser: assets.add(asset_server.load(PLAYER_LASER).into()),
        enemy_material: assets.add(asset_server.load(ENEMY_SPRITE).into())
    });

    // reposition window to center
    let mut window = windows.get_primary_mut().unwrap();

    let window_height = window.height();
    let window_width = window.width();

    commands.insert_resource(GameWindowSize {
        width: WINDOW_SIZE.0,
        height: WINDOW_SIZE.1
    });


    window.set_position(IVec2::new(0, 0));
}
