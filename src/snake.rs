use bevy::prelude::*;

use crate::{
    food::EatCollisionEvent,
    utilities::{self, RGB},
    DeathCollisionEvent, GameSpeed, MovementTimer,
};

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_snake)
            .add_system(bound)
            .add_system(move_snake.label("move_snake"))
            .add_system(reset_snake)
            .add_system(grow.after("move_snake"))
            .add_system(check_run_into_self_collision)
            .add_system(eat_and_speed_up);
    }
}

const SNAKE_COLOR: (u32, u32, u32) = (204, 76, 32);
const TAIL_COLOR: (u32, u32, u32) = (136, 50, 21);
const SNAKE_HEAD_SIZE: f32 = 50.0;

#[derive(Component)]
pub struct SnakeHead {
    direction: Direction,
}

#[derive(Component)]
pub struct Tail {
    direction: Direction,
}

#[derive(Component, Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn spawn_snake(mut commands: Commands) {
    let snake_head_size: utilities::Size = utilities::Size::square(SNAKE_HEAD_SIZE);

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: RGB::new(SNAKE_COLOR),
                custom_size: Some(Vec2::new(snake_head_size.width, snake_head_size.height)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        })
        .insert(SnakeHead {
            direction: Direction::Down,
        })
        .insert(snake_head_size);
}

fn move_snake(
    timer: Res<MovementTimer>,
    keyboard_input: Res<Input<KeyCode>>,
    mut snake_head_query: Query<(&mut SnakeHead, &utilities::Size, &mut Transform)>,
    mut tail_query: Query<(&mut Tail, &utilities::Size, &mut Transform), Without<SnakeHead>>,
) {
    for (mut snake, size, mut transform) in snake_head_query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            snake.direction = Direction::Left;
        }

        if keyboard_input.pressed(KeyCode::Right) {
            snake.direction = Direction::Right;
        }

        if keyboard_input.pressed(KeyCode::Down) {
            snake.direction = Direction::Down;
        }

        if keyboard_input.pressed(KeyCode::Up) {
            snake.direction = Direction::Up;
        }

        if !timer.0.finished() {
            return;
        }

        let mut direction_x = 0.0;
        let mut direction_y = 0.0;

        match snake.direction {
            Direction::Up => direction_y = 1.,
            Direction::Down => direction_y = -1.,
            Direction::Left => direction_x = -1.,
            Direction::Right => direction_x = 1.,
        }

        transform.translation.x += size.width * direction_x;
        transform.translation.y += size.height * direction_y;

        let mut last_direction = snake.direction.to_owned();

        for (mut tail, tail_size, mut tail_transform) in tail_query.iter_mut() {
            let mut direction_x = 0.0;
            let mut direction_y = 0.0;

            match tail.direction {
                Direction::Up => direction_y = 1.,
                Direction::Down => direction_y = -1.,
                Direction::Left => direction_x = -1.,
                Direction::Right => direction_x = 1.,
            }

            tail_transform.translation.x += tail_size.width * direction_x;
            tail_transform.translation.y += tail_size.height * direction_y;

            let new_direction = tail.direction.to_owned();
            tail.direction = last_direction.to_owned();
            last_direction = new_direction.to_owned();
        }
    }
}

fn grow(
    eat_collision_event: EventReader<EatCollisionEvent>,
    mut commands: Commands,
    snake_query: Query<(&SnakeHead, &utilities::Size, &Transform)>,
    tail_query: Query<(&Tail, &utilities::Size, &mut Transform), Without<SnakeHead>>,
) {
    if !eat_collision_event.is_empty() {
        eat_collision_event.clear();

        let snake_transforms = snake_query.iter().map(|(_, _, transform)| transform);
        let tail_transforms = tail_query.iter().map(|(_, _, transform)| transform);

        if let Some(last_transforms) = snake_transforms.chain(tail_transforms).last() {
            let mut tail_placement_x = last_transforms.translation.x;
            let mut tail_placement_y = last_transforms.translation.y;

            let (snake_head, snake_head_size, _) = snake_query.single();

            let mut tail_direction = snake_head.direction.to_owned();

            if let Some((last_tail, tail_size, _)) = tail_query.into_iter().last() {
                tail_direction = last_tail.direction.clone();

                match last_tail.direction {
                    Direction::Up => tail_placement_y -= tail_size.height,
                    Direction::Down => tail_placement_y += tail_size.height,
                    Direction::Left => tail_placement_x += tail_size.width,
                    Direction::Right => tail_placement_x -= tail_size.width,
                };
            } else {
                match snake_head.direction {
                    Direction::Up => tail_placement_y -= snake_head_size.height,
                    Direction::Down => tail_placement_y += snake_head_size.height,
                    Direction::Left => tail_placement_x += snake_head_size.width,
                    Direction::Right => tail_placement_x -= snake_head_size.width,
                };
            }

            let tail_size: utilities::Size = utilities::Size::square(SNAKE_HEAD_SIZE);

            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: RGB::new(TAIL_COLOR),
                        custom_size: Some(Vec2::new(tail_size.width, tail_size.height)),
                        ..default()
                    },
                    transform: Transform::from_xyz(tail_placement_x, tail_placement_y, 0.0),
                    ..default()
                })
                .insert(Tail {
                    direction: tail_direction.to_owned(),
                })
                .insert(tail_size);
        }
    }
}

fn eat_and_speed_up(
    eat_collision_event: EventReader<EatCollisionEvent>,
    mut movement_timer: ResMut<MovementTimer>,
    mut game_speed: ResMut<GameSpeed>,
) {
    if !eat_collision_event.is_empty() {
        eat_collision_event.clear();

        if game_speed.0 > 0.1 {
            game_speed.0 -= 0.01;

            movement_timer.0 = Timer::from_seconds(game_speed.0, true);
        }
    }
}

fn bound(
    windows: ResMut<Windows>,
    mut query: Query<(&utilities::Size, &mut Transform), With<SnakeHead>>,
    mut collision_events: EventWriter<DeathCollisionEvent>,
) {
    let primary_window = windows.get_primary().unwrap();
    let window_top_limit = primary_window.height() * 0.5;
    let window_bottom_limit = primary_window.height() * 0.5 * -1.0;
    let window_left_limit = primary_window.width() * 0.5;
    let window_right_limit = primary_window.width() * 0.5 * -1.0;

    for (size, mut transform) in query.iter_mut() {
        let top = window_top_limit - { size.height * 0.5 };
        let bottom = window_bottom_limit + { size.height * 0.5 };
        let left = window_right_limit + { size.width * 0.5 };
        let right = window_left_limit - { size.width * 0.5 };

        transform.translation.x = transform.translation.x.clamp(left, right);
        transform.translation.y = transform.translation.y.clamp(bottom, top);

        if transform.translation.x <= left
            || transform.translation.x >= right
            || transform.translation.y <= bottom
            || transform.translation.y >= top
        {
            collision_events.send_default();
        }
    }
}

fn check_run_into_self_collision(
    mut collision_events: EventWriter<DeathCollisionEvent>,
    snake_query: Query<(&SnakeHead, &utilities::Size, &Transform)>,
    tail_query: Query<(&Tail, &utilities::Size, &mut Transform), Without<SnakeHead>>,
) {
    for (_, snake_size, snake) in snake_query.into_iter() {
        for (_, tail_size, tail) in tail_query.into_iter() {
            if bevy::sprite::collide_aabb::collide(
                snake.translation,
                Vec2::new(snake_size.width, snake_size.height),
                tail.translation,
                Vec2::new(tail_size.width, tail_size.height),
            )
            .is_some()
            {
                collision_events.send_default();
            }
        }
    }
}

fn reset_snake(
    collision_events: EventReader<DeathCollisionEvent>,
    query: Query<Entity, With<utilities::Size>>,
    mut commands: Commands,
    mut movement_timer: ResMut<MovementTimer>,
    mut game_speed: ResMut<GameSpeed>,
) {
    if !collision_events.is_empty() {
        for snake in &query {
            commands.entity(snake).despawn();
        }

        game_speed.0 = 0.3;
        movement_timer.0 = Timer::from_seconds(game_speed.0, true);

        spawn_snake(commands);
    }
}
