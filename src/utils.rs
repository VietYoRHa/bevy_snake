use bevy::math::Vec3;

pub const TIME_STEP: f64 = 1. / 9.;
pub const WINDOW_WIDTH: f32 = 700.;
pub const WINDOW_HEIGHT: f32 = 700.;
pub const GRID_SIZE: i32 = 28;
pub const GRID_SQUARE_SIZE: f32 = WINDOW_WIDTH / GRID_SIZE as f32;
pub const SPRITE_SCALE: Vec3 = Vec3::splat(GRID_SQUARE_SIZE / 40.0);
