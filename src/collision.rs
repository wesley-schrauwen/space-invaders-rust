use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use crate::ActiveEnemies;
use crate::laser::Laser;
use crate::enemy::{EnemyDimensions, Enemy};
use bevy::math::Vec3Swizzles;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(enemy_laser_collision.system());
    }
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

            // the 32.0 is just me being lazy. It should be held as a dimensional struct similar to the enemy struct
            let collision = collide(
                l_transform.translation,
                l_transform.scale.xy() * 32.0,
                e_transform.translation,
                enemy_dimensions.to_vec()
            );

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
