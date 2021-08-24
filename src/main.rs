#![allow(unused)]

use bevy::prelude::*;
use bevy::input::keyboard::KeyboardInput;

// 64x64
const PLAYER_SPRITE: &str = "player_1.png";
// width * height
const WINDOW_SIZE: (f32, f32) = (800.0, 600.0);
// one tick every second
const ENGINE_POLL_RATE: f32 = 1.0 / 60.0;

struct MaterialManifest {
    player_material: Handle<ColorMaterial>
}

struct GameWindowSize {
    width: f32,
    height: f32
}

struct Player;
struct PlayerSpeed(f32);
impl Default for PlayerSpeed {
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
        player_material: assets.add(asset_server.load(PLAYER_SPRITE).into())
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
    }).insert(Player).insert(PlayerSpeed::default());
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    game_window_size: Res<GameWindowSize>,
    mut query: Query<(&PlayerSpeed, &mut Transform, With<Player>)>
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
