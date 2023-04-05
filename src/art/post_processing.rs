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
        app.add_plugin(Material2dPlugin::<ChromaticAbrasionMaterial>::default())
            .add_plugin(Material2dPlugin::<WavyMaterial>::default())
            .add_plugin(Material2dPlugin::<DistortionMaterial>::default());
    }
}

/// Our custom post processing material
#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "bc2f08eb-a0fb-43f1-a908-54871ea597d5"]
pub struct ChromaticAbrasionMaterial {
    /// In this example, this image will be the result of the main camera.
    #[texture(0)]
    #[sampler(1)]
    pub source_image: Handle<Image>,
}

impl Material2d for ChromaticAbrasionMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/chromatic_aberration.wgsl".into()
    }
}

#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "342f08eb-a4fb-93a1-ab08-54871ea597d5"]
pub struct DistortionMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub source_image: Handle<Image>,
    #[texture(2)]
    #[sampler(3)]
    pub distortion_image: Handle<Image>,
    #[uniform(4)]
    pub strength: f32,
}

impl Material2d for DistortionMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/distort.wgsl".into()
    }
}

#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "129ea159-a4fb-93a1-ab08-54871ea91252"]
pub struct WavyMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub source_image: Handle<Image>,
}

impl Material2d for WavyMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/wavy.wgsl".into()
    }
}
