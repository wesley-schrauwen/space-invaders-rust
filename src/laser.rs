use bevy::app::{Plugin, AppBuilder};
use bevy::ecs::*;
use crate::{GameWindowSize, Speed, ENGINE_POLL_RATE};
use bevy::ecs::prelude::{With, Query, Commands, Res, IntoSystem};
use bevy::ecs::entity::Entity;
use bevy::prelude::Transform;

pub struct LaserPlugin;

pub struct Laser;

impl Plugin for LaserPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(laser_movement.system());
    }

    fn name(&self) -> &str {
        "LaserPlugin"
    }
}

fn laser_movement(
    mut commands: Commands,
    game_window_size: Res<GameWindowSize>,
    mut query: Query<(Entity, &mut Transform, &Speed, With<Laser>)>
) {
    for (laser_entity, mut transform, speed, laser) in query.iter_mut() {

        let entity: Entity = laser_entity;

        let y_coords = transform.translation.y + speed.0 * ENGINE_POLL_RATE;

        if y_coords > game_window_size.height  {
            commands.entity(laser_entity).despawn();
        } else {
            transform.translation.y = y_coords;
        }
    }
}
