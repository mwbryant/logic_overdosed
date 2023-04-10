#![allow(clippy::type_complexity)]
mod art;
mod cutscene;
mod map;
mod menu;
mod player;
mod timer;
mod utils;

pub mod prelude {
    pub const WIDTH: f32 = 640.0;
    pub const HEIGHT: f32 = 480.0;
    pub const RESOLUTION: f32 = WIDTH / HEIGHT;

    pub use crate::art::*;
    pub use crate::cutscene::*;
    pub use crate::map::*;
    pub use crate::menu::*;
    pub use crate::player::*;
    pub use crate::timer::*;
    pub use crate::utils::*;

    pub use bevy::{prelude::*, utils::HashMap};
    pub use bevy_rapier2d::prelude::*;

    pub const BACKGROUND_Z: f32 = 10.0;
    pub const ENEMY_Z: f32 = 90.0;
    pub const NPC_Z: f32 = 95.0;
    pub const CHARACTER_Z: f32 = 100.0;
    pub const WEAPON_Z: f32 = 150.0;
    pub const PARTICLE_Z: f32 = 750.0;
    pub const ICON_Z: f32 = 850.0;
    pub const WORLD_UI_Z: f32 = 999.0;

    pub struct DisableEffectsEvent;

    #[derive(Resource)]
    pub struct MainRender(pub Handle<Image>);

    #[derive(States, PartialEq, Eq, Debug, Default, Clone, Hash)]
    pub enum GameState {
        #[default]
        Menu,
        Platforming,
        Cutscene,
        Win,
    }
    #[derive(Component)]
    pub struct MainCamera;

    #[derive(Component)]
    pub struct Potion;

    #[derive(Component)]
    pub struct Door;

    #[derive(Component)]
    pub struct PotionFade(pub usize);

    #[derive(Resource, Default)]
    pub struct StoryProgression {
        pub story_marker: usize,
        pub current_map: usize,
        pub respawn_point: Vec3,
        pub respawn_alt: bool,
        pub potion_spawns: Vec<Vec2>,
        pub levels: Vec<String>,
        pub potion_effects: Vec<PlayerStats>,
    }
}
