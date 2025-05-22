use bevy::ecs::component::*;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::PrimaryWindow;
use compute::distributions::Distribution;
use compute::prelude::Normal;
use rand::Rng;

use crate::cloud_material::CloudMaterial;
use crate::region::Region;
use crate::region_set::RegionSet;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (scroll_camera, despawn_platform, spawn_platform));
    }
}

// Create a background camera component
#[derive(Component)]
pub struct WorldCamera;

#[derive(Component)]
pub struct Ground;

// COLOURS FOR PLATFORMING WORLD
const SKY_BLUE: Color = Color::rgb(0.53, 0.81, 0.92);
const DARK_GRAY: Color = Color::rgb(0.14, 0.14, 0.14);

// PLATFORM SPAWN CONSTANTS
const MIN_PLATFORM_DISTANCE: f64 = 250.0;
const MAX_PLATFORM_DISTANCE: f64 = 2500.0;

// MOVEMENT CONSTANTS
const CAMERA_SCROLL_SPEED: f32 = 50.0;

pub fn setup(mut commands: Commands, window: Query<&Window, With<PrimaryWindow>>) {
    let window = window.get_single().unwrap();
    let width = window.width();
    let height = window.height();
    let sky_size = 1.2 * Vec2::new(width, height);

    let sky_bundle = SpriteBundle {
        sprite: Sprite {
            color: SKY_BLUE,
            custom_size: Some(sky_size),
            ..default()
        },
        ..default()
    };

    commands
        .spawn((
            Camera2dBundle {
                camera: Camera {
                    clear_color: ClearColorConfig::Custom(DARK_GRAY),
                    ..default()
                },
                ..default()
            },
            WorldCamera,
        ))
        .with_children(|parent| {
            parent.spawn(sky_bundle);
        });

    let ground_width = 120000.0;
    let ground_height = 60000.0;

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
                1.0,
            )),
            ..default()
        },
        Ground,
    ));
}

pub fn scroll_camera(
    time: Res<Time>,
    mut q_camera: Query<(&Camera, &mut Transform), With<WorldCamera>>,
) {
    for (_, mut transform) in &mut q_camera {
        let local_y = transform.local_y();
        transform.translation += local_y * CAMERA_SCROLL_SPEED * time.delta_seconds();
    }
}

pub fn despawn_platform(
    mut commands: Commands,
    q_ground: Query<(Entity, &Sprite, &Transform), With<Ground>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<WorldCamera>>,
) {
    let (camera, camera_transform) = q_camera.single();
    for (ground_id, ground_sprite, ground_transform) in q_ground.iter() {
        let ground_y_offset = if let Some(size) = ground_sprite.custom_size {
            size.y / 2.0
        } else {
            0.0
        };
        let ground_top_translation =
            ground_transform.translation + Vec3::new(0.0, ground_y_offset, 0.0);

        if let Some(ndc) = camera.world_to_ndc(camera_transform, ground_top_translation) {
            if ndc.y < -1.0 {
                commands.entity(ground_id).despawn();
            }
        }
    }
}

pub fn spawn_platform(
    mut commands: Commands,
    mut materials: ResMut<Assets<CloudMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    q_window: Query<&Window>,
    q_ground: Query<&GlobalTransform, With<Ground>>,
    q_camera: Query<&GlobalTransform, With<WorldCamera>>,
) {
    // Use a normal distrubution to create natural variance in the spawn distances,
    // while still maintaining some consistency in difficulty
    let normal = Normal::new(600.0, 100.0);
    let platform_distance = normal
        .sample()
        .clamp(MIN_PLATFORM_DISTANCE, MAX_PLATFORM_DISTANCE) as f32;

    // Use global transforms to ensure the positions are relative (and therefore measure valid distances)
    // Our single player game has 1 WorldCamera, and 1 window
    let window = q_window.single();
    let camera_translation = q_camera.single().translation();
    let top_of_camera = camera_translation.y + window.height() / 2.0;
    let window_half_width = window.width() / 2.0;
    let left_of_camera = camera_translation.x - window_half_width;
    let right_of_camera = camera_translation.x + window_half_width;

    let mut exclusions = vec![];

    // Iterate over existing platforms, convert into regions of exclusion at minimum height 'y'
    for ground_transform in q_ground.iter() {
        let ground_translation = ground_transform.translation();
        let y_offset = top_of_camera - ground_translation.y;

        // We can safely assume the square root will always be positive (we have two positive numbers)
        // Therefore, we can compare the children without squaring.
        // Check if there is a non-complex solution to the x_offset
        if platform_distance > y_offset {
            let platform_distance_squared = platform_distance * platform_distance;
            let y_offset_squared = y_offset * y_offset;
            let x_offset = (platform_distance_squared - y_offset_squared).sqrt();
            let lower_exclusion_boundary = ground_translation.x - x_offset;
            let upper_exclusion_boundary = ground_translation.x + x_offset;
            let exclusion_region = Region {
                lower: lower_exclusion_boundary,
                upper: upper_exclusion_boundary,
            };
            exclusions.push(exclusion_region);
        }
    }
    // Order the vec by a key (that being the lower value of each region)
    // We need to define a total ordering because we are working with floats...
    // Therefore NOTE: No NANs allowed in the vector!!!
    exclusions.sort_unstable_by(|a, b| a.lower.partial_cmp(&b.lower).unwrap());
    let region_set = RegionSet::with_sorted(left_of_camera, right_of_camera, exclusions);
    if let Some(spawn_region) = region_set.random() {
        let x_coord = rand::thread_rng().gen_range(spawn_region.lower..=spawn_region.upper);
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(Rectangle::new(200., 200.)).into(),
                transform: Transform::from_xyz(x_coord, top_of_camera, 2.),
                material: materials.add(CloudMaterial {
                    blue_noise: Some(asset_server.load("noise/blue.png")),
                    perlin_noise: Some(asset_server.load("noise/perlin.png")),
                    seed: rand::thread_rng().gen(),
                }),
                ..default()
            },
            Ground,
        ));
    }
}
