use bevy::prelude::*;
use rand::Rng;
use crate::components::{Direction, Axial, Food, Position, SnakeHead, SnakeSegment, SnakeTail};
use crate::utils::{GRID_SIZE, SPRITE_SCALE};
use crate::resources::GameTextures;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let game_textures = GameTextures {
        head_up: asset_server.load("head_up.png"),
        head_down: asset_server.load("head_down.png"),
        head_left: asset_server.load("head_left.png"),
        head_right: asset_server.load("head_right.png"),
        body_horizontal: asset_server.load("body_horizontal.png"),
        body_vertical: asset_server.load("body_vertical.png"),
        body_bottomleft: asset_server.load("body_bottomleft.png"),
        body_bottomright: asset_server.load("body_bottomright.png"),
        body_topleft: asset_server.load("body_topleft.png"),
        body_topright: asset_server.load("body_topright.png"),
        tail_up: asset_server.load("tail_up.png"),
        tail_down: asset_server.load("tail_down.png"),
        tail_left: asset_server.load("tail_left.png"),
        tail_right: asset_server.load("tail_right.png"),
        apple: asset_server.load("apple.png"),
    };
    commands.insert_resource(game_textures.clone());

    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        SpriteBundle {
            texture: game_textures.head_up.clone(),
            transform: Transform{
                scale: SPRITE_SCALE,
                ..default()
            },
            ..default()
        },
        SnakeHead {
            direction: Direction::Up,
            texture: game_textures.head_up.clone(),
        },
        Position {
            x: 0,
            y: 2
        }
    ));

    commands.spawn((
        SpriteBundle {
            texture: game_textures.body_vertical.clone(),
            transform: Transform{
                scale: SPRITE_SCALE,
                ..default()
            },
            ..default()
        },
        SnakeSegment {
            texture: game_textures.body_vertical.clone(),
            axial: Axial::Vertical,
        },
        Position {
            x: 0,
            y: 1
        }
    ));

    commands.spawn((
        SpriteBundle {
            texture: game_textures.tail_down.clone(),
            transform: Transform{
                scale: SPRITE_SCALE,
                ..default()
            },
            ..default()
        },
        SnakeTail {
            direction: Direction::Down,
            texture: game_textures.tail_down.clone(),
        },
        Position {
            x: 0,
            y: 0
        }
    ));
}

pub fn spawn_food(mut commands: Commands, game_textures: Res<GameTextures>) {
    commands.spawn((
        SpriteBundle {
            texture: game_textures.apple.clone(),
            transform: Transform{
                scale: SPRITE_SCALE,
                ..default()
            },
            ..default()
        },
        Food,
        Position {
            x: rand::thread_rng().gen_range(0..GRID_SIZE),
            y: rand::thread_rng().gen_range(0..GRID_SIZE)
        }
    ));
}

pub fn handle_movement_input(
    keys: Res<Input<KeyCode>>, 
    mut query: Query<&mut SnakeHead>,
) {
    let mut head = query.iter_mut().next().unwrap();

    if keys.pressed(KeyCode::Up) && head.direction != Direction::Down {
        head.direction = Direction::Up;
    } else if keys.pressed(KeyCode::Down) && head.direction != Direction::Up {
        head.direction = Direction::Down;
    } else if keys.pressed(KeyCode::Left) && head.direction != Direction::Right {
        head.direction = Direction::Left;
    } else if keys.pressed(KeyCode::Right) && head.direction != Direction::Left {
        head.direction = Direction::Right;
    }
}

pub fn handle_movement(
    mut head_query: Query<(&mut SnakeHead, &mut Position, &mut Handle<Image>), With<SnakeHead>>,
    mut segment_query: Query<(&mut SnakeSegment, &mut Position, &mut Handle<Image>), (With<SnakeSegment>, Without<SnakeHead>, Without<SnakeTail>)>,
    mut tail_query: Query<(&mut SnakeTail, &mut Position, &mut Handle<Image>), (With<SnakeTail>, Without<SnakeHead>, Without<SnakeSegment>)>,
    game_textures: Res<GameTextures>
) {
    let (mut head, mut head_pos, mut head_texture) = head_query.iter_mut().next().unwrap();
    let head_prev_transform = head_pos.clone();

    match head.direction {
        Direction::Up => {
            head_pos.y += 1;
            *head_texture = game_textures.head_up.clone();
        }
        Direction::Down => {
            head_pos.y -= 1;
            *head_texture = game_textures.head_down.clone();
        }
        Direction::Left => {
            head_pos.x -= 1;
            *head_texture = game_textures.head_left.clone();
        }
        Direction::Right => {
            head_pos.x += 1;
            *head_texture = game_textures.head_right.clone();
        }
    }

    let mut segment_prev_pos = head_prev_transform;
    let mut segment_next_pos = head_pos.clone();
    for (mut segment, mut segment_pos, mut segment_texture) in segment_query.iter_mut() {
        if segment_pos.x != segment_next_pos.x && segment_pos.y != segment_next_pos.y {
            let seg_relation = ((segment_pos.x - segment_next_pos.x, segment_pos.y - segment_next_pos.y), (segment_pos.x - segment_prev_pos.x, segment_pos.y - segment_prev_pos.y));
            *segment_texture = match seg_relation {
                ((-1, 1), (-1, 0)) | ((1, -1), (0, -1))=> game_textures.body_bottomleft.clone(),
                ((-1, -1), (0, -1)) | ((1, 1), (1, 0))=> game_textures.body_bottomright.clone(),
                ((-1, -1), (-1, 0)) | ((1, 1), (0, 1))=> game_textures.body_topleft.clone(),
                ((-1, 1), (0, 1)) | ((1, -1), (1, 0))=> game_textures.body_topright.clone(),
                _ => segment_texture.clone(),
            };
        } else if segment_pos.x == segment_next_pos.x {
            *segment_texture = game_textures.body_vertical.clone();
        } else if segment_pos.y == segment_next_pos.y {
            *segment_texture = game_textures.body_horizontal.clone();
        }

        let prev = segment_pos.clone();
        segment_pos.x = segment_prev_pos.x;
        segment_pos.y = segment_prev_pos.y;
        segment_prev_pos = prev;
        segment_next_pos = segment_pos.clone();
    }

    let (mut tail, mut tail_pos, mut tail_texture) = tail_query.iter_mut().next().unwrap();
    let tail_pos_sub = ((tail_pos.x - segment_next_pos.x, tail_pos.y - segment_next_pos.y), (tail_pos.x - segment_prev_pos.x, tail_pos.y - segment_prev_pos.y));
    (*tail_texture, tail.direction) = match tail_pos_sub {
        ((-1, -1), (-1, 0)) | ((1, -1), (1, 0)) => (game_textures.tail_down.clone(), Direction::Down),
        ((-1, 1), (-1, 0)) | ((1, 1), (1, 0)) => (game_textures.tail_up.clone(), Direction::Up),
        ((-1, 1), (0, 1)) | ((-1, -1), (0, -1)) => (game_textures.tail_left.clone(), Direction::Left),
        ((1, 1), (0, 1)) | ((1, -1), (0, -1))=> (game_textures.tail_right.clone(), Direction::Right),
        _ => (tail_texture.clone(), tail.direction),
    };
    let tail_prev_translation = segment_prev_pos;
    tail_pos.x = tail_prev_translation.x;
    tail_pos.y = tail_prev_translation.y;
}

pub fn handle_eat_food(
    mut commands: Commands,
    head_query: Query<(&Position, &SnakeHead), With<SnakeHead>>,
    tail_query: Query<(&Position, &SnakeTail), With<SnakeTail>>,
    food_query: Query<(Entity, &Position), With<Food>>,
    game_textures: Res<GameTextures>
) {
    let (head_pos, head_enity) = head_query.single();
    let (tail_pos, tail) = tail_query.single();

    let (segment_texture, segment_axial) = match tail.direction {
        Direction::Up | Direction::Down => (game_textures.body_vertical.clone(), Axial::Vertical),
        Direction::Left | Direction::Right => (game_textures.body_horizontal.clone(), Axial::Horizontal),
    };
    for (food_entity, food_pos) in food_query.iter() {
        if head_pos.x == food_pos.x && head_pos.y == food_pos.y {
            // Despawn the food
            commands.entity(food_entity).despawn();
            commands.spawn((
                SpriteBundle {
                    texture: segment_texture.clone(),
                    transform: Transform{
                        scale: SPRITE_SCALE,
                        ..default()
                    },
                    ..default()
                },
                SnakeSegment {
                    axial: segment_axial,
                    texture: segment_texture.clone(),
                },
                Position {
                    x: tail_pos.x,
                    y: tail_pos.y
                }
            ));
        }
    }
}

pub fn check_gameover(
    mut commands: Commands,
    entity_query: Query<Entity, Without<Camera2d>>,
    head_query: Query<&Position, With<SnakeHead>>,
    segments_query: Query<&Position, With<SnakeSegment>>,
    game_textures: Res<GameTextures>
) {
    let mut is_head_collision = false;
    let head = head_query.single();
    if head.x < 0 || head.x >= GRID_SIZE as i32 || head.y < 0 || head.y >= GRID_SIZE as i32{
        is_head_collision = true;
    }
    for segment in segments_query.iter() {
        if is_head_collision | (head.x == segment.x && head.y == segment.y) {
            for entity in entity_query.iter() {
                commands.entity(entity).despawn();
            }

            commands.spawn((
                SpriteBundle {
                    texture: game_textures.head_up.clone(),
                    transform: Transform{
                        scale: SPRITE_SCALE,
                        ..default()
                    },
                    ..default()
                },
                SnakeHead {
                    direction: Direction::Up,
                    texture: game_textures.head_up.clone()
                },
                Position {
                    x: 0,
                    y: 0
                }
            ));

            commands.spawn((
                SpriteBundle {
                    texture: game_textures.body_vertical.clone(),
                    transform: Transform{
                        scale: SPRITE_SCALE,
                        ..default()
                    },
                    ..default()
                },
                SnakeSegment {
                    texture: game_textures.body_vertical.clone(),
                    axial: Axial::Vertical,
                },
                Position {
                    x: 0,
                    y: -1
                }
            ));

            commands.spawn((
                SpriteBundle {
                    texture: game_textures.tail_down.clone(),
                    transform: Transform{
                        scale: SPRITE_SCALE,
                        ..default()
                    },
                    ..default()
                },
                SnakeTail {
                    direction: Direction::Down,
                    texture: game_textures.tail_down.clone(),
                },
                Position {
                    x: 0,
                    y: -2
                }
            ));
            return;
        }
    }
}

pub fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }
    let window = windows.get_primary().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, GRID_SIZE as f32),
            convert(pos.y as f32, window.height() as f32, GRID_SIZE as f32),
            0.0,
        );
    }
}
