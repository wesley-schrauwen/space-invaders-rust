use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use crate::{ActiveEnemies, MaterialManifest};
use crate::laser::Laser;
use crate::enemy::{EnemyDimensions, Enemy};
use bevy::math::Vec3Swizzles;

pub struct CollisionPlugin;

struct Explosion;
struct ExplosionPosition(Vec3);

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(enemy_laser_collision.system())
            .add_system(spawn_explosion.system())
            .add_system(animate_explosion.system());
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

            let collision = collide(
                l_transform.translation,
                // the 32.0 is just me being lazy. It should be held as a dimensional struct similar to the enemy struct
                l_transform.scale.xy() * 32.0,
                e_transform.translation,
                enemy_dimensions.to_vec()
            );

            if collision.is_some() {
                despawn_entities.push(laser_entity);

                if !despawn_entities.contains(&enemy_entity) {
                    active_enemies.0 -= 1;
                    despawn_entities.push(enemy_entity);

                    commands.spawn()
                        .insert(ExplosionPosition(e_transform.translation.clone()));
                }
            }

        }
    }

    for entity in despawn_entities.iter() {
        commands.entity(*entity).despawn();
    }
}

fn spawn_explosion(
    mut commands: Commands,
    query: Query<(Entity, &ExplosionPosition)>,
    materials: Res<MaterialManifest>
) {
    for (explosion_entity, position) in query.iter() {
        commands.spawn_bundle(SpriteSheetBundle {
            texture_atlas: materials.explosion.clone(),
            transform: Transform {
                translation: position.0,
                scale: Vec3::new(0.4, 0.4 ,0.0),
                ..Default::default()
            },
            ..Default::default()
        })
            .insert(Explosion)
            .insert(Timer::from_seconds(0.05, true));

        commands.entity(explosion_entity).despawn();
    }
}

fn animate_explosion(
    mut commands: Commands,
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(Entity, &mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>, With<Explosion>)>
) {
    for (entity, mut timer, mut texture_atlas_sprite, texture_atlas_handle, _) in query.iter_mut() {
        timer.tick(time.delta());

        if timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            texture_atlas_sprite.index += 1;

            if texture_atlas_sprite.index == texture_atlas.textures.len() as u32 {
                commands.entity(entity).despawn();
            }
        }
    }
}
