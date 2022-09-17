mod food;
mod snake;
mod utilities;

use bevy::{prelude::*, time::FixedTimestep};
use utilities::RGB;

const TIME_STEP: f32 = 1.0 / 60.0;
const BACKGROUND_COLOR_RGB: (u32, u32, u32) = (255, 240, 167);

fn main() {
    App::new()
        .add_system_set(SystemSet::new().with_run_criteria(FixedTimestep::step(TIME_STEP as f64)))
        .add_startup_system(setup)
        .insert_resource(ClearColor(RGB::new(BACKGROUND_COLOR_RGB)))
        .insert_resource(MovementTimer(Timer::from_seconds(0.3, true)))
        .insert_resource(GameSpeed(0.3))
        .add_system(tick_timer)
        .add_event::<DeathCollisionEvent>()
        .add_plugins(DefaultPlugins)
        .add_plugin(snake::SnakePlugin)
        .add_plugin(food::FoodPlugin)
        .add_system(bevy::window::close_on_esc)
        .add_system(pause_game)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

#[derive(Component)]
pub struct MovementTimer(Timer);

#[derive(Component)]
pub struct GameSpeed(f32);

fn tick_timer(time: Res<Time>, mut timer: ResMut<MovementTimer>) {
    timer.0.tick(time.delta());
}

#[derive(Default)]
struct DeathCollisionEvent;

fn pause_game(mut timer: ResMut<MovementTimer>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if timer.0.paused() {
            timer.0.unpause()
        } else {
            timer.0.pause()
        }
    }
}
