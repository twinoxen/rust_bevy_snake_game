mod food;
mod snake;
mod utilities;

use bevy::{prelude::*};
use utilities::RGB;

const BACKGROUND_COLOR_RGB: (u32, u32, u32) = (255, 240, 167);

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Snake!".to_string(),
            width: 1000.0,
            height: 900.0,
            ..default()
        })
        .add_startup_system(setup)
        .insert_resource(ClearColor(RGB::new(BACKGROUND_COLOR_RGB)))
        .add_event::<DeathCollisionEvent>()
        .add_plugins(DefaultPlugins)
        .add_plugin(snake::SnakePlugin)
        .add_plugin(food::FoodPlugin)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}



#[derive(Default)]
struct DeathCollisionEvent;
