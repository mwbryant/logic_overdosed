use crate::prelude::*;

pub struct SpriteAnimationPlugin;

impl Plugin for SpriteAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<AnimatedSpriteStrip>()
            .add_system(animate_sprite_strips);
    }
}

/*

AnimatedSpriteStrip {
    frames: vec![2, 3],
    frame_timer: Timer::from_seconds(0.2, TimerMode::Repeating),
    sprite_size: Vec2::splat(16.0),
    ..default()
},
*/

fn animate_sprite_strips(
    mut sprites: Query<(&mut Sprite, &mut AnimatedSpriteStrip)>,
    time: Res<Time>,
) {
    for (mut sprite, mut strip) in &mut sprites {
        strip.frame_timer.tick(time.delta());
        if strip.frame_timer.just_finished() {
            let index = strip.current_index;
            strip.current_index = index + 1;
        }

        if strip.current_index >= strip.frames.len() {
            strip.current_index = 0;
        }

        let buffer = Vec2::splat(0.1);

        sprite.rect = Some(Rect {
            min: Vec2::new(
                strip.sprite_size.x * strip.frames[strip.current_index] as f32,
                0.0,
            ) + buffer,
            max: Vec2::new(
                strip.sprite_size.x * (strip.frames[strip.current_index] as f32 + 1.0),
                strip.sprite_size.y,
            ) - buffer,
        });
    }
}
