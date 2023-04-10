use crate::prelude::*;
use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        render_resource::{AsBindGroup, ShaderRef},
        view::RenderLayers,
    },
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
};

pub struct PostProcessingPlugin;

impl Plugin for PostProcessingPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_post_processing_textures.in_base_set(StartupSet::PostStartup))
            .add_system(match_render_to_screen_size)
            .add_system(toggle_chromatic)
            .add_system(toggle_distort)
            .add_system(activate_post_processing)
            .add_system(toggle_wavy)
            .add_system(toggle_weird)
            .add_system(disable_post_processing)
            .add_plugin(Material2dPlugin::<ChromaticAbrasionMaterial>::default())
            .add_plugin(Material2dPlugin::<SpinnyMaterial>::default())
            .add_plugin(Material2dPlugin::<BlurMaterial>::default())
            .add_plugin(Material2dPlugin::<DistortionMaterial>::default())
            .add_plugin(Material2dPlugin::<WeirdMaterial>::default());
    }
}

pub fn disable_post_processing(
    mut disable: EventReader<DisableEffectsEvent>,
    mut effects: ParamSet<(
        Query<&mut Visibility, With<Handle<ChromaticAbrasionMaterial>>>,
        Query<&mut Visibility, With<Handle<DistortionMaterial>>>,
        Query<&mut Visibility, With<Handle<SpinnyMaterial>>>,
        Query<&mut Visibility, With<Handle<WeirdMaterial>>>,
    )>,
) {
    if disable.iter().count() == 0 {
        return;
    }
    for mut visible in &mut effects.p0() {
        *visible = Visibility::Hidden;
    }
    for mut visible in &mut effects.p1() {
        *visible = Visibility::Hidden;
    }
    for mut visible in &mut effects.p2() {
        *visible = Visibility::Hidden;
    }
    for mut visible in &mut effects.p3() {
        *visible = Visibility::Hidden;
    }
}

pub fn activate_post_processing(
    fadeout: Query<&Fadeout, With<PotionFade>>,
    progression: Res<StoryProgression>,
    //mut potion_grab: EventReader<PotionPickupEvent>,
    mut effects: ParamSet<(
        Query<&mut Visibility, With<Handle<ChromaticAbrasionMaterial>>>,
        Query<&mut Visibility, With<Handle<DistortionMaterial>>>,
        Query<&mut Visibility, With<Handle<WeirdMaterial>>>,
        Query<&mut Visibility, With<Handle<SpinnyMaterial>>>,
    )>,
) {
    if let Ok(fadeout) = fadeout.get_single() {
        if !fadeout.fade_in_just_finished {
            return;
        }
        match progression.current_map {
            0 => {
                for mut visible in &mut effects.p0() {
                    if *visible == Visibility::Hidden {
                        *visible = Visibility::Visible;
                    } else {
                        //*visible = Visibility::Hidden;
                    }
                }
            }
            1 => {
                for mut visible in &mut effects.p1() {
                    if *visible == Visibility::Hidden {
                        *visible = Visibility::Visible;
                    } else {
                        //*visible = Visibility::Hidden;
                    }
                }
            }
            2 => {
                for mut visible in &mut effects.p2() {
                    if *visible == Visibility::Hidden {
                        *visible = Visibility::Visible;
                    } else {
                        //*visible = Visibility::Hidden;
                    }
                }
            }
            3 => {
                for mut visible in &mut effects.p3() {
                    if *visible == Visibility::Hidden {
                        *visible = Visibility::Visible;
                    } else {
                        //*visible = Visibility::Hidden;
                    }
                }
            }
            _ => unreachable!("Bad potion pickup"),
        }
    }
}

fn match_render_to_screen_size(
    mut texture: Query<&mut Transform, With<PostProcessingQuad>>,
    windows: Query<&Window>,
) {
    let window = windows.single();
    for mut texture in &mut texture {
        texture.scale.x = window.resolution.width() / WIDTH;
        texture.scale.y = window.resolution.height() / HEIGHT;
    }
}

fn toggle_chromatic(
    mut texture: Query<&mut Visibility, With<Handle<ChromaticAbrasionMaterial>>>,
    keyboard: Res<Input<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::P) {
        for mut visible in &mut texture {
            if *visible == Visibility::Hidden {
                *visible = Visibility::Visible;
            } else {
                *visible = Visibility::Hidden;
            }
        }
    }
}

fn toggle_distort(
    mut texture: Query<&mut Visibility, With<Handle<DistortionMaterial>>>,
    keyboard: Res<Input<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::O) {
        for mut visible in &mut texture {
            if *visible == Visibility::Hidden {
                *visible = Visibility::Visible;
            } else {
                *visible = Visibility::Hidden;
            }
        }
    }
}

fn toggle_wavy(
    mut texture: Query<&mut Visibility, With<Handle<SpinnyMaterial>>>,
    keyboard: Res<Input<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::I) {
        for mut visible in &mut texture {
            if *visible == Visibility::Hidden {
                *visible = Visibility::Visible;
            } else {
                *visible = Visibility::Hidden;
            }
        }
    }
}

fn toggle_weird(
    mut texture: Query<&mut Visibility, With<Handle<WeirdMaterial>>>,
    keyboard: Res<Input<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::U) {
        for mut visible in &mut texture {
            if *visible == Visibility::Hidden {
                *visible = Visibility::Visible;
            } else {
                *visible = Visibility::Hidden;
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn spawn_post_processing_textures(
    mut commands: Commands,
    assets: Res<AssetServer>,
    image: Res<MainRender>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut chromatic_materials: ResMut<Assets<ChromaticAbrasionMaterial>>,
    mut distort_materials: ResMut<Assets<DistortionMaterial>>,
    mut wavy_materials: ResMut<Assets<SpinnyMaterial>>,
    mut weird_materials: ResMut<Assets<WeirdMaterial>>,
    mut blur_materials: ResMut<Assets<BlurMaterial>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let post_processing_pass_layer = RenderLayers::layer((RenderLayers::TOTAL_LAYERS - 1) as u8);

    let quad_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(WIDTH, HEIGHT))));

    let image_handle = image.0.clone();

    let chromatic_handle = chromatic_materials.add(ChromaticAbrasionMaterial {
        source_image: image_handle.clone(),
    });

    let blur_handle = blur_materials.add(BlurMaterial {
        source_image: image_handle.clone(),
    });

    let wavy_handle = wavy_materials.add(SpinnyMaterial {
        source_image: image_handle.clone(),
    });

    let distort_handle = distort_materials.add(DistortionMaterial {
        source_image: image_handle.clone(),
        distortion_image: assets.load("distortion.png"),
    });

    let weird_handle = weird_materials.add(WeirdMaterial {
        source_image: image_handle.clone(),
        distortion_image: assets.load("weird.png"),
    });

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: quad_handle.clone().into(),
            material: chromatic_handle,
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 1.5),
                ..default()
            },
            visibility: Visibility::Visible,
            ..default()
        },
        PostProcessingQuad,
        post_processing_pass_layer,
        Name::new("Post Processing CA"),
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: quad_handle.clone().into(),
            material: blur_handle,
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 100.5),
                ..default()
            },
            visibility: Visibility::Hidden,
            ..default()
        },
        PostProcessingQuad,
        post_processing_pass_layer,
        Name::new("Post Processing Blur"),
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: quad_handle.clone().into(),
            material: wavy_handle,
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 5.5),
                ..default()
            },
            visibility: Visibility::Hidden,
            ..default()
        },
        PostProcessingQuad,
        post_processing_pass_layer,
        Name::new("Post Processing Wavy"),
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: quad_handle.clone().into(),
            material: distort_handle,
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 1.5),
                ..default()
            },
            visibility: Visibility::Hidden,
            ..default()
        },
        PostProcessingQuad,
        post_processing_pass_layer,
        Name::new("Post Processing Distort"),
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: quad_handle.clone().into(),
            material: weird_handle,
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 7.5),
                ..default()
            },
            visibility: Visibility::Hidden,
            ..default()
        },
        PostProcessingQuad,
        post_processing_pass_layer,
        Name::new("Post Processing Weird"),
    ));

    let material_handle = materials.add(ColorMaterial {
        texture: Some(image_handle),
        ..default()
    });

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: quad_handle.into(),
            material: material_handle,
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..default()
            },
            ..default()
        },
        PostProcessingQuad,
        post_processing_pass_layer,
        Name::new("Base Render"),
    ));
}

#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "bc2f08eb-a0fb-43f1-a908-54871ea597d5"]
pub struct ChromaticAbrasionMaterial {
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
#[uuid = "4913fbab-a0fb-43f1-a908-54871ea19243"]
pub struct BlurMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub source_image: Handle<Image>,
}

impl Material2d for BlurMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/blur.wgsl".into()
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
}

impl Material2d for DistortionMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/distort.wgsl".into()
    }
}

#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "912f08eb-a4ab-3331-1b08-54871ea59295"]
pub struct WeirdMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub source_image: Handle<Image>,
    #[texture(2)]
    #[sampler(3)]
    pub distortion_image: Handle<Image>,
}

impl Material2d for WeirdMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/weird.wgsl".into()
    }
}

#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "129ea159-a4fb-93a1-ab08-54871ea91252"]
pub struct SpinnyMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub source_image: Handle<Image>,
}

impl Material2d for SpinnyMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/wavy.wgsl".into()
    }
}
