use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Axial {
    Vertical,
    Horizontal,
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct SnakeHead {
    pub direction: Direction,
    pub texture: Handle<Image>,
}

#[derive(Component)]
pub struct SnakeSegment {
    pub axial: Axial,
    pub texture: Handle<Image>,
}

#[derive(Component)]
pub struct SnakeTail {
    pub direction: Direction,
    pub texture: Handle<Image>,
}

#[derive(Component)]
pub struct Food;
