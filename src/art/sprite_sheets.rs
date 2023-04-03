use bevy::sprite::Anchor;

use crate::prelude::*;

pub struct SpriteSheetPlugin;

impl Plugin for SpriteSheetPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_spritesheet_maps.in_base_set(StartupSet::PreStartup))
            .add_system(update_art);
    }
}

fn update_art(
    mut characters: Query<(
        &mut TextureAtlasSprite,
        &mut Handle<TextureAtlas>,
        &Character,
    )>,
    sprite_sheets: Res<SpriteSheetMaps>,
) {
    for (mut sprite, mut atlas, character) in &mut characters {
        *atlas = sprite_sheets.character_atlas.clone();
        sprite.index = sprite_sheets.characters[character];
    }
}

fn setup_spritesheet_maps(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("player.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(25.0, 31.0),
        CHARACTER_SHEET_WIDTH,
        CHARACTER_SHEET_HEIGHT,
        Some(Vec2::splat(1.0)),
        None,
    );
    let character_atlas = texture_atlases.add(texture_atlas);

    let texture_handle = asset_server.load("input_icons/Tilemap/tilemap.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(16.0, 16.0),
        ICON_SHEET_WIDTH,
        24,
        Some(Vec2::splat(1.0)),
        None,
    );
    let icon_atlas = texture_atlases.add(texture_atlas);

    let characters = HashMap::from([(Character::Player, 0)]);

    let icons = HashMap::from([(Icon::KeyE, 0)]);

    commands.insert_resource(SpriteSheetMaps {
        character_atlas,
        icon_atlas,
        characters,
        icons,
    });
}

impl Default for CharacterBundle {
    fn default() -> Self {
        Self {
            sprite_sheet: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    //custom_size: Some(Vec2::splat(1.0)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, CHARACTER_Z)),
                ..Default::default()
            },
            character: Character::Player,
        }
    }
}

impl CharacterBundle {
    pub fn new(position: Vec3, character: Character) -> Self {
        let mut bundle = CharacterBundle {
            character,
            ..default()
        };

        bundle.sprite_sheet.transform.translation = position;

        bundle
    }
}

impl Default for IconBundle {
    fn default() -> Self {
        Self {
            sprite_sheet: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    // custom_size: Some(Vec2::splat(1.0)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, ICON_Z)),
                ..Default::default()
            },
            icon: Icon::KeyE,
        }
    }
}

impl IconBundle {
    pub fn new(position: Vec2, icon: Icon, scale: Vec2) -> Self {
        let mut bundle = IconBundle { icon, ..default() };

        bundle.sprite_sheet.transform.translation = position.extend(ICON_Z);
        bundle.sprite_sheet.transform.scale = scale.extend(1.0);

        bundle
    }
}
