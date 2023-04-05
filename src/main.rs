use bevy::{input::common_conditions::input_toggle_active, render::camera::ScalingMode};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use logic_overdosed::{comp_from_config, prelude::*};

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

pub const WIDTH: f32 = 640.0;
pub const HEIGHT: f32 = 480.0;
pub const RESOLUTION: f32 = WIDTH / HEIGHT;

fn main() {
    let mut app = App::new();

    app.add_state::<GameState>()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Logic Game".into(),
                        resolution: (WIDTH, HEIGHT).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_plugin(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
        )
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(50.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup)
        .add_system(update_lifetimes.in_base_set(CoreSet::PostUpdate))
        .add_startup_system(setup_camera)
        .add_system(match_render_to_screen_size)
        .add_system(toggle_chromatic)
        .add_system(toggle_distort)
        .add_plugin(PlayerPlugin)
        .add_plugin(MapPlugin)
        .add_plugin(ArtPlugin);

    app.run();
}

#[derive(Component)]
pub struct PostProcessingQuad;

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

fn setup_camera(
    mut commands: Commands,
    windows: Query<&Window>,
    assets: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut chromatic_materials: ResMut<Assets<ChromaticAbrasionMaterial>>,
    mut distort_materials: ResMut<Assets<DistortionMaterial>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    let window = windows.single();

    let size = Extent3d {
        width: window.resolution.physical_width(),
        height: window.resolution.physical_height(),
        ..default()
    };

    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::bevy_default(),
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    image.resize(size);

    let image_handle = images.add(image);

    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::FixedVertical(HEIGHT);
    camera.transform.translation.x = 320.0;
    camera.transform.translation.y = 240.0;
    camera.camera.target = RenderTarget::Image(image_handle.clone());

    // Main camera, first to render
    commands.spawn((camera, UiCameraConfig { show_ui: false }));

    let post_processing_pass_layer = RenderLayers::layer((RenderLayers::TOTAL_LAYERS - 1) as u8);

    let quad_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(WIDTH, HEIGHT))));

    let chromatic_handle = chromatic_materials.add(ChromaticAbrasionMaterial {
        source_image: image_handle.clone(),
    });

    let distort_handle = distort_materials.add(DistortionMaterial {
        source_image: image_handle.clone(),
        distortion_image: assets.load("distortion.png"),
        strength: 0.045,
    });

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: quad_handle.clone().into(),
            material: chromatic_handle,
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 1.5),
                ..default()
            },
            visibility: Visibility::Hidden,
            ..default()
        },
        PostProcessingQuad,
        post_processing_pass_layer,
        Name::new("Post Processing CA"),
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

    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                // renders after the first main camera which has default value: 0.
                order: 999,
                ..default()
            },
            ..Camera2dBundle::default()
        },
        post_processing_pass_layer,
    ));
}

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = assets.load("smoke_particles.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 1, 1, None, None);
    let particle_atlas = texture_atlases.add(texture_atlas);

    let feet_particle_emitter = spawn_new_rect_emitter(
        &mut commands,
        ParticleDesc {
            particle: Particle {
                lifetime: Timer::from_seconds(0.4, TimerMode::Once),
            },
            sprite: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    custom_size: Some(Vec2::splat(12.0)),
                    ..default()
                },
                texture_atlas: particle_atlas,
                ..default()
            },
            falling: Some(FallingParticle { speed: 12.0 }),
            radial: Some(RadialParticle {
                speed: 16.0,
                direction: Vec2::ZERO,
            }),
            rotating: Some(RotatingParticle { speed: 24.0 }),
            fading: Some(FadingParticle {}),
        },
        Vec2::new(0.0, -13.0),
        Vec2::new(7.0, 3.0),
        None,
        1,
        None,
    );

    commands
        .entity(feet_particle_emitter)
        .insert(PlayerFeetParticles);

    let texture_handle = assets.load("particles.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 1, 1, None, None);
    let particle_atlas = texture_atlases.add(texture_atlas);

    let head_particle_emitter = spawn_new_rect_emitter(
        &mut commands,
        ParticleDesc {
            particle: Particle {
                lifetime: Timer::from_seconds(0.4, TimerMode::Once),
            },
            sprite: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    custom_size: Some(Vec2::splat(8.0)),
                    ..default()
                },
                texture_atlas: particle_atlas,
                ..default()
            },
            falling: Some(FallingParticle { speed: 12.0 }),
            radial: Some(RadialParticle {
                speed: 14.0,
                direction: Vec2::ZERO,
            }),
            rotating: Some(RotatingParticle { speed: 6.0 }),
            fading: Some(FadingParticle {}),
        },
        Vec2::new(0.0, 13.0),
        Vec2::new(16.0, 4.0),
        None,
        1,
        None,
    );

    commands
        .entity(head_particle_emitter)
        .insert(PlayerHeadParticles);

    commands
        .spawn((
            CharacterBundle::new(Vec3::new(240.0, 240.0, CHARACTER_Z), Character::Player),
            RigidBody::KinematicPositionBased,
            //Collider::capsule(Vec2::new(0.0, -6.3), Vec2::new(0.0, 2.5), 20.0 / 2.0),
            Collider::cuboid(17.0 / 2.0, 28.0 / 2.0),
            PlayerVelocity {
                velocity: Vec2::ZERO,
                last_grounded: 0,
            },
            KinematicCharacterController::default(),
            Name::new("Player"),
        ))
        .add_child(head_particle_emitter)
        .add_child(feet_particle_emitter);

    load_map(&mut commands, &assets);

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(0.1, 0.1)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 999.9),
            ..default()
        },
        Name::new("WhiteDot"),
    ));
}
