use bevy::prelude::*;
use rand::Rng;
use crate::components::{Direction, Axial, Food, Position, SnakeHead, SnakeSegment, SnakeTail};
use crate::utils::{GRID_SIZE, GRID_SQUARE_SIZE, SPRITE_SCALE};
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
    mut tail_query: Query<(&mut Position, &mut Handle<Image>), (With<SnakeTail>, Without<SnakeHead>, Without<SnakeSegment>)>,
    game_textures: Res<GameTextures>
) {
    let (mut head, mut head_pos, mut head_texture) = head_query.iter_mut().next().unwrap();
    let head_prev_transform = head_pos.clone();
    let mut head_axial = Axial::Vertical;

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
            head_axial = Axial::Horizontal;
        }
        Direction::Right => {
            head_pos.x += 1;
            *head_texture = game_textures.head_right.clone();
            head_axial = Axial::Horizontal;
        }
    }

    let mut segment_prev_translation = head_prev_transform;
    for (mut segment, mut segment_pos, mut segment_texture) in segment_query.iter_mut() {
        match head_axial {
            Axial::Vertical => {
                segment.axial = Axial::Vertical;
                *segment_texture = game_textures.body_vertical.clone();
            }
            Axial::Horizontal => {
                segment.axial = Axial::Horizontal;
                *segment_texture = game_textures.body_horizontal.clone();
            }
        };

        let prev = segment_pos.clone();
        segment_pos.x = segment_prev_translation.x;
        segment_pos.y = segment_prev_translation.y;
        segment_prev_translation = prev;
    }

    let (mut tail_postion, mut tail_texture) = tail_query.iter_mut().next().unwrap();
    let tail_pos_sub = (tail_postion.x - segment_prev_translation.x, tail_postion.y - segment_prev_translation.y);
    *tail_texture = match tail_pos_sub {
        (0, -1) => game_textures.tail_down.clone(),
        (0, 1) => game_textures.tail_up.clone(),
        (-1, 0) => game_textures.tail_left.clone(),
        (1, 0) => game_textures.tail_right.clone(),
        _ => tail_texture.clone(),
    };
    let tail_prev_translation = segment_prev_translation;
    tail_postion.x = tail_prev_translation.x;
    tail_postion.y = tail_prev_translation.y;
}

pub fn handle_eat_food(
    mut commands: Commands,
    head_query: Query<&Position, With<SnakeHead>>,
    tail_query: Query<(&Position, &SnakeTail), With<SnakeTail>>,
    food_query: Query<(Entity, &Position), With<Food>>,
    game_textures: Res<GameTextures>
) {
    let head_pos = head_query.single();
    let (tail_pos, tail_enity) = tail_query.single();

    let (segment_texture, segment_axial) = match tail_enity.direction {
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
    let head = head_query.single();
    for segment in segments_query.iter() {
        if head.x == segment.x && head.y == segment.y {
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
