use bevy::time::Stopwatch;

use crate::prelude::*;

pub struct SpeedrunPlugin;

impl Plugin for SpeedrunPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_timer_ui)
            .add_system(update_timer_ui);
    }
}

#[derive(Component)]
pub struct TimerUI(Stopwatch);

fn spawn_timer_ui(mut commands: Commands, assets: Res<AssetServer>) {
    //FIXME: Global font setting
    let font = assets.load("fonts/pointfree.ttf");

    let parent = (
        NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(80.0), Val::Percent(10.0)),
                align_self: AlignSelf::FlexStart,
                align_items: AlignItems::FlexEnd,
                flex_direction: FlexDirection::Column,
                position_type: PositionType::Absolute,
                position: UiRect::right(Val::Percent(5.0)),
                ..default()
            },
            ..default()
        },
        Name::new("Timer"),
    );
    let timer_text = (
        TextBundle::from_section(
            "000:00",
            TextStyle {
                font,
                font_size: 48.0,
                color: Color::WHITE,
            },
        )
        .with_text_alignment(TextAlignment::Left),
        TimerUI(Stopwatch::default()),
    );
    commands.spawn(parent).with_children(|commands| {
        commands.spawn(timer_text);
    });
}

fn update_timer_ui(mut ui: Query<(&mut Text, &mut TimerUI)>, time: Res<Time>) {
    for (mut text, mut timer) in &mut ui {
        timer.0.tick(time.delta());
        text.sections[0].value = format!("{:0.2}s", timer.0.elapsed().as_secs_f32());
    }
}
