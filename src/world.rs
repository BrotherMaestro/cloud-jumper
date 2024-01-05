use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::ecs::component::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use compute::distributions::Distribution;
use compute::prelude::Normal;

// Create a background camera component
#[derive(Component)]
pub struct WorldCamera;

#[derive(Component)]
pub struct Ground;

// COLOURS FOR PLATFORMING WORLD
const SKY_BLUE: Color = Color::rgb(0.53, 0.81, 0.92);
const DARK_GRAY: Color = Color::rgb(0.14, 0.14, 0.14);

// PLATFORM SPAWN CONSTANTS
const MIN_PLATFORM_DISTANCE: f64 = 5.0;
const MAX_PLATFORM_DISTANCE: f64 = 250.0;

pub fn setup(mut commands: Commands, window: Query<&Window, With<PrimaryWindow>>) {
    let window = window.get_single().unwrap();
    let width = window.width();
    let height = window.height();

    commands.spawn((
        Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(DARK_GRAY),
            },
            ..default()
        },
        WorldCamera,
    ));

    let sky_size = 3.0 * width;
    let ground_width = 120000.0;
    let ground_height = 60000.0;

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: SKY_BLUE,
            custom_size: Some(Vec2::new(sky_size, sky_size)),
            ..default()
        },
        ..default()
    });

    // Spawn Ground after Sky (render above)
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::YELLOW_GREEN,
                custom_size: Some(Vec2::new(ground_width, ground_height)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(
                0.0,
                (-ground_height / 2.0) - (height / 4.0),
                0.0,
            )),
            ..default()
        },
        Ground,
    ));
}

pub fn scroll_ground(time: Res<Time>, mut q_ground: Query<&mut Transform, With<Ground>>) {
    for mut ground in q_ground.iter_mut() {
        ground.translation.y -= 10.0 * time.delta_seconds();
    }
}

pub fn despawn_ground(
    mut commands: Commands,
    q_ground: Query<(Entity, &Sprite, &Transform), With<Ground>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<WorldCamera>>,
) {
    let (camera, camera_transform) = q_camera.single();
    for (ground_id, ground_sprite, ground_transform) in q_ground.iter() {
        let mut ground_y_offset = 0.0;
        if let Some(size) = ground_sprite.custom_size {
            ground_y_offset = size.y / 2.0;
        }
        let ground_top_translation =
            ground_transform.translation + Vec3::new(0.0, ground_y_offset, 0.0);

        if let Some(ndc) = camera.world_to_ndc(camera_transform, ground_top_translation) {
            if ndc.y < -1.0 {
                commands.entity(ground_id).despawn();
            }
        }
    }
}

pub fn spawn_ground(mut commands: Commands, q_ground: Query<(&Sprite, &Transform), With<Ground>>) {
    // Use a resource to store previous spawn coord - for now just calculate
    // Maybe use a list to perform multiple spawns in single pass (until saturated)
    let normal = Normal::new(50.0, 10.0);
    let normal_sample = normal
        .sample()
        .clamp(MIN_PLATFORM_DISTANCE, MAX_PLATFORM_DISTANCE);

    // Use iterators to find closest distance to y spawn point
    // Spawn same height is allowed
}
