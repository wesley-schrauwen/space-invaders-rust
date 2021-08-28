#![allow(unused)]

mod player;
mod laser;
mod enemy;

use bevy::prelude::*;
use bevy::input::keyboard::KeyboardInput;
use crate::player::PlayerPlugin;
use crate::laser::{LaserPlugin, Laser};
use crate::enemy::{EnemyPlugin, Enemy, EnemyDimensions};
use bevy::sprite::collide_aabb::collide;
use bevy::math::Vec3Swizzles;

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
        .add_system(enemy_laser_collision.system())
        .add_startup_system(startup.system())
        .run();
}

fn enemy_laser_collision(
    mut commands: Commands,
    mut active_enemies: ResMut<ActiveEnemies>,
    mut laser_query: Query<(Entity, &Transform, With<Laser>)>,
    mut enemy_query: Query<(Entity, &Transform, &EnemyDimensions, With<Enemy>)>
) {

    let mut despawn_entities: Vec<Entity> = Vec::new();

    for (laser_entity, laser_transform, _) in laser_query.iter_mut() {
        for (enemy_entity, enemy_transform, enemy_dimensions, _) in enemy_query.iter_mut() {

            let l_transform: &Transform = laser_transform;
            let e_transform: &Transform = enemy_transform;

            let collision = collide(l_transform.translation, l_transform.scale.xy() * 32.0, e_transform.translation, enemy_dimensions.to_vec());

            if collision.is_some() {
                despawn_entities.push(laser_entity);

                if !despawn_entities.contains(&enemy_entity) {
                    active_enemies.0 -= 1;
                    despawn_entities.push(enemy_entity);
                }
            }

        }
    }

    for entity in despawn_entities.iter() {
        commands.entity(*entity).despawn();
    }
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
