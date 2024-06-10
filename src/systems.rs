use bevy::prelude::*;
use rand::Rng;
use crate::components::{Direction, Food, Position, SnakeHead, SnakeSegment};
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
            direction: Direction::Up
        },
        Position {
            x: 0,
            y: 0
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
        SnakeSegment,
        Position {
            x: 0,
            y: -1
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

pub fn handle_movement_input(keys: Res<Input<KeyCode>>, mut query: Query<&mut SnakeHead>) {
    let mut head = query.iter_mut().next().unwrap();

    if keys.pressed(KeyCode::W) && head.direction != Direction::Down {
        head.direction = Direction::Up;
    } else if keys.pressed(KeyCode::S) && head.direction != Direction::Up {
        head.direction = Direction::Down;
    } else if keys.pressed(KeyCode::A) && head.direction != Direction::Right {
        head.direction = Direction::Left;
    } else if keys.pressed(KeyCode::D) && head.direction != Direction::Left {
        head.direction = Direction::Right;
    }
}

pub fn handle_movement(
    mut query: Query<(&mut SnakeHead, &mut Position), (With<SnakeHead>, Without<SnakeSegment>)>,
    mut segment_query: Query<&mut Position, (With<SnakeSegment>, Without<SnakeHead>)>
) {
    let tuple = query.iter_mut().next().unwrap();
    let head = tuple.0;
    let mut pos = tuple.1;

    let prev_transform = pos.clone();

    match head.direction {
        Direction::Up => {
            pos.y += 1;
        }
        Direction::Down => {
            pos.y -= 1;
        }
        Direction::Left => {
            pos.x -= 1;
        }
        Direction::Right => {
            pos.x += 1;
        }
    }

    let mut prev_translation = prev_transform;
    for mut segment in segment_query.iter_mut() {
        let prev = segment.clone();
        segment.x = prev_translation.x;
        segment.y = prev_translation.y;

        prev_translation = prev;
    }
}

pub fn handle_eat_food(
    mut commands: Commands,
    head_query: Query<&Position, With<SnakeHead>>,
    food_query: Query<(Entity, &Position), With<Food>>
) {
    let head_pos = head_query.single();

    for food in food_query.iter() {
        if head_pos.x == food.1.x && head_pos.y == food.1.y {
            commands.entity(food.0).despawn();
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::GREEN,
                        ..default()
                    },
                    transform: Transform::default().with_scale(Vec3::splat(GRID_SQUARE_SIZE)),
                    ..default()
                },
                SnakeSegment,
                Position {
                    x: -1,
                    y: -1
                }
            ));
        }
    }
}

pub fn check_gameover(
    mut commands: Commands,
    entity_query: Query<Entity, Without<Camera2d>>,
    head_query: Query<&Position, With<SnakeHead>>,
    segments_query: Query<&Position, With<SnakeSegment>>
) {
    let head = head_query.single();
    for segment in segments_query.iter() {
        if head.x == segment.x && head.y == segment.y {
            for entity in entity_query.iter() {
                commands.entity(entity).despawn();
            }

            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::MIDNIGHT_BLUE,
                        ..default()
                    },
                    transform: Transform::default().with_scale(Vec3::splat(GRID_SQUARE_SIZE)),
                    ..default()
                },
                SnakeHead {
                    direction: Direction::Up
                },
                Position {
                    x: 0,
                    y: 0
                }
            ));

            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::GREEN,
                        ..default()
                    },
                    transform: Transform::default().with_scale(Vec3::splat(GRID_SQUARE_SIZE)),
                    ..default()
                },
                SnakeSegment,
                Position {
                    x: 0,
                    y: -1
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
