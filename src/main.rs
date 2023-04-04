use bevy::{input::common_conditions::input_toggle_active, render::camera::ScalingMode};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use logic_overdosed::{comp_from_config, prelude::*};

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
        .add_plugin(PlayerPlugin)
        .add_plugin(MapPlugin)
        .add_plugin(ArtPlugin);

    app.run();
}

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::FixedVertical(HEIGHT);
    camera.transform.translation.x = 320.0;
    camera.transform.translation.y = 240.0;
    commands.spawn(camera);

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
