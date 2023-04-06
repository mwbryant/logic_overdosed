use crate::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, _app: &mut App) {}
}

#[derive(Component)]
pub struct MapEntity;

fn spawn_exit(commands: &mut Commands, translation: Vec2) {
    commands.spawn((
        Collider::cuboid(32.0, 32.0),
        Sensor,
        Door,
        MapEntity,
        Name::new("Exit"),
        SpatialBundle::from_transform(Transform::from_translation(translation.extend(0.0))),
    ));
}

pub fn load_map(
    commands: &mut Commands,
    assets: &Res<AssetServer>,
    progression: &StoryProgression,
) {
    let map = &progression.levels[progression.current_map];
    //let file = File::open("assets/maps/test_room.map").unwrap();
    //let reader = BufReader::new(map);

    let mut lines: Vec<String> = map.lines().map(|l| l.to_string()).collect();

    let (map_file, lines) = lines.split_first_mut().unwrap();

    lines.reverse();

    //TODO cleanup this gross logic
    let mut max_x = 0;
    let max_y = lines.len();
    let mut boxes_to_spawn = Vec::new();

    for (y, line) in lines.iter().enumerate() {
        max_x = line.len();
        let mut in_run = false;
        let mut run_start = 0;
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                //Start a new run
                if !in_run {
                    in_run = true;
                    run_start = x;
                }
            } else {
                //Run ends
                if in_run {
                    in_run = false;
                    boxes_to_spawn.push((run_start, y, x - run_start));
                }
            }
            if c == 'E' {
                spawn_exit(
                    commands,
                    Vec2::new((x + 1) as f32 * 32.0, (y + 1) as f32 * 32.0),
                );
            }
        }
        //Cleanup ongoing run
        if in_run {
            boxes_to_spawn.push((run_start, y, line.chars().count() - run_start));
        }
    }

    for (x, y, width) in boxes_to_spawn {
        //FIXME can connect y values too
        spawn_hit_box(
            commands,
            Vec2::new(width as f32, 1.0),
            Vec2::new(x as f32, y as f32),
        );
    }

    //Left wall
    spawn_hit_box(commands, Vec2::new(1.0, 30.0), Vec2::new(-1.0, 0.0));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite { ..default() },
            texture: assets.load(map_file.to_string()),
            transform: Transform::from_xyz(
                (32.0 * max_x as f32) / 2.0,
                (32.0 * max_y as f32) / 2.0,
                BACKGROUND_Z,
            ),
            ..default()
        },
        MapEntity,
        Name::new("Background"),
    ));
}

fn spawn_hit_box(commands: &mut Commands, block_size: Vec2, bottom_left_position: Vec2) {
    let half_size = block_size * Vec2::splat(16.0);
    commands
        .spawn(Collider::cuboid(half_size.x, half_size.y))
        .insert(TransformBundle::from(Transform::from_xyz(
            bottom_left_position.x * 32.0 + half_size.x,
            bottom_left_position.y * 32.0 + half_size.y,
            0.0,
        )))
        .insert(MapEntity)
        .insert(Name::new("Hitbox"));
}
