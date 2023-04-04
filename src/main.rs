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
    let mut texture = texture.single_mut();
    texture.scale.x = window.resolution.width() / WIDTH;
    texture.scale.y = window.resolution.height() / HEIGHT;
}

fn setup_camera(
    mut commands: Commands,
    windows: Query<&Window>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut post_processing_materials: ResMut<Assets<PostProcessingMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    // This assumes we only have a single window
    let window = windows.single();

    let size = Extent3d {
        width: window.resolution.physical_width(),
        height: window.resolution.physical_height(),
        ..default()
    };

    // This is the texture that will be rendered to.
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

    // fill image.data with zeroes
    image.resize(size);

    let image_handle = images.add(image);

    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::FixedVertical(HEIGHT);
    camera.transform.translation.x = 320.0;
    camera.transform.translation.y = 240.0;
    camera.camera.target = RenderTarget::Image(image_handle.clone());

    // Main camera, first to render
    commands.spawn((
        camera,
        // Disable UI rendering for the first pass camera. This prevents double rendering of UI at
        // the cost of rendering the UI without any post processing effects.
        UiCameraConfig { show_ui: false },
    ));

    // This specifies the layer used for the post processing camera, which will be attached to the post processing camera and 2d quad.
    let post_processing_pass_layer = RenderLayers::layer((RenderLayers::TOTAL_LAYERS - 1) as u8);

    let quad_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(WIDTH, HEIGHT))));

    // This material has the texture that has been rendered.
    let material_handle = post_processing_materials.add(PostProcessingMaterial {
        source_image: image_handle,
    });

    // Post processing 2d quad, with material using the render texture done by the main camera, with a custom shader.
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: quad_handle.into(),
            material: material_handle,
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 1.5),
                ..default()
            },
            ..default()
        },
        PostProcessingQuad,
        post_processing_pass_layer,
        Name::new("Post Processing"),
    ));

    // The post-processing pass camera.
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                // renders after the first main camera which has default value: 0.
                order: 1,
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

    let particle_emitter = spawn_new_rect_emitter(
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
            radial: None,
            rotating: Some(RotatingParticle { speed: 24.0 }),
            fading: Some(FadingParticle {}),
        },
        Vec2::new(0.0, -13.0),
        Vec2::new(3.0, 3.0),
        None,
        1,
        None,
    );

    commands.entity(particle_emitter).insert(PlayerParticles);

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
        .add_child(particle_emitter);

    load_map(&mut commands);

    commands.spawn((
        SpriteBundle {
            sprite: Sprite { ..default() },
            texture: assets.load("background_1.png"),
            transform: Transform::from_xyz(320.0, 240.0, BACKGROUND_Z),
            ..default()
        },
        Name::new("Background"),
    ));

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
