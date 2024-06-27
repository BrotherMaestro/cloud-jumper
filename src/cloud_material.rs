use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CloudMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub blue_noise: Option<Handle<Image>>,
}

impl Material2d for CloudMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/cloud_material.wgsl".into()
    }
}
