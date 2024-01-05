// Hayden Sip 2024
use bevy::prelude::*;

mod region;
mod region_set;
mod world;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, world::setup)
        .add_systems(
            Update,
            (
                world::scroll_camera,
                world::despawn_ground,
                world::spawn_ground,
            ),
        )
        .run();
}
