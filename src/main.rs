// Hayden Sip 2024
use bevy::prelude::*;

mod region;
mod world;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, world::setup)
        .add_systems(Update, (world::scroll_ground, world::despawn_ground))
        .run();
}
