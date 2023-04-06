use bevy::ui::update;

use crate::prelude::*;

#[derive(Component)]
pub struct DialogUI;

#[derive(Component)]
pub struct DialogText;

pub struct DialogPlugin;

impl Plugin for DialogPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((update_dialog_box, close_dialog))
            .add_systems((show_blur, enter_cutscene).in_schedule(OnEnter(GameState::Cutscene)))
            .add_system(hide_blur.in_schedule(OnExit(GameState::Cutscene)));
    }
}

fn show_blur(mut texture: Query<&mut Visibility, With<Handle<BlurMaterial>>>) {
    for mut visible in &mut texture {
        *visible = Visibility::Visible;
    }
}

fn hide_blur(mut texture: Query<&mut Visibility, With<Handle<BlurMaterial>>>) {
    for mut visible in &mut texture {
        *visible = Visibility::Hidden;
    }
}

fn update_dialog_box(
    mut text: Query<&mut Style, With<DialogText>>,
    camera: Query<&Camera, With<MainCamera>>,
) {
    let camera = camera.single();

    let screen_width = camera.logical_viewport_size().unwrap().x;

    for mut text in &mut text {
        //AHHHHH why must this be Px not percent :(
        //https://github.com/bevyengine/bevy/issues/1490
        text.size.width = Val::Px(screen_width * 0.9 * 0.75 - 30.);
    }
}

fn enter_cutscene(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut progression: ResMut<StoryProgression>,
) {
    // include for wasm safety
    let lines = include_str!("../assets/plot.txt");
    let lines: Vec<String> = lines.lines().map(|l| l.to_string()).collect();
    spawn_dialog_box(&mut commands, &assets, &lines[progression.story_marker]);
    progression.story_marker += 1;
}

fn close_dialog(
    mut commands: Commands,
    mut overworld_state: ResMut<NextState<GameState>>,
    input: Res<Input<KeyCode>>,
    dialog: Query<Entity, With<DialogUI>>,
) {
    if input.just_pressed(KeyCode::Space) {
        for dialog in &dialog {
            commands.entity(dialog).despawn_recursive();
            overworld_state.set(GameState::Platforming);
        }
    }
}

pub fn spawn_dialog_box(
    commands: &mut Commands,
    assets: &Res<AssetServer>,
    starting_text: &str,
) -> Entity {
    //FIXME: Global font setting
    let font = assets.load("fonts/pointfree.ttf");

    let parent = (
        NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(90.0), Val::Percent(30.0)),
                align_self: AlignSelf::FlexEnd,
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::FlexStart,
                flex_direction: FlexDirection::Row,
                position_type: PositionType::Absolute,
                position: UiRect::left(Val::Percent(5.0)),
                margin: UiRect::bottom(Val::Percent(4.0)),
                ..default()
            },
            ..default()
        },
        DialogUI,
        Name::new("Dialog UI"),
    );

    let player_pfp = (
        ImageBundle {
            image: UiImage {
                texture: assets.load("player_pfp.png"),
                ..default()
            },
            style: Style {
                size: Size::new(Val::Percent(25.0), Val::Percent(100.0)),
                align_self: AlignSelf::FlexEnd,
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::FlexStart,
                flex_direction: FlexDirection::Row,
                //position: UiRect::left(Val::Percent(10.0)),
                //margin: UiRect::bottom(Val::Percent(4.0)),
                ..default()
            },
            ..default()
        },
        DialogUI,
        Name::new("Dialog UI"),
    );

    let text_parent = (
        NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(75.0), Val::Percent(100.0)),
                align_self: AlignSelf::FlexEnd,
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::FlexStart,
                flex_direction: FlexDirection::Row,
                //position: UiRect::left(Val::Percent(10.0)),
                //margin: UiRect::bottom(Val::Percent(4.0)),
                padding: UiRect::new(Val::Percent(1.0), Val::Auto, Val::Px(7.0), Val::Auto),
                ..default()
            },
            background_color: BackgroundColor(Color::rgb(0.9, 0.9, 0.9)),
            ..default()
        },
        Name::new("Text Box"),
    );

    let dialog_text = (
        TextBundle::from_section(
            starting_text,
            TextStyle {
                font,
                font_size: 30.0,
                color: Color::BLACK,
            },
        )
        .with_text_alignment(TextAlignment::Left),
        DialogText,
    );

    commands
        .spawn(parent)
        .with_children(|commands| {
            commands.spawn(player_pfp);
            commands.spawn(text_parent).with_children(|commands| {
                commands.spawn(dialog_text);
            });
        })
        .id()
}
