use bevy::prelude::*;
use bevy::input::keyboard::KeyboardInput;
use crate::{MaterialManifest, GameWindowSize, Speed, ENGINE_POLL_RATE};
use crate::laser::Laser;

struct Player;
struct PlayerCanFire(bool);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_stage(
            "actors",
            SystemStage::single(spawn_player.system())
        )
            .add_system(player_movement.system())
            .add_system(player_shoot.system());
    }

    fn name(&self) -> &str {
        "PlayerPlugin"
    }
}

fn spawn_player(
    mut commands: Commands,
    material_manifest: Res<MaterialManifest>,
    game_window_size: Res<GameWindowSize>
) {
    commands.spawn_bundle(SpriteBundle {
        material: material_manifest.player_material.clone(),
        transform: Transform {
            // point of origin is the center of the screen not top left
            // 32.0 is half the size of the raw sprite
            // 5.0 for padding purposes
            translation: Vec3::new(
                0.0,
                -game_window_size.height.clone() / 2.0 + 32.0 + 5.0,
                0.0
            ),
            ..Default::default()
        },
        ..Default::default()
    }).insert(Player).insert(Speed::default()).insert(PlayerCanFire(true));
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    game_window_size: Res<GameWindowSize>,
    mut query: Query<(&Speed, &mut Transform, With<Player>)>
) {
    // handles block if single_mut is OK, no real restriction on number of players
    if let Ok((player_speed, mut transform, player)) = query.single_mut() {
        let change = if keyboard_input.pressed(KeyCode::Left) {
            -1.0
        } else if keyboard_input.pressed(KeyCode::Right) {
            1.0
        } else {
            0.0
        };

        let x_coords = transform.translation.x + change * player_speed.0 * ENGINE_POLL_RATE;

        if x_coords.abs() < game_window_size.width.clone() / 2.0 - 32.0 {
            transform.translation.x = x_coords
        }
    }
}

fn player_shoot(
    mut commands: Commands,
    materials: Res<MaterialManifest>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut PlayerCanFire, With<Player>)>
) {
    if let Ok((mut transform, mut player_can_fire, _)) = query.single_mut() {
        if player_can_fire.0 && keyboard_input.pressed(KeyCode::Space) {

            let gun_x_offset = 20.0;
            let gun_y_coords: f32 = transform.translation.y + 6.0;

            let mut spawn_lasers = |x_offset: &f32| {
                commands.spawn_bundle(SpriteBundle {
                    material: materials.player_laser.clone(),
                    transform: Transform {
                        translation: Vec3::new(
                            transform.translation.x + x_offset,
                            gun_y_coords,
                            0.0
                        ),
                        scale: Vec3::new(0.4, 0.5, 0.5),
                        ..Default::default()
                    },
                    ..Default::default()
                }).insert(Laser).insert(Speed::default());
            };

            spawn_lasers(&gun_x_offset);
            spawn_lasers(&-gun_x_offset);

            // this is a mutable reference so when we change this it will change the correct entities condition
            player_can_fire.0 = false;
        }

        if keyboard_input.just_released(KeyCode::Space) && !player_can_fire.0 {
            player_can_fire.0 = true;
        }
    }
}
