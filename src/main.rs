use bevy::{
    input::common_conditions::input_toggle_active,
    render::{
        camera::ScalingMode,
        render_resource::{AddressMode, FilterMode, SamplerDescriptor},
    },
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use logic_overdosed::prelude::*;

use bevy::render::{
    camera::RenderTarget,
    render_resource::{
        Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
    },
    texture::BevyDefault,
    view::RenderLayers,
};

fn main() {
    let mut app = App::new();

    app.add_state::<GameState>()
        .add_event::<DisableEffectsEvent>()
        .insert_resource(StoryProgression {
            story_marker: 0,
            respawn_point: Vec3::new(55.0, 50.0, CHARACTER_Z),
            current_map: 0,
            potion_spawns: vec![Vec2::new(380.0, 130.0), Vec2::new(380.0, 130.0)],
            levels: vec![
                //TODO find better way to handle this that also works on web
                include_str!("../assets/maps/map_1.map").to_string(),
                include_str!("../assets/maps/map_2.map").to_string(),
                include_str!("../assets/maps/map_3.map").to_string(),
                include_str!("../assets/maps/map_4.map").to_string(),
            ],
            potion_effects: vec![
                ron::from_str::<PlayerStats>(include_str!("../assets/potions/level_1.ron"))
                    .unwrap(),
                ron::from_str::<PlayerStats>(include_str!("../assets/potions/level_2.ron"))
                    .unwrap(),
                ron::from_str::<PlayerStats>(include_str!("../assets/potions/level_3.ron"))
                    .unwrap(),
                ron::from_str::<PlayerStats>(include_str!("../assets/potions/level_4.ron"))
                    .unwrap(),
            ],
        })
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin {
                    default_sampler: SamplerDescriptor {
                        address_mode_u: AddressMode::Repeat,
                        address_mode_v: AddressMode::Repeat,
                        address_mode_w: AddressMode::Repeat,
                        mag_filter: FilterMode::Nearest,
                        min_filter: FilterMode::Nearest,
                        mipmap_filter: FilterMode::Nearest,
                        ..default()
                    },
                })
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
        .add_system(setup_player.in_schedule(OnExit(GameState::Menu)))
        .add_system(update_lifetimes.in_base_set(CoreSet::PostUpdate))
        .add_startup_system(setup_camera)
        .add_system(setup_default_map.in_schedule(OnExit(GameState::Menu)))
        .add_system(camera_updating)
        .add_plugin(PlayerPlugin)
        .add_plugin(DialogPlugin)
        .add_plugin(MapPlugin)
        .add_plugin(MenuPlugin)
        .add_plugin(SpeedrunPlugin)
        .add_plugin(ArtPlugin);

    app.run();
}

fn camera_updating(
    player: Query<&Transform, (With<PlayerVelocity>, Without<MainCamera>)>,
    mut camera: Query<&mut Transform, With<MainCamera>>,
) {
    if let Ok(player) = player.get_single() {
        let mut camera = camera.single_mut();
        camera.translation.x = player.translation.x;
        camera.translation.x = camera.translation.x.clamp(WIDTH / 2.0, WIDTH * 3.5);
    }
}

fn setup_default_map(
    mut commands: Commands,
    assets: Res<AssetServer>,
    progression: Res<StoryProgression>,
) {
    load_map(&mut commands, &assets, &progression);
}

fn setup_camera(
    mut commands: Commands,
    windows: Query<&Window>,
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

    commands.spawn((camera, MainCamera, UiCameraConfig { show_ui: false }));

    commands.insert_resource(MainRender(image_handle));

    let post_processing_pass_layer = RenderLayers::layer((RenderLayers::TOTAL_LAYERS - 1) as u8);

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

fn setup_player(
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
            CharacterBundle::new(Vec3::new(55.0, 50.0, CHARACTER_Z), Character::Player),
            RigidBody::KinematicPositionBased,
            //Collider::capsule(Vec2::new(0.0, -6.3), Vec2::new(0.0, 2.5), 20.0 / 2.0),
            Collider::cuboid(17.0 / 2.0, 28.0 / 2.0),
            PlayerVelocity {
                velocity: Vec2::ZERO,
                on_wall: OnWall::NotOnWall,
                last_on_wall: 0,
            },
            KinematicCharacterController {
                filter_flags: QueryFilterFlags::EXCLUDE_SENSORS,
                ..default()
            },
            ron::from_str::<PlayerStats>(include_str!("../assets/potions/default_player.ron"))
                .unwrap(),
            Name::new("Player"),
        ))
        .add_child(head_particle_emitter)
        .add_child(feet_particle_emitter);

    /*
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
    */
}
