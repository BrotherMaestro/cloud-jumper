// Hayden Sip 2024
use bevy::{prelude::*, sprite::Material2dPlugin};
use bevy_hanabi::HanabiPlugin;
use cloud_material::CloudMaterial;
use world::WorldPlugin;

mod cloud_material;
mod hanabi_effects;
mod region;
mod region_set;
mod world;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            Material2dPlugin::<CloudMaterial>::default(),
            HanabiPlugin,
            WorldPlugin,
        ))
        .add_systems(Startup, hanabi_effects::setup)
        .run();
}
