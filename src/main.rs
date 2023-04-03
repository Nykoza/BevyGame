use bevy::{
    prelude::*,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    window::WindowResized,
    window::{CursorGrabMode, PresentMode}
};
use bevy::pbr::wireframe::Wireframe;
use bevy::prelude::KeyCode::D;
use bevy::prelude::system_adapter::new;
use bevy::window::WindowResolution;

const PLAYER_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const WALL_COLOR: Color = Color::rgb(0.3, 0.1, 0.7);
const BOX_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);
const HOLE_COLOR: Color = Color::rgb(0.2, 0.5, 0.5);

const CELL_SIZE: u32 = 25;

const MAP_HEIGHT: u32 = 8;
const MAP_WIDTH: u32 = 8;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_startup_system(window_resize)
        .add_startup_system(spawn_hole)
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_map)
        .add_startup_system(spawn_box)
        .add_startup_system(setup_camera)
        .add_system(player_direction.before(push_boxes_direction).in_base_set(CoreSet::FixedUpdate))
        .add_system(push_boxes_direction.before(push_boxes_checks).after(player_direction).in_base_set(CoreSet::FixedUpdate))
        .add_system(push_boxes_checks.before(boxes_movement).after(push_boxes_direction).in_base_set(CoreSet::FixedUpdate))
        .add_system(boxes_movement.before(player_movement).after(push_boxes_checks).in_base_set(CoreSet::FixedUpdate))
        .add_system(player_movement.after(boxes_movement).in_base_set(CoreSet::FixedUpdate))
        .add_systems((position_translation, size_scaling))
        .add_system(check_win)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Component)]
struct Player {
    direction: Direction,
}

#[derive(Component)]
struct Wall;

#[derive(Component)]
struct Box {
    push_direction: Direction,
}

#[derive(Component)]
struct Hole;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Size {
    width: f32,
    height: f32,
}
#[derive(PartialEq, Copy, Clone)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
    None
}

impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

#[derive(Component)]
struct MovementState {
    has_finished_movement: bool
}

fn spawn_player(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: PLAYER_COLOR,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(10.0, 10.0, 10.0),
                ..default()
            },
            ..default()
        })
        .insert(Player { direction: Direction::None })
        .insert(Position { x: 3, y: 3})
        .insert(Size::square(0.8))
        .insert(MovementState { has_finished_movement: true });
}

fn spawn_box(mut commands: Commands) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: BOX_COLOR,
            ..default()
        },
        transform: Transform {
            scale: Vec3::new(10.0, 10.0, 10.0),
            ..default()
        },
        ..default()
    })
        .insert(Box { push_direction: Direction::None })
        .insert(Position { x: 3, y: 4})
        .insert(Size::square(0.7));

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: BOX_COLOR,
            ..default()
        },
        transform: Transform {
            scale: Vec3::new(10.0, 10.0, 10.0),
            ..default()
        },
        ..default()
    })
        .insert(Box { push_direction: Direction::None })
        .insert(Position { x: 3, y: 5})
        .insert(Size::square(0.7));
}

fn spawn_hole(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: HOLE_COLOR,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(10.0, 10.0, 10.0),
                ..default()
            },
            ..default()
        })
        .insert(Hole)
        .insert(Position { x: 3, y: 6})
        .insert(Size::square(0.3));
}

fn spawn_map(mut commands: Commands) {
    let mut y = 0;

    while y < MAP_HEIGHT {
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: WALL_COLOR,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(10.0, 10.0, 10.0),
                ..default()
            },
            ..default()
        })
            .insert(Wall)
            .insert(Position { x: 0, y: y as i32 })
            .insert(Size::square(0.9));
        y += 1;
    }

    y = 0;
    while y < MAP_HEIGHT {
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: WALL_COLOR,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(10.0, 10.0, 10.0),
                ..default()
            },
            ..default()
        })
            .insert(Wall)
            .insert(Position { x: MAP_WIDTH as i32 - 1, y: y as i32 })
            .insert(Size::square(0.9));
        y += 1;
    }

    let mut x = 0;
    while x < MAP_WIDTH {
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: WALL_COLOR,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(10.0, 10.0, 10.0),
                ..default()
            },
            ..default()
        })
            .insert(Wall)
            .insert(Position { x: x as i32, y: 0 })
            .insert(Size::square(0.9));
        x += 1;
    }

    x = 0;
    while x < MAP_WIDTH {
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: WALL_COLOR,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(10.0, 10.0, 10.0),
                ..default()
            },
            ..default()
        })
            .insert(Wall)
            .insert(Position { x: x as i32, y: MAP_HEIGHT as i32 - 1 })
            .insert(Size::square(0.9));
        x += 1;
    }
}

fn check_player_can_move(obstacles: &mut Query<&Position, (Without<Player>, Without<Hole>)>, new_pos: &Position) -> bool {
    for obstacle_pos in obstacles.iter_mut() {
        if obstacle_pos.x == new_pos.x && obstacle_pos.y == new_pos.y {
            return false;
        }
    }
    true
}

fn player_direction(keyboard_input: Res<Input<KeyCode>>, mut player_query: Query<(&mut Player, &mut MovementState), With<Player>>) {
    for (mut player, mut mov) in player_query.iter_mut() {
        if mov.has_finished_movement {
            if keyboard_input.pressed(KeyCode::Left) {
                player.direction = Direction::Left;
                mov.has_finished_movement = false;
            }
            if keyboard_input.pressed(KeyCode::Right) {
                player.direction = Direction::Right;
                mov.has_finished_movement = false;
            }
            if keyboard_input.pressed(KeyCode::Down) {
                player.direction = Direction::Down;
                mov.has_finished_movement = false;
            }
            if keyboard_input.pressed(KeyCode::Up) {
                player.direction = Direction::Up;
                mov.has_finished_movement = false;
            }
        } else {
            if keyboard_input.any_just_released([KeyCode::Left, KeyCode::Right, KeyCode::Down, KeyCode::Up]) {
                mov.has_finished_movement = true;
            }
        }
    }
}

fn get_new_pos(direction: &Direction, position: &Position) -> Position {
    if *direction == Direction::None {
        return *position;
    } else if *direction == Direction::Left {
        return Position { x: position.x - 1, y: position.y }
    } else if *direction == Direction::Right {
        return Position { x: position.x + 1, y: position.y }
    } else if *direction == Direction::Up {
        return Position { x: position.x, y: position.y + 1 }
    } else if *direction == Direction::Down {
        return Position { x: position.x, y: position.y - 1 }
    }
    return *position;
}

fn player_movement(mut player_query: Query<(&mut Position, &mut Player), With<Player>>,
                    mut obstacles: Query<&Position, (Without<Player>, Without<Hole>)>
) {
    for (mut pos, mut player) in player_query.iter_mut() {
        let player_new_pos = get_new_pos(&player.direction, &pos);
        let can_move = check_player_can_move(&mut obstacles, &player_new_pos);

        player.direction = Direction::None;
        if can_move {
            *pos = player_new_pos;
        }
    }
}

fn boxes_movement(mut boxes: Query<(&mut Position, &mut Box)>) {
    for (mut position, mut _box) in boxes.iter_mut() {
        if _box.push_direction != Direction::None {
            *position = get_new_pos(&_box.push_direction, &position);
            _box.push_direction = Direction::None;
        }
    }
}

fn push_boxes_checks(mut boxes: Query<(&Position, &mut Box), (Without<Player>, Without<Wall>, Without<Hole>)>, obstacles: Query<&Position, (Without<Player>, Without<Hole>)>) {

    for (pos, mut _box) in boxes.iter_mut() {
        if _box.push_direction == Direction::None {
            continue;
        }
        let box_new_pos = get_new_pos(&_box.push_direction, &pos);

        for obstacle_pos in obstacles.iter() {
            if obstacle_pos.x == box_new_pos.x && obstacle_pos.y == box_new_pos.y {
                _box.push_direction = Direction::None;
                return;
            }
        }
    }
}

fn push_boxes_direction(mut boxes: Query<(&mut Position, &mut Box), (With<Box>, Without<Player>, Without<Wall>)>,
                        mut player: Query<(&mut Position, &mut Player), (With<Player>, Without<Box>, Without<Wall>)>
              ) {
    let (mut player_position, mut player) = player.single_mut();

    let new_player_pos = get_new_pos(&player.direction, &player_position);

    for (mut box_position, mut _box) in boxes.iter_mut() {

        if new_player_pos.x == box_position.x && new_player_pos.y == box_position.y {
            _box.push_direction = player.direction;
        }
    }
}

/*
 Win condition
*/

fn is_hole_not_covered(hole_pos: &Position, boxes: &Query<&Position, With<Box>>) -> bool {
    for _box in boxes.iter() {
        if hole_pos.x == _box.x && hole_pos.y == _box.y {
            return false
        }
    }
    true
}

fn check_win(holes: Query<&Position, With<Hole>>, boxes: Query<&Position, With<Box>>) {
    for hole_pos in holes.iter() {
        if is_hole_not_covered(&hole_pos, &boxes) {
            return;
        }
    }

    println!("You won");
}


/*
 Window changes, apply position
 */
fn size_scaling(mut windows: Query<&mut Window>, mut q: Query<(&Size, &mut Transform)>) {
    let mut window = windows.single_mut();

    for (sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width / MAP_WIDTH as f32 * window.width() as f32,
            sprite_size.height / MAP_HEIGHT as f32 * window.height() as f32,
            1.0,
        );
    }
}

fn window_resize(mut windows: Query<&mut Window>) {
    let mut window = windows.single_mut();

    window.title = "Sokoban!".to_string();
    window.resolution = WindowResolution::new(500.0, 500.0);
}

fn position_translation(mut windows: Query<&mut Window>, mut q: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }

    let window = windows.single_mut();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, MAP_WIDTH as f32),
            convert(pos.y as f32, window.height() as f32, MAP_HEIGHT as f32),
            0.0,
        );
    }
}