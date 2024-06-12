use bevy::prelude::*;
use rand::Rng;
use crate::components::{Direction, Axial, Food, Position, SnakeHead, SnakeSegment};
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
            is_tail: false,
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
        SnakeSegment {
            is_tail: true,
            texture: game_textures.tail_down.clone(),
            axial: Axial::Vertical,
        },
        Position {
            x: 0,
            y: -2
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
    mut query: Query<(&mut SnakeHead, &mut Position, &mut Handle<Image>), With<SnakeHead>>,
    mut segment_query: Query<(&mut Position, &mut SnakeSegment), (Without<SnakeHead>, With<SnakeSegment>)>,
    game_textures: Res<GameTextures>
) {
    let (mut head, mut pos, mut texture) = query.iter_mut().next().unwrap();
    let prev_transform = pos.clone();

    match head.direction {
        Direction::Up => {
            pos.y += 1;
            *texture = game_textures.head_up.clone();
        }
        Direction::Down => {
            pos.y -= 1;
            *texture = game_textures.head_down.clone();
        }
        Direction::Left => {
            pos.x -= 1;
            *texture = game_textures.head_left.clone();
        }
        Direction::Right => {
            pos.x += 1;
            *texture = game_textures.head_right.clone();
        }
    }

    let mut prev_translation = prev_transform;
    let mut prev_axial = match head.direction {
        Direction::Up | Direction::Down => Axial::Vertical,
        Direction::Left | Direction::Right => Axial::Horizontal,
    };

    for (mut segment_pos, mut segment) in segment_query.iter_mut() {
        let prev = segment_pos.clone();
        segment_pos.x = prev_translation.x;
        segment_pos.y = prev_translation.y;

        let new_axial = if segment.is_tail {
            prev_axial // Keep the axial of the tail the same
        } else {
            let current_axial = match head.direction {
                Direction::Up | Direction::Down => Axial::Vertical,
                Direction::Left | Direction::Right => Axial::Horizontal,
            };

            match (prev_axial, current_axial) {
                (Axial::Vertical, Axial::Horizontal) => {
                    // Change texture to corner piece
                    if head.direction == Direction::Right {
                        segment.texture = game_textures.body_bottomright.clone();
                    } else {
                        segment.texture = game_textures.body_bottomleft.clone();
                    }
                },
                (Axial::Horizontal, Axial::Vertical) => {
                    // Change texture to corner piece
                    if head.direction == Direction::Up {
                        segment.texture = game_textures.body_topright.clone();
                    } else {
                        segment.texture = game_textures.body_topleft.clone();
                    }
                },
                _ => {},
            }

            current_axial
        };

        prev_axial = new_axial;
        prev_translation = prev;
    }
}

pub fn handle_eat_food(
    mut commands: Commands,
    head_query: Query<&Position, With<SnakeHead>>,
    segment_query: Query<(Entity, &Position, &mut SnakeSegment), Without<SnakeHead>>,
    food_query: Query<(Entity, &Position), With<Food>>,
    game_textures: Res<GameTextures>
) {
    let head_pos = head_query.single();

    for (food_entity, food_pos) in food_query.iter() {
        if head_pos.x == food_pos.x && head_pos.y == food_pos.y {
            // Despawn the food
            commands.entity(food_entity).despawn();

            // Find the current tail segment
            let mut tail_entity = None;
            let mut tail_position = None;
            let mut tail_segment = None;

            for (entity, pos, segment) in segment_query.iter() {
                if segment.is_tail {
                    tail_entity = Some(entity);
                    tail_position = Some(*pos);
                    tail_segment = Some(segment.clone());
                    break;
                }
            }

            // Update the current tail segment to a body segment
            if let Some(tail_entity) = tail_entity {
                commands.entity(tail_entity).insert((
                    match tail_segment.as_ref().unwrap().axial {
                        Axial::Vertical => game_textures.body_vertical.clone(),
                        Axial::Horizontal => game_textures.body_horizontal.clone(),
                    },
                    SnakeSegment {
                        is_tail: false,
                        texture: Handle::default(),
                        axial: tail_segment.unwrap().axial,
                    }
                ));
            }

            // Add a new tail segment at the old tail position
            if let Some(tail_pos) = tail_position {
                let old_tail_texture = tail_segment.unwrap().texture.clone();
                commands.spawn((
                    SpriteBundle {
                        texture: old_tail_texture.clone(), // Adjust the texture if needed
                        transform: Transform{
                            scale: SPRITE_SCALE,
                            ..default()
                        },
                        ..default()
                    },
                    SnakeSegment {
                        is_tail: true,
                        texture: old_tail_texture,
                        axial: tail_segment.unwrap().axial,
                    },
                    Position {
                        x: tail_pos.x,
                        y: tail_pos.y,
                    }
                ));
            }
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
                    is_tail: false,
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
                SnakeSegment {
                    is_tail: true,
                    texture: game_textures.tail_down.clone(),
                    axial: Axial::Vertical,
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
