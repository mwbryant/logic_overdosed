use crate::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_menu_ui.in_schedule(OnEnter(GameState::Menu)))
            .add_system(despawn_with::<MenuElement>.in_schedule(OnExit(GameState::Menu)))
            .add_system(spawn_win_ui.in_schedule(OnEnter(GameState::Win)))
            .add_systems((update_menu_ui, exit_menu).in_set(OnUpdate(GameState::Menu)));
    }
}

#[derive(Component)]
struct MenuElement;

#[derive(Component)]
struct MenuFade;

fn spawn_win_ui(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut camera: Query<&mut Transform, With<MainCamera>>,
    timer: Query<&TimerUI>,
) {
    let mut camera = camera.single_mut();
    let timer = timer.single();
    camera.translation.x = WIDTH / 2.0;

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(640.0 / 2.0, 480.0 / 2.0, 950.0),
            texture: assets.load("win.png"),
            ..default()
        },
        MenuElement,
    ));
    //FIXME: Global font setting
    let font = assets.load("fonts/pointfree.ttf");

    let parent = (
        NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(55.0), Val::Percent(15.0)),
                align_self: AlignSelf::Center,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                position_type: PositionType::Absolute,
                position: UiRect::new(
                    Val::Undefined,
                    Val::Percent(40.0),
                    Val::Percent(65.0),
                    Val::Undefined,
                ),
                ..default()
            },
            background_color: Color::rgb(154.0 / 255.0, 151.0 / 255.0, 185.0 / 255.0).into(),
            ..default()
        },
        Name::new("Winner Text"),
    );

    let menu_text = (TextBundle::from_section(
        format!("Final Time: {:0.2}s", timer.0.elapsed().as_secs_f32()),
        TextStyle {
            font,
            font_size: 36.0,
            color: Color::BLACK,
        },
    )
    .with_text_alignment(TextAlignment::Left),);
    commands.spawn(parent).with_children(|commands| {
        commands.spawn(menu_text);
    });
}

fn spawn_menu_ui(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(640.0 / 2.0, 480.0 / 2.0, 100.0),
            texture: assets.load("title.png"),
            ..default()
        },
        MenuElement,
    ));
    //FIXME: Global font setting
    let font = assets.load("fonts/pointfree.ttf");

    let parent = (
        ButtonBundle {
            style: Style {
                size: Size::new(Val::Percent(40.0), Val::Percent(15.0)),
                align_self: AlignSelf::Center,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                position_type: PositionType::Absolute,
                position: UiRect::new(
                    Val::Undefined,
                    Val::Percent(40.0),
                    Val::Percent(70.0),
                    Val::Undefined,
                ),
                ..default()
            },
            background_color: Color::rgb(154.0 / 255.0, 151.0 / 255.0, 185.0 / 255.0).into(),
            ..default()
        },
        MenuElement,
        Name::new("Button"),
    );

    let menu_text = (TextBundle::from_section(
        "Play Game",
        TextStyle {
            font,
            font_size: 48.0,
            color: Color::BLACK,
        },
    )
    .with_text_alignment(TextAlignment::Left),);
    commands.spawn(parent).with_children(|commands| {
        commands.spawn(menu_text);
    });
}

fn update_menu_ui(mut commands: Commands, fade: Query<&Fadeout>, button: Query<&Interaction>) {
    //FIXME make this a run condition
    if fade.iter().count() != 0 {
        return;
    }
    for button in &button {
        if button == &Interaction::Clicked {
            let entity = spawn_fadeout(&mut commands, 0.4, 0.2, 0.2);
            commands.entity(entity).insert(MenuFade);
        }
    }
}

fn exit_menu(
    fade: Query<&Fadeout, With<MenuFade>>,
    mut disable: EventWriter<DisableEffectsEvent>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for fade in &fade {
        if fade.fade_in_just_finished {
            disable.send(DisableEffectsEvent);
            next_state.set(GameState::Cutscene);
        }
    }
}
