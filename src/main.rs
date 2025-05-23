// Hayden Sip 2024
use bevy::{prelude::*, sprite::Material2dPlugin};
use cloud_material::CloudMaterial;
use world::WorldPlugin;

mod app_state;
mod cloud_material;
mod region;
mod region_set;
mod world;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            Material2dPlugin::<CloudMaterial>::default(),
            WorldPlugin,
        ))
        .run();
}
