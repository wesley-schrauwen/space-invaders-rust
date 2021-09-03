use bevy::app::{Plugin, AppBuilder};
use bevy::ecs::prelude::*;
use crate::{GameWindowSize, ActiveEnemies, Speed, MaterialManifest, ENGINE_POLL_RATE};
use bevy::prelude::*;
use crate::laser::Laser;
use bevy::math::{Vec3};
use bevy::core::FixedTimestep;
use rand::{Rng};
use std::collections::btree_map::Range;

pub struct EnemyPlugin;
pub struct Enemy;

// X and Y scales
pub struct EnemyScale {
    pub width: f32,
    pub height: f32
}

pub struct EnemyDimensions {
    pub width: f32,
    pub height: f32
}

pub struct EnemyMovement {
    x: f32,
    y: f32
}

impl EnemyDimensions {
    pub fn to_vec(&self) -> Vec2 {
        return Vec2::new(self.width.clone(), self.height.clone());
    }
}

pub const ENEMY_RAW_HEIGHT: f32 = 64.0;
pub const ENEMY_RAW_WIDTH: f32 = 64.0;

pub const ENEMY_RENDER_HEIGHT: f32 = 32.0;
pub const ENEMY_RENDER_WIDTH: f32 = 32.0;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::new().with_run_criteria(
                FixedTimestep::step(2.5)
            ).with_system(
                spawn_enemy.system()
            )
        )
            .add_system_set(SystemSet::new().with_run_criteria(FixedTimestep::step(0.5)).with_system(set_enemy_movement.system()))
            .add_system(move_enemy.system())
            .insert_resource(EnemyScale {
            width: ENEMY_RENDER_WIDTH / ENEMY_RAW_WIDTH,
            height: ENEMY_RENDER_HEIGHT / ENEMY_RAW_HEIGHT
        }).insert_resource(EnemyDimensions {
            width: ENEMY_RENDER_WIDTH,
            height: ENEMY_RENDER_HEIGHT
        });
    }
}

fn set_enemy_movement(
    mut commands: Commands,
    mut query: Query<(&mut EnemyMovement, With<Enemy>)>
) {

    let mut rng = rand::thread_rng();

    for (mut movement, _) in query.iter_mut() {
        movement.x = rng.gen_range(-2.5 .. 2.5);
        movement.y = rng.gen_range(-2.5 .. 2.5);
    }
}

fn move_enemy(
    mut commands: Commands,
    game_window_size: Res<GameWindowSize>,
    enemy_dimensions: Res<EnemyDimensions>,
    mut query: Query<(Entity, &mut Transform, &Speed, &mut EnemyMovement, With<Enemy>)>
) {

    let mut thread_rng = rand::thread_rng();

    for (entity, mut transform, speed, mut enemy_movement,  _) in query.iter_mut() {

        let x_translation = transform.translation.x + speed.0 * ENGINE_POLL_RATE * enemy_movement.x;
        let y_translation = transform.translation.y + speed.0 * ENGINE_POLL_RATE * enemy_movement.y;

        if x_translation.abs() < game_window_size.width.clone() / 2.0 {
            transform.translation.x = x_translation;
        } else {
            enemy_movement.x = thread_rng.gen_range(-5.0 .. 5.0);
        }

        if y_translation.abs() < game_window_size.height.clone() / 2.0 {
            transform.translation.y = y_translation;
        } else {
            enemy_movement.y = thread_rng.gen_range(-5.0 .. 5.0);
        }

    }
}

fn spawn_enemy(
    mut commands: Commands,
    material_manifest: Res<MaterialManifest>,
    game_window_size: Res<GameWindowSize>,
    enemy_scale: Res<EnemyScale>,
    enemy_dimensions: Res<EnemyDimensions>,
    mut active_enemies: ResMut<ActiveEnemies>
) {

    if active_enemies.0 >= 5 {
        return;
    }

    active_enemies.0 += 1;

    let mut rng = rand::thread_rng();

    let width_padding: f32 = enemy_dimensions.width.clone() / 2.0;
    let height_padding: f32 = enemy_dimensions.height.clone() / 2.0 ;

    commands.spawn_bundle(SpriteBundle {
        material: material_manifest.enemy_material.clone(),
        transform: Transform {
            translation: Vec3::new(
                rng.gen_range((-game_window_size.width.clone() / 2.0 + &width_padding) .. (game_window_size.width.clone() / 2.0 - &width_padding)),
                rng.gen_range( 0.0 .. (game_window_size.height.clone() / 2.0 - &height_padding)),
                0.0
            ),
            scale: Vec3::new(enemy_scale.width.clone(), enemy_scale.height.clone(), 0.5),
            ..Default::default()
        },
        ..Default::default()
    })
        .insert(Enemy)
        .insert(Speed(200.0))
        .insert(EnemyMovement {
            x: rng.gen_range(-1.0 .. 1.0),
            y: rng.gen_range(-1.0 .. 1.0)
        })
        .insert(EnemyDimensions {
            width: ENEMY_RENDER_WIDTH,
            height: ENEMY_RENDER_HEIGHT
        });

}
