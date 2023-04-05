use crate::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((player_gravity, player_jump, player_control, player_update).chain())
            .add_system(player_particles)
            .add_system(player_animation);
    }
}

#[derive(Component)]
pub struct PlayerVelocity {
    pub velocity: Vec2,
    pub last_grounded: usize,
}

#[derive(Component)]
pub struct PlayerFeetParticles;

#[derive(Component)]
pub struct PlayerHeadParticles;

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
            //FIXME gives the shakes
            let deccel_amount =
                -player_deccel * velocity.velocity.x.signum() * time.delta_seconds();
            if velocity.velocity.x.abs() < deccel_amount.abs() {
                velocity.velocity.x = 0.0;
            } else {
                velocity.velocity.x += deccel_amount
            }
        }

        velocity.velocity.x = velocity
            .velocity
            .x
            .clamp(-player_max_velocity, player_max_velocity);
    }
}

fn player_jump(
    mut controllers: Query<(&KinematicCharacterControllerOutput, &mut PlayerVelocity)>,
    mut player_particles: Query<&mut RectParticleEmitter, With<PlayerHeadParticles>>,
    keyboard: Res<Input<KeyCode>>,
) {
    let jump_strength = 320.0;
    for (controller, mut velocity) in controllers.iter_mut() {
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
                velocity.velocity += Vec2::new(0.0, jump_strength);
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
