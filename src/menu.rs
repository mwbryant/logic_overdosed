use crate::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_menu_ui.in_schedule(OnEnter(GameState::Menu)))
            .add_system(despawn_with::<MenuElement>.in_schedule(OnExit(GameState::Menu)))
            .add_systems((update_menu_ui, exit_menu).in_set(OnUpdate(GameState::Menu)));
    }
}

#[derive(Component)]
struct MenuElement;

#[derive(Component)]
struct MenuFade;

fn spawn_menu_ui(mut commands: Commands, assets: Res<AssetServer>) {
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
                position: UiRect::right(Val::Percent(30.0)),
                ..default()
            },
            background_color: Color::ALICE_BLUE.into(),
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

fn exit_menu(fade: Query<&Fadeout, With<MenuFade>>, mut next_state: ResMut<NextState<GameState>>) {
    for fade in &fade {
        if fade.fade_in_just_finished {
            next_state.set(GameState::Cutscene);
        }
    }
}
