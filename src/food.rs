use crate::{
    snake::SnakeHead,
    utilities::{self, RGB},
};
use bevy::{ecs::query::QuerySingleError, prelude::*};
use rand::Rng;

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_food)
            .add_event::<EatCollisionEvent>()
            .add_system(detect_eat_collision.label("detect_eat_collision"))
            .add_system(watch_for_eat.after("detect_eat_collision"));
    }
}

#[derive(Component)]
struct Food;

#[derive(Default)]
pub struct EatCollisionEvent;

const FOOD_COLOR: (u32, u32, u32) = (69, 167, 129);
const FOOD_SIZE: f32 = 50.0;

fn spawn_food(windows: ResMut<Windows>, mut commands: Commands, query: Query<&Food>) {
    if !query.is_empty() {
        return;
    }

    let primary_window = windows.get_primary().unwrap();

    let food_size: utilities::Size = utilities::Size::square(FOOD_SIZE);
    let mut rng = rand::thread_rng();
    let random_x: f32 = rng.gen_range(
        (primary_window.width() / 2. - food_size.width) * -1.,
        primary_window.width() / 2. - food_size.width,
    );
    let random_y: f32 = rng.gen_range(
        (primary_window.height() / 2. - food_size.height) * -1.,
        primary_window.height() / 2. - food_size.height,
    );

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: RGB::new(FOOD_COLOR),
                custom_size: Some(Vec2::new(food_size.width, food_size.height)),
                ..default()
            },
            transform: Transform::from_xyz(
                (random_x / food_size.width).round() * food_size.width,
                (random_y / food_size.height).round() * food_size.height,
                0.0,
            ),
            ..default()
        })
        .insert(Food)
        .insert(food_size);
}

fn detect_eat_collision(
    snake_head_query: Query<(&utilities::Size, &Transform), With<SnakeHead>>,
    food_query: Query<(&utilities::Size, &Transform), With<Food>>,
    mut eat_collision_event: EventWriter<EatCollisionEvent>,
) {
    match snake_head_query.get_single() {
        Ok((snake_size, snake)) => {
            if let Some(last) = food_query.iter().last() {
                let (food_size, food) = last;
                if bevy::sprite::collide_aabb::collide(
                    snake.translation,
                    Vec2::new(snake_size.width, snake_size.height),
                    food.translation,
                    Vec2::new(food_size.width, food_size.height),
                )
                .is_some()
                {
                    eat_collision_event.send_default();
                }
            }
        }
        Err(QuerySingleError::NoEntities(_)) => {
            println!("Error: There is no player!");
        }
        Err(QuerySingleError::MultipleEntities(_)) => {
            println!("Error: There is more than one player!");
        }
    }
}

fn watch_for_eat(
    eat_collision_event: EventReader<EatCollisionEvent>,
    entity_query: Query<Entity, With<Food>>,
    mut commands: Commands,
) {
    if !eat_collision_event.is_empty() {
        eat_collision_event.clear();

        for entity in &entity_query {
            commands.entity(entity).despawn();
        }
    }
}
