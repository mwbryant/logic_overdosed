use bevy_easings::Lerp;

use crate::prelude::*;

#[derive(Component)]
pub struct DialogUI;

#[derive(Component)]
pub struct DialogText;

pub struct DialogPlugin;

impl Plugin for DialogPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((update_dialog_box, close_dialog))
            .insert_resource(CutsceneTracker {
                timer: Timer::from_seconds(1.5, TimerMode::Once),
            })
            .add_systems((enter_cutscene,).in_schedule(OnEnter(GameState::Cutscene)))
            .add_system(show_blur.in_set(OnUpdate(GameState::Cutscene)))
            .add_system(hide_blur.in_schedule(OnExit(GameState::Cutscene)));
    }
}

#[derive(Resource)]
pub struct CutsceneTracker {
    timer: Timer,
}

fn show_blur(
    mut texture: Query<&mut Visibility, With<Handle<BlurMaterial>>>,
    fadeout: Query<&Fadeout>,
) {
    if let Ok(fadeout) = fadeout.get_single() {
        if fadeout.fade_in_just_finished {
            for mut visible in &mut texture {
                *visible = Visibility::Visible;
            }
        }
    }
}

fn hide_blur(mut texture: Query<&mut Visibility, With<Handle<BlurMaterial>>>) {
    for mut visible in &mut texture {
        *visible = Visibility::Hidden;
    }
}

fn update_dialog_box(
    mut text: Query<&mut Style, With<DialogText>>,
    mut parent: Query<&mut Style, (With<DialogUI>, Without<DialogText>)>,
    camera: Query<&Camera, With<MainCamera>>,
    mut cutscene: ResMut<CutsceneTracker>,
    fadeout: Query<&Fadeout>,
    time: Res<Time>,
) {
    if fadeout.iter().count() != 0 {
        //Don't do anything if there is a fade happening
        return;
    }
    cutscene.timer.tick(time.delta());
    let camera = camera.single();

    let screen_width = camera.logical_viewport_size().unwrap().x;
    for mut parent in &mut parent {
        //parent.align_content
        parent.margin = UiRect::bottom(Val::Percent(Lerp::lerp(
            &-25.0,
            &4.0,
            &cutscene.timer.percent(),
        )));
    }

    for mut text in &mut text {
        //AHHHHH why must this be Px not percent :(
        //https://github.com/bevyengine/bevy/issues/1490
        text.size.width = Val::Px(screen_width * 0.9 * 0.75 - 30.);
    }
}

fn enter_cutscene(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut cutscene: ResMut<CutsceneTracker>,
    mut progression: ResMut<StoryProgression>,
) {
    cutscene.timer.reset();
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
    cutscene: ResMut<CutsceneTracker>,
) {
    if !cutscene.timer.finished() {
        return;
    }
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
                margin: UiRect::bottom(Val::Percent(-25.0)),
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
        Name::new("Player pfp"),
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
