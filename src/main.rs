use bevy::app::App;
use bevy::DefaultPlugins;
use bevy::prelude::*;
use bevy::time::FixedTimestep;
use bevy::window::{PresentMode, WindowCloseRequested};


mod components;
mod resources;
mod systems;
mod utils;

use systems::{handle_movement_input, handle_movement, handle_eat_food, check_gameover, position_translation, setup, spawn_food};
use crate::utils::{WINDOW_WIDTH, WINDOW_HEIGHT, TIME_STEP};

fn handle_window_close_event(
    mut event_reader: EventReader<WindowCloseRequested>,
) {
    for _event in event_reader.iter() {
        std::process::exit(0);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Snake Xenzia".to_string(),
                width: WINDOW_WIDTH,
                height: WINDOW_HEIGHT,
                present_mode: PresentMode::AutoVsync,
                resizable: false,
                ..default()
            },
            ..default()
        }))
        .add_system(handle_movement_input)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP))
                .with_system(handle_movement)
                .with_system(handle_eat_food.after(handle_movement))
                .with_system(check_gameover.after(handle_movement))
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(2.))
                .with_system(spawn_food)
        )
        .add_startup_system(setup)
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(position_translation)
        )
        .insert_resource(ClearColor(Color::rgb(0.686, 0.843, 0.275)))
        .add_system(handle_window_close_event)
        .run();
}
