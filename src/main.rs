use bevy::{input::common_conditions::input_toggle_active, render::camera::ScalingMode};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
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
        .add_systems((player_gravity, player_jump, player_control, player_update).chain())
        .add_plugin(ArtPlugin);

    app.run();
}

#[derive(Component)]
pub struct PlayerVelocity {
    pub velocity: Vec2,
    pub last_grounded: usize,
}

fn player_gravity(
    mut player: Query<&mut PlayerVelocity>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let float_gravity = -450.0;
    let true_gravity = -1500.0;
    for mut velocity in player.iter_mut() {
        if keyboard.pressed(KeyCode::Space) {
            velocity.velocity += Vec2::new(0.0, float_gravity * time.delta_seconds());
        } else {
            velocity.velocity += Vec2::new(0.0, true_gravity * time.delta_seconds());
        }
    }
}

fn player_control(
    mut player: Query<&mut PlayerVelocity>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let player_accel = 600.0;
    let player_deccel = 450.0;
    let player_max_velocity = 225.0;
    for mut velocity in player.iter_mut() {
        if keyboard.pressed(KeyCode::A) {
            velocity.velocity += Vec2::new(-player_accel * time.delta_seconds(), 0.0);
        }
        if keyboard.pressed(KeyCode::D) {
            velocity.velocity += Vec2::new(player_accel * time.delta_seconds(), 0.0);
        }
        if !keyboard.pressed(KeyCode::A) && !keyboard.pressed(KeyCode::D) {
            //TODO time dependent slow down?
            velocity.velocity.x +=
                -player_deccel * velocity.velocity.x.signum() * time.delta_seconds();
        }

        velocity.velocity.x = velocity
            .velocity
            .x
            .clamp(-player_max_velocity, player_max_velocity);
    }
}

fn player_jump(
    mut controllers: Query<(&KinematicCharacterControllerOutput, &mut PlayerVelocity)>,
    keyboard: Res<Input<KeyCode>>,
) {
    let jump_strength = 320.0;
    for (controller, mut velocity) in controllers.iter_mut() {
        if controller.desired_translation.y - controller.effective_translation.y > 0.1 {
            velocity.velocity.y = -0.1;
        }
        if (controller.desired_translation.x - controller.effective_translation.x).abs() > 0.1 {
            velocity.velocity.x = 0.0;
        }
        if controller.grounded {
            velocity.velocity.y = -0.1;
            if keyboard.just_pressed(KeyCode::Space) {
                velocity.velocity += Vec2::new(0.0, jump_strength);
                velocity.last_grounded = 999;
            }
        } else {
            info!("NOT Grounded");
        }
    }
}

fn player_update(
    mut controllers: Query<(&mut KinematicCharacterController, &PlayerVelocity)>,
    time: Res<Time>,
) {
    for (mut controller, velocity) in controllers.iter_mut() {
        controller.translation = Some(velocity.velocity * time.delta_seconds());
    }
}

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::FixedVertical(HEIGHT);

    commands.spawn(camera);

    commands.spawn((
        CharacterBundle::new(Vec3::new(-3.0, 0.0, CHARACTER_Z), Character::Player),
        RigidBody::KinematicPositionBased,
        Collider::capsule(Vec2::new(0.0, -6.3), Vec2::new(0.0, 2.5), 20.0 / 2.0),
        PlayerVelocity {
            velocity: Vec2::ZERO,
            last_grounded: 0,
        },
        KinematicCharacterController::default(),
        Name::new("Player"),
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite { ..default() },
            texture: assets.load("background_1.png"),
            transform: Transform::from_xyz(0.0, 0.0, BACKGROUND_Z),
            ..default()
        },
        Name::new("Background"),
    ));

    commands
        .spawn(Collider::cuboid(500.0, 50.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)));

    commands
        .spawn(Collider::cuboid(48.0, 16.0))
        .insert(TransformBundle::from(Transform::from_xyz(-114.0, 0.0, 0.0)));

    commands
        .spawn(Collider::cuboid(32.0, 16.0))
        .insert(TransformBundle::from(Transform::from_xyz(128.0, 0.0, 0.0)));

    commands
        .spawn(Collider::cuboid(16.0, 16.0))
        .insert(TransformBundle::from(Transform::from_xyz(16.0, 128.0, 0.0)));

    commands
        .spawn(Collider::cuboid(16.0, 16.0))
        .insert(TransformBundle::from(Transform::from_xyz(16.0, 64.0, 0.0)));

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
