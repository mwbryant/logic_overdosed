use crate::prelude::*;
use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    reflect::TypeUuid,
    render::{
        camera::RenderTarget,
        render_resource::{
            AsBindGroup, Extent3d, ShaderRef, TextureDescriptor, TextureDimension, TextureFormat,
            TextureUsages,
        },
        texture::BevyDefault,
        view::RenderLayers,
    },
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
};

pub struct PostProcessingPlugin;

impl Plugin for PostProcessingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(Material2dPlugin::<PostProcessingMaterial>::default());
    }
}

/// Our custom post processing material
#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "bc2f08eb-a0fb-43f1-a908-54871ea597d5"]
pub struct PostProcessingMaterial {
    /// In this example, this image will be the result of the main camera.
    #[texture(0)]
    #[sampler(1)]
    pub source_image: Handle<Image>,
}

impl Material2d for PostProcessingMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/chromatic_aberration.wgsl".into()
    }
}
