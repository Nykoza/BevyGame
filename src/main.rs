use bevy::{
    prelude::*,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    window::WindowResized,
    window::{CursorGrabMode, PresentMode}
};
use bevy::prelude::system_adapter::new;
use bevy::window::WindowResolution;

const PLAYER_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const WALL_COLOR: Color = Color::rgb(0.3, 0.1, 0.7);
const BOX_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);

const CELL_SIZE: u32 = 25;

const MAP_HEIGHT: u32 = 8;
const MAP_WIDTH: u32 = 8;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_startup_system(window_resize)
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_map)
        .add_startup_system(spawn_box)
        .add_startup_system(setup_camera)
        .add_system(player_movement)
        .add_systems((position_translation, size_scaling))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Wall;

#[derive(Component)]
struct Box;

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
        .insert(Player)
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
        .insert(Box)
        .insert(Position { x: 3, y: 4})
        .insert(Size::square(0.7));
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

fn check_can_move(walls: &mut Query<(Entity, &Position), (With<Wall>, Without<Player>)>, new_pos: &Position) -> bool {
    for (e, wall_pos) in walls.iter_mut() {
        if wall_pos.x == new_pos.x && wall_pos.y == new_pos.y {
            return false;
        }
    }
    true
}

fn player_movement(keyboard_input: Res<Input<KeyCode>>,
                    mut player: Query<(&mut Position, &mut MovementState), With<Player>>,
                    mut walls:  Query<(Entity, &Position), (With<Wall>, Without<Player>)>
) {
    for (mut pos, mut mov) in player.iter_mut() {
        if mov.has_finished_movement {
            if keyboard_input.pressed(KeyCode::Left) {
                let new_pos = Position {x: pos.x - 1, y: pos.y };
                let can_move = check_can_move(&mut walls, &new_pos);

                if can_move {
                    pos.x = new_pos.x;
                    pos.y = new_pos.y;
                    mov.has_finished_movement = false;
                }
            }
            if keyboard_input.pressed(KeyCode::Right) {
                let new_pos = Position {x: pos.x + 1, y: pos.y };
                let can_move = check_can_move(&mut walls, &new_pos);

                if can_move {
                    pos.x = new_pos.x;
                    pos.y = new_pos.y;
                    mov.has_finished_movement = false;
                }
            }
            if keyboard_input.pressed(KeyCode::Down) {
                let new_pos = Position {x: pos.x, y: pos.y - 1 };
                let can_move = check_can_move(&mut walls, &new_pos);

                if can_move {
                    pos.x = new_pos.x;
                    pos.y = new_pos.y;
                    mov.has_finished_movement = false;
                }
            }
            if keyboard_input.pressed(KeyCode::Up) {
                let new_pos = Position {x: pos.x, y: pos.y + 1 };
                let can_move = check_can_move(&mut walls, &new_pos);

                if can_move {
                    pos.x = new_pos.x;
                    pos.y = new_pos.y;
                    mov.has_finished_movement = false;
                }
            }
        } else {
            if keyboard_input.any_just_released([KeyCode::Left, KeyCode::Right, KeyCode::Down, KeyCode::Up]) {
                mov.has_finished_movement = true;
            }
        }
    }
}

fn size_scaling(mut windows: Query<&mut Window>, mut q: Query<(&Size, &mut Transform)>){
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