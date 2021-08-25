use bevy::app::{Plugin, AppBuilder};
use bevy::ecs::prelude::*;
use crate::{GameWindowSize, ActiveEnemies, Speed, MaterialManifest};
use bevy::prelude::*;
use crate::laser::Laser;
use bevy::math::{Vec3};
use bevy::core::FixedTimestep;
use rand::{Rng};
use std::collections::btree_map::Range;

pub struct EnemyPlugin;
pub struct Enemy;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::new().with_run_criteria(
                FixedTimestep::step(1.0)
            ).with_system(
                spawn_enemy.system()
            )
        );
    }
}

fn spawn_enemy(
    mut commands: Commands,
    material_manifest: Res<MaterialManifest>,
    game_window_size: Res<GameWindowSize>,
    mut active_enemies: ResMut<ActiveEnemies>
) {

    active_enemies.0 += 1;

    let mut rng = rand::thread_rng();

    commands.spawn_bundle(SpriteBundle {
        material: material_manifest.enemy_material.clone(),
        transform: Transform {
            translation: Vec3::new(
                rng.gen_range((-game_window_size.width.clone() / 2.0 + 64.0) .. (game_window_size.width.clone() / 2.0 - 64.0)),
                rng.gen_range( 0.0 .. (game_window_size.height.clone() / 2.0 - 64.0)),
                0.0
            ),
            scale: Vec3::new(0.5, 0.5, 0.5),
            ..Default::default()
        },
        ..Default::default()
    })
        .insert(Enemy)
        .insert(Speed)
        .insert(Laser);

}
