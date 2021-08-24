#![allow(unused)]

use bevy::prelude::*;
use bevy::input::keyboard::KeyboardInput;

// 64x64
const PLAYER_SPRITE: &str = "player_1.png";
const PLAYER_LASER: &str = "player_laser_1.png";
// width * height
const WINDOW_SIZE: (f32, f32) = (800.0, 600.0);
// one tick every second
const ENGINE_POLL_RATE: f32 = 1.0 / 60.0;

struct MaterialManifest {
    player_material: Handle<ColorMaterial>,
    player_laser: Handle<ColorMaterial>
}

struct GameWindowSize {
    width: f32,
    height: f32
}

struct Player;
struct Laser;
struct PlayerCanFire(bool);
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
        .add_plugins(DefaultPlugins)
        .add_startup_system(startup.system())
        .add_startup_stage(
            "actors",
            SystemStage::single(spawn_player.system())
        )
        .add_system(player_movement.system())
        .add_system(player_shoot.system())
        .add_system(laser_movement.system())
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
        player_laser: assets.add(asset_server.load(PLAYER_LASER).into())
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

fn laser_movement(
    mut commands: Commands,
    game_window_size: Res<GameWindowSize>,
    mut query: Query<(Entity, &mut Transform, &Speed, With<Laser>)>
) {
    for (laser_entity, mut transform, speed, _) in query.iter_mut() {
        let y_coords = transform.translation.y + speed.0 * ENGINE_POLL_RATE;

        if y_coords > game_window_size.height  {
            commands.entity(laser_entity).despawn();
        } else {
            transform.translation.y = y_coords;
        }
    }
}
