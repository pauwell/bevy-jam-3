// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::{
    prelude::{Query, Rect, Res, Transform, Vec2, With, Without},
    time::Time,
};
use rand::{thread_rng, Rng};

use crate::plugins::{
    enemy::{
        components::{Enemy, EnemyChangeDirectionTimer, EnemyDirection, EnemySize, EnemyWalkSpeed},
        states::EnemyState,
    },
    player::{components::Player, states::PlayerState},
    tilemap::components::TilemapColliders,
};

pub fn enemy_random_movement(
    mut enemy_query: Query<
        (
            &mut Transform,
            &mut EnemyChangeDirectionTimer,
            &mut EnemyDirection,
            &EnemySize,
            &EnemyWalkSpeed,
            &EnemyState,
        ),
        With<Enemy>,
    >,
    player_query: Query<(&Transform, &PlayerState), (With<Player>, Without<Enemy>)>,
    tilemap_collider_query: Query<&TilemapColliders>,
    delta: Res<Time>,
) {
    if enemy_query.is_empty() || player_query.is_empty() {
        return;
    }
    let (player_transform, player_state) = player_query
        .get_single()
        .expect("0 or more than 1 player found.");

    if *player_state == PlayerState::Talk {
        return;
    }

    let mut move_vector;

    for (
        mut enemy_transform,
        mut enemy_timer,
        mut enemy_direction,
        enemy_size,
        enemy_walk_speed,
        enemy_state,
    ) in enemy_query.iter_mut()
    {
        if *enemy_state == EnemyState::Stunned {
            continue;
        }

        if player_transform
            .translation
            .distance(enemy_transform.translation)
            < 85.0
        {
            // Follow the player.
            move_vector = (player_transform.translation - enemy_transform.translation)
                .truncate()
                .normalize();
        } else {
            // New random direction.
            enemy_timer.0.tick(delta.delta());
            if enemy_timer.0.just_finished() {
                let mut rng = thread_rng();
                *enemy_direction = match rng.gen_range(0..3) {
                    0 => EnemyDirection::Up,
                    1 => EnemyDirection::Down,
                    2 => EnemyDirection::Left,
                    _ => EnemyDirection::Right,
                };
            }

            move_vector = match *enemy_direction {
                EnemyDirection::Up => Vec2::new(0.0, 1.0),
                EnemyDirection::Down => Vec2::new(0.0, -1.0),
                EnemyDirection::Left => Vec2::new(-1.0, 0.0),
                EnemyDirection::Right => Vec2::new(1.0, 0.0),
            };
        }

        let mut enemy_velocity = move_vector * enemy_walk_speed.0;

        // Get new position on each axis.
        let mut new_position_horizontal = enemy_transform.translation;
        new_position_horizontal.x += enemy_velocity.x * delta.delta_seconds();
        let mut new_position_vertical = enemy_transform.translation;
        new_position_vertical.y += enemy_velocity.y * delta.delta_seconds();

        // Calculate separate bounding-boxes for each axis-movement.
        let new_enemy_rect_horizontal =
            Rect::from_center_size(new_position_horizontal.truncate(), enemy_size.0 / 2.0);
        let new_enemy_rect_vertical =
            Rect::from_center_size(new_position_vertical.truncate(), enemy_size.0 / 2.0);

        // Check for collision with `TilemapColliders`.
        if let Ok(tilemap_colliders) = tilemap_collider_query.get_single() {
            for tilemap_collider_rect in tilemap_colliders.0.iter() {
                if !tilemap_collider_rect
                    .intersect(new_enemy_rect_horizontal)
                    .is_empty()
                {
                    enemy_velocity.x = 0.0;
                }

                if !tilemap_collider_rect
                    .intersect(new_enemy_rect_vertical)
                    .is_empty()
                {
                    enemy_velocity.y = 0.0;
                }
            }
        }

        enemy_transform.translation += enemy_velocity.extend(0.0) * delta.delta_seconds();
    }
}
