use rand::{rngs::ThreadRng, Rng};

use crate::prelude::*;

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(particle_emitter_spawn)
            .add_system(particles_lifetime)
            .add_system(particles_rotate)
            .add_system(particles_radial)
            .add_system(particles_fade)
            .add_system(particles_fall);
    }
}

pub fn spawn_new_rect_emitter(
    commands: &mut Commands,
    particle_desc: ParticleDesc,
    position: Vec2,
    size: Vec2,
    lifetime: Option<f32>,
    varients: usize,
    rate: Option<f32>,
) -> Entity {
    let parent = commands
        .spawn((
            SpatialBundle::from_transform(Transform::from_xyz(position.x, position.y, PARTICLE_Z)),
            ParticleParent,
            Name::new("ParticleParent"),
        ))
        .id();
    let mut desc = RectParticleEmitter {
        particle_parent: parent,
        size,
        rate: None,
        force_spawn: 0,
        varients,
        desc: particle_desc.clone(),
    };

    if let Some(rate) = rate {
        desc.rate = Some(Timer::from_seconds(rate, TimerMode::Repeating));
    }

    let emitter = commands
        .spawn((
            SpatialBundle::from_transform(Transform::from_xyz(position.x, position.y, PARTICLE_Z)),
            desc,
            Name::new("ParticleEmitter"),
        ))
        .id();

    if let Some(lifetime) = lifetime {
        commands.entity(emitter).insert(Lifetime {
            timer: Timer::from_seconds(lifetime, TimerMode::Once),
        });

        commands.entity(parent).insert(Lifetime {
            timer: Timer::from_seconds(
                lifetime + particle_desc.particle.lifetime.remaining_secs(),
                TimerMode::Once,
            ),
        });
    }

    emitter
}

fn spawn_particle(
    commands: &mut Commands,
    parents: &Query<&GlobalTransform, With<ParticleParent>>,
    emitter: &RectParticleEmitter,
    emitter_transform: &GlobalTransform,
    rng: &mut ThreadRng,
) {
    let (x_offset, y_offset) = (
        rng.gen_range((-emitter.size.x / 2.0)..(emitter.size.x / 2.0)),
        rng.gen_range((-emitter.size.y / 2.0)..(emitter.size.y / 2.0)),
    );
    let emitter_to_parent_difference = emitter_transform.translation().truncate()
        - parents
            .get(emitter.particle_parent)
            .expect("No parent")
            .translation()
            .truncate();

    //Faster to spawn batch or not noticible?
    //TODO move all generic emitter work to a standalone function
    let mut sprite = emitter.desc.sprite.clone();
    sprite.sprite.index = rng.gen_range(0..emitter.varients);
    sprite.transform.translation =
        Vec3::new(x_offset, y_offset, 0.0) + emitter_to_parent_difference.extend(0.0);

    let mut particle = commands.spawn((sprite, emitter.desc.particle.clone()));

    if let Some(falling) = &emitter.desc.falling {
        particle.insert(falling.clone());
    }
    if let Some(fading) = &emitter.desc.fading {
        particle.insert(fading.clone());
    }
    if let Some(radial) = &emitter.desc.radial {
        let mut radial = radial.clone();
        radial.direction = Vec2::new(x_offset, y_offset);
        particle.insert(radial);
    }
    if let Some(rotating) = &emitter.desc.rotating {
        particle.insert(rotating.clone());
    }

    let particle = particle.id();
    commands.entity(emitter.particle_parent).add_child(particle);
}

fn particle_emitter_spawn(
    mut commands: Commands,
    //Global transforms allow for moving emitters and static parents
    mut emitters: Query<(&mut RectParticleEmitter, &GlobalTransform)>,
    parents: Query<&GlobalTransform, With<ParticleParent>>,
    time: Res<Time>,
) {
    let mut rng = rand::thread_rng();
    for (mut emitter, emitter_transform) in &mut emitters {
        if emitter.rate.is_some() {
            emitter.rate.as_mut().unwrap().tick(time.delta());

            for _i in 0..emitter.rate.as_ref().unwrap().times_finished_this_tick() {
                spawn_particle(
                    &mut commands,
                    &parents,
                    &emitter,
                    emitter_transform,
                    &mut rng,
                )
            }
        }
        for _i in 0..emitter.force_spawn {
            spawn_particle(
                &mut commands,
                &parents,
                &emitter,
                emitter_transform,
                &mut rng,
            )
        }
        emitter.force_spawn = 0;
    }
}

//TODO object pooling?
fn particles_lifetime(
    mut commands: Commands,
    mut particles: Query<(Entity, &mut Particle)>,
    time: Res<Time>,
) {
    for (entity, mut particle) in &mut particles {
        particle.lifetime.tick(time.delta());
        if particle.lifetime.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn particles_fall(
    mut particles: Query<(&mut Transform, &FallingParticle), With<Particle>>,
    time: Res<Time>,
) {
    for (mut transform, falling) in &mut particles {
        transform.translation.y -= falling.speed * time.delta_seconds();
    }
}

fn particles_radial(
    mut particles: Query<(&mut Transform, &RadialParticle), With<Particle>>,
    time: Res<Time>,
) {
    for (mut transform, radial) in &mut particles {
        transform.translation +=
            (radial.speed * time.delta_seconds()) * radial.direction.extend(0.0);
    }
}

fn particles_fade(
    mut particles: Query<(&mut TextureAtlasSprite, &Particle), With<FadingParticle>>,
) {
    for (mut sprite, particle) in &mut particles {
        sprite.color.set_a(particle.lifetime.percent_left());
    }
}

fn particles_rotate(
    mut particles: Query<(&mut Transform, &RotatingParticle), With<Particle>>,
    time: Res<Time>,
) {
    for (mut transform, rotating) in &mut particles {
        transform.rotation *= Quat::from_rotation_z(rotating.speed * time.delta_seconds());
    }
}

impl Default for Particle {
    fn default() -> Self {
        Self {
            lifetime: Timer::from_seconds(1.0, TimerMode::Once),
        }
    }
}

impl Particle {
    pub fn new(lifetime: f32) -> Self {
        Self {
            lifetime: Timer::from_seconds(lifetime, TimerMode::Once),
        }
    }
}
