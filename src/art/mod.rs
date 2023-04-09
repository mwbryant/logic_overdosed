mod animation;
mod fade_in;
mod particles;
mod post_processing;
mod sprite_animation;
pub mod sprite_sheets;
use serde::{Deserialize, Serialize};
pub use sprite_sheets::*;

use crate::prelude::*;

pub use fade_in::spawn_fadeout;
pub use particles::spawn_new_rect_emitter;
pub use post_processing::*;

use self::animation::AnimationPlugin;
use self::fade_in::FadeInPlugin;
use self::particles::ParticlePlugin;
use self::sprite_animation::SpriteAnimationPlugin;

pub struct ArtPlugin;

impl Plugin for ArtPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(SpriteSheetPlugin)
            .add_plugin(ParticlePlugin)
            .add_plugin(AnimationPlugin)
            .add_plugin(SpriteAnimationPlugin)
            .add_plugin(FadeInPlugin)
            .add_plugin(PostProcessingPlugin)
            .register_type::<Icon>()
            .register_type::<Particle>()
            .register_type::<FallingParticle>()
            .register_type::<FadingParticle>()
            .register_type::<RadialParticle>()
            .register_type::<Fadeout>()
            .register_type::<RotatingParticle>()
            .register_type::<Character>();
    }
}

#[derive(Component)]
pub struct PostProcessingQuad;

#[derive(Component, Default, Reflect, Clone)]
#[reflect(Component)]
pub struct AnimatedSpriteStrip {
    pub current_index: usize,
    pub frames: Vec<usize>,
    pub frame_timer: Timer,
    pub sprite_size: Vec2,
}

#[derive(Reflect)]
pub enum FadeoutState {
    FadingIn,
    Hold,
    FadingOut,
}

#[derive(Component, Reflect)]
pub struct Fadeout {
    pub fade_in_just_finished: bool,
    in_timer: Timer,
    hold_timer: Timer,
    out_timer: Timer,
    state: FadeoutState,
}

#[derive(Component, Reflect)]
pub struct DeathAnimation;

#[derive(Component)]
pub struct RectParticleEmitter {
    pub particle_parent: Entity,
    pub size: Vec2,
    pub rate: Option<Timer>,
    pub force_spawn: usize,
    pub varients: usize,
    //It would be nice to be able to give the emitter a tag to add to particles
    pub desc: ParticleDesc,
}

#[derive(Component, Default, Clone)]
pub struct ParticleDesc {
    pub particle: Particle,
    pub sprite: SpriteSheetBundle,
    pub falling: Option<FallingParticle>,
    pub radial: Option<RadialParticle>,
    pub rotating: Option<RotatingParticle>,
    pub fading: Option<FadingParticle>,
}

#[derive(Component, Reflect)]
pub struct ParticleParent;

#[derive(Component, Clone, Reflect)]
pub struct Particle {
    pub lifetime: Timer,
}

#[derive(Component, Clone, Reflect)]
pub struct FallingParticle {
    pub speed: f32,
}

#[derive(Component, Clone, Reflect)]
pub struct RadialParticle {
    pub speed: f32,
    pub direction: Vec2,
}

#[derive(Component, Clone, Reflect)]
pub struct RotatingParticle {
    pub speed: f32,
}

#[derive(Component, Clone, Reflect)]
pub struct FadingParticle {}

#[derive(Bundle)]
pub struct CharacterBundle {
    #[bundle]
    pub sprite_sheet: SpriteSheetBundle,
    character: Character,
}

#[derive(Bundle)]
pub struct IconBundle {
    #[bundle]
    sprite_sheet: SpriteSheetBundle,
    icon: Icon,
}

pub const CHARACTER_SHEET_WIDTH: usize = 1;
pub const CHARACTER_SHEET_HEIGHT: usize = 1;
pub const ICON_SHEET_WIDTH: usize = 34;

#[derive(Component, Clone, PartialEq, Eq, Hash, Default, Reflect)]
pub enum Icon {
    #[default]
    KeyE,
}

#[derive(Component, Clone, PartialEq, Eq, Hash, Default, Reflect, Serialize, Deserialize)]
pub enum Character {
    #[default]
    Player,
}

#[derive(Resource)]
pub struct SpriteSheetMaps {
    character_atlas: Handle<TextureAtlas>,
    pub characters: HashMap<Character, usize>,
}
