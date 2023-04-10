use serde::{Deserialize, Serialize};

use crate::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                player_respawn,
                player_exit_level,
                player_gravity,
                player_jump,
                player_control,
                player_update,
                player_death,
            )
                .chain()
                .in_set(OnUpdate(GameState::Platforming)),
        )
        .add_system(player_particles)
        .add_system(apply_player_effect)
        .add_system(player_pickups)
        .add_system(player_animation);
    }
}

#[derive(Component)]
pub struct PlayerVelocity {
    pub velocity: Vec2,
    pub on_wall: OnWall,
    pub last_on_wall: usize,
}

#[derive(PartialEq, Eq, Debug)]
pub enum OnWall {
    NotOnWall,
    OnLeft,
    OnRight,
}

#[derive(Component, Serialize, Deserialize, Clone, Copy)]
pub struct PlayerStats {
    pub float_gravity: f32,
    pub true_gravity: f32,
    pub player_accel: f32,
    pub player_deccel: f32,
    pub player_max_velocity: f32,
    pub jump_strength: f32,
    pub can_wall_jump: bool,
    pub wall_jump_strength: f32,
}

#[derive(Component)]
pub struct PlayerFeetParticles;

#[derive(Component)]
pub struct PlayerHeadParticles;

#[derive(Component)]
pub struct DeathFade;

#[derive(Component)]
pub struct ExitFade;

fn player_death(
    mut commands: Commands,
    player: Query<&Transform, With<PlayerStats>>,
    fade: Query<&DeathFade>,
) {
    if fade.iter().count() > 0 {
        return;
    }
    let player = player.single();
    if player.translation.y < -64.0 {
        let fade = spawn_fadeout(&mut commands, 1.0, 0.4, 1.0);
        commands.entity(fade).insert(DeathFade);
    }
}

#[allow(clippy::too_many_arguments)]
fn player_exit_level(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut player: Query<(&mut PlayerVelocity, &mut Transform), With<PlayerStats>>,
    //TODO despawn on event with util system
    map_entities: Query<Entity, With<MapEntity>>,
    mut disable_effects: EventWriter<DisableEffectsEvent>,
    mut progression: ResMut<StoryProgression>,
    fade: Query<&Fadeout, With<ExitFade>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if let Ok(fade) = fade.get_single() {
        let (mut velocity, mut player) = player.single_mut();
        velocity.velocity.x = 0.0;
        if fade.fade_in_just_finished {
            for map_ent in &map_entities {
                commands.entity(map_ent).despawn_recursive();
            }
            progression.current_map += 1;
            disable_effects.send(DisableEffectsEvent);
            load_map(&mut commands, &assets, &progression);
            player.translation = progression.respawn_point;
            next_state.set(GameState::Cutscene);
        }
    }
}

fn player_respawn(
    mut player: Query<(&mut PlayerVelocity, &mut Transform), With<PlayerStats>>,
    progression: Res<StoryProgression>,
    fade: Query<&Fadeout, With<DeathFade>>,
) {
    if let Ok(fade) = fade.get_single() {
        if fade.fade_in_just_finished {
            let (mut velocity, mut player) = player.single_mut();
            player.translation = progression.respawn_point;
            velocity.velocity = Vec2::ZERO;
        }
    }
}

fn apply_player_effect(
    progression: Res<StoryProgression>,
    fadeout: Query<&Fadeout, With<PotionFade>>,
    mut player: Query<&mut PlayerStats, With<PlayerVelocity>>,
) {
    if let Ok(fadeout) = fadeout.get_single() {
        if fadeout.fade_in_just_finished {
            for mut stats in &mut player {
                *stats = progression.potion_effects[progression.current_map];
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn player_pickups(
    mut commands: Commands,
    sensors: Query<&Name, (With<Sensor>, With<Potion>)>,
    exits: Query<&Name, (With<Sensor>, With<Door>, Without<Potion>)>,
    progression: Res<StoryProgression>,
    rapier_context: Res<RapierContext>,
    player: Query<&Transform, With<PlayerVelocity>>,
    //TODO potions hold refrence to effect?
    //mut event: EventWriter<PotionPickupEvent>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for transform in &player {
        let shape = Collider::cuboid(15.0, 15.0);
        let shape_pos = transform.translation.truncate();
        let filter = QueryFilter::default();

        rapier_context.intersections_with_shape(shape_pos, 0.0, &shape, filter, |entity| {
            if let Ok(_sensors) = sensors.get(entity) {
                //event.send(PotionPickupEvent(progression.current_map));
                let fade = spawn_fadeout(&mut commands, 0.4, 0.1, 0.2);
                commands
                    .entity(fade)
                    .insert(PotionFade(progression.current_map));
                next_state.set(GameState::Cutscene);

                commands.entity(entity).despawn_recursive();
            }
            if let Ok(door) = exits.get(entity) {
                info!("Hit Door {:?} {:?}", entity, door);
                let fadeout = spawn_fadeout(&mut commands, 1.0, 0.4, 1.0);
                commands.entity(fadeout).insert(ExitFade);
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
    if let Ok((mut sprite, player)) = player.get_single_mut() {
        if player.velocity.x > 0.0 {
            sprite.flip_x = true;
        }
        if player.velocity.x < 0.0 {
            sprite.flip_x = false;
        }
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
        if (controller.desired_translation.x - controller.effective_translation.x).abs() > 0.02 {
            if !controller.grounded {
                if controller.desired_translation.x - controller.effective_translation.x > 0.0 {
                    velocity.last_on_wall = 0;
                    velocity.on_wall = OnWall::OnLeft;
                } else {
                    velocity.last_on_wall = 0;
                    velocity.on_wall = OnWall::OnRight;
                }
            } else {
                velocity.last_on_wall += 1;
                velocity.on_wall = OnWall::NotOnWall;
            }
            velocity.velocity.x = 0.0;
        } else if controller.desired_translation.x.abs() > 0.02 {
            velocity.last_on_wall += 1;
            velocity.on_wall = OnWall::NotOnWall;
        }
        if controller.grounded {
            velocity.velocity.y = -0.1;
            if keyboard.just_pressed(KeyCode::Space) {
                velocity.velocity += Vec2::new(0.0, stats.jump_strength);
            }
        } else if stats.can_wall_jump
            && velocity.last_on_wall < 6
            && keyboard.just_pressed(KeyCode::Space)
        {
            if velocity.on_wall == OnWall::OnLeft {
                velocity.velocity.y = -0.1;
                velocity.velocity += Vec2::new(-stats.wall_jump_strength, stats.jump_strength);
            } else {
                velocity.velocity.y = -0.1;
                velocity.velocity += Vec2::new(stats.wall_jump_strength, stats.jump_strength);
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
