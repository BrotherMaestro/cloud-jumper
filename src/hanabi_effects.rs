use bevy::asset::{Assets, Handle};
use bevy::ecs::system::{Commands, ResMut, Resource};
use bevy::math::{Vec3, Vec4};
use bevy_hanabi::prelude::*;

#[derive(Resource, Clone)]
pub struct HanabiEffectHandles {
    pub cloud: Handle<EffectAsset>,
}

pub fn setup(mut commands: Commands, mut effects: ResMut<Assets<EffectAsset>>) {
    let mut gradient = Gradient::new();
    gradient.add_key(0., Vec4::new(1., 1., 1., 1.));
    gradient.add_key(1., Vec4::splat(0.));

    let mut module = Module::default();

    let init_pos = SetPositionSphereModifier {
        center: module.lit(Vec3::new(0., -20., 0.)),
        radius: module.lit(20.),
        dimension: bevy_hanabi::ShapeDimension::Volume,
    };

    let init_vel = SetVelocitySphereModifier {
        center: module.lit(Vec3::ZERO),
        speed: module.lit(6.),
    };

    let lifetime = module.lit(10.);
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    let accel = module.lit(Vec3::new(0., -3., 0.));
    let update_accel = AccelModifier::new(accel);

    let effect = EffectAsset::new(32768, Spawner::rate(5.0.into()), module)
        .with_name("Cloud")
        .init(init_pos)
        .init(init_vel)
        .init(init_lifetime)
        .update(update_accel)
        .render(ColorOverLifetimeModifier { gradient });

    //  effect_handle is necessary for spawning... somehow get to world.rs ?
    let cloud = effects.add(effect);
    commands.insert_resource(HanabiEffectHandles { cloud })
}
