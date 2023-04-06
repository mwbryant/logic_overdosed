use crate::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((player_gravity, player_jump, player_control, player_update).chain())
            .add_system(player_particles)
            .add_system(player_pickups)
            .add_system(player_animation);
    }
}

#[derive(Component)]
pub struct PlayerVelocity {
    pub velocity: Vec2,
    pub last_grounded: usize,
}

#[derive(Component)]
pub struct PlayerStats {
    pub float_gravity: f32,
    pub true_gravity: f32,
    pub player_accel: f32,
    pub player_deccel: f32,
    pub player_max_velocity: f32,
    pub jump_strength: f32,
}

#[derive(Component)]
pub struct PlayerFeetParticles;

#[derive(Component)]
pub struct PlayerHeadParticles;

fn player_pickups(
    mut commands: Commands,
    sensors: Query<&Name, (With<Sensor>, With<Potion>)>,
    exits: Query<&Name, (With<Sensor>, With<Door>, Without<Potion>)>,
    mut player: Query<(&mut PlayerStats, &Transform), With<PlayerVelocity>>,
    rapier_context: Res<RapierContext>,
    mut texture: Query<&mut Visibility, With<Handle<ChromaticAbrasionMaterial>>>,
) {
    for (mut stats, transform) in &mut player {
        let shape = Collider::cuboid(15.0, 15.0);
        let shape_pos = transform.translation.truncate();
        let filter = QueryFilter::default();

        rapier_context.intersections_with_shape(shape_pos, 0.0, &shape, filter, |entity| {
            if let Ok(sensors) = sensors.get(entity) {
                info!("Hit {:?} {:?}", entity, sensors);
                stats.jump_strength = 320.0;
                for mut visible in &mut texture {
                    *visible = Visibility::Visible;
                }
                commands.entity(entity).despawn_recursive();
            }
            if let Ok(door) = exits.get(entity) {
                info!("Hit Door {:?} {:?}", entity, door);
                commands.entity(entity).despawn_recursive();
            }
            //XXX what does this do...
            true
        });
    }
}

fn player_particles(
    player: Query<(&PlayerVelocity, &KinematicCharacterControllerOutput)>,
    mut player_particles: Query<&mut RectParticleEmitter, With<PlayerFeetParticles>>,
) {
    for (player, output) in &player {
        let mut particles = player_particles.single_mut();
        if output.grounded {
            particles.force_spawn = player.velocity.x.abs() as usize / 180;
        }
    }
}

fn player_animation(mut player: Query<(&mut TextureAtlasSprite, &PlayerVelocity)>) {
    let (mut sprite, player) = player.single_mut();

    if player.velocity.x > 0.0 {
        sprite.flip_x = true;
    }
    if player.velocity.x < 0.0 {
        sprite.flip_x = false;
    }
}

fn player_gravity(
    mut player: Query<(&mut PlayerVelocity, &PlayerStats)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut velocity, stats) in player.iter_mut() {
        if keyboard.pressed(KeyCode::Space) {
            velocity.velocity += Vec2::new(0.0, stats.float_gravity * time.delta_seconds());
        } else {
            velocity.velocity += Vec2::new(0.0, stats.true_gravity * time.delta_seconds());
        }
    }
}

fn player_control(
    mut player: Query<(&mut PlayerVelocity, &PlayerStats)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut velocity, stats) in player.iter_mut() {
        if keyboard.pressed(KeyCode::A) {
            velocity.velocity += Vec2::new(-stats.player_accel * time.delta_seconds(), 0.0);
        }
        if keyboard.pressed(KeyCode::D) {
            velocity.velocity += Vec2::new(stats.player_accel * time.delta_seconds(), 0.0);
        }
        if !keyboard.pressed(KeyCode::A) && !keyboard.pressed(KeyCode::D) {
            //TODO time dependent slow down?
            //FIXME gives the shakes
            let deccel_amount =
                -stats.player_deccel * velocity.velocity.x.signum() * time.delta_seconds();
            if velocity.velocity.x.abs() < deccel_amount.abs() {
                velocity.velocity.x = 0.0;
            } else {
                velocity.velocity.x += deccel_amount
            }
        }

        velocity.velocity.x = velocity
            .velocity
            .x
            .clamp(-stats.player_max_velocity, stats.player_max_velocity);
    }
}

fn player_jump(
    mut controllers: Query<(
        &KinematicCharacterControllerOutput,
        &mut PlayerVelocity,
        &PlayerStats,
    )>,
    mut player_particles: Query<&mut RectParticleEmitter, With<PlayerHeadParticles>>,
    keyboard: Res<Input<KeyCode>>,
) {
    for (controller, mut velocity, stats) in controllers.iter_mut() {
        if controller.desired_translation.y - controller.effective_translation.y > 0.1 {
            let mut particles = player_particles.single_mut();
            particles.force_spawn = 6;
            velocity.velocity.y = -0.1;
        }
        if (controller.desired_translation.x - controller.effective_translation.x).abs() > 0.1 {
            velocity.velocity.x = 0.0;
        }
        if controller.grounded {
            velocity.velocity.y = -0.1;
            if keyboard.just_pressed(KeyCode::Space) {
                velocity.velocity += Vec2::new(0.0, stats.jump_strength);
                velocity.last_grounded = 999;
            }
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
