// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::{
    prelude::{Input, KeyCode, Query, Rect, Res, Transform, Vec2, With, Without},
    time::Time,
};

use crate::plugins::{
    npc::components::{Npc, NpcSize},
    player::{
        components::{
            Player, PlayerDebuffSlowWalk, PlayerDirection, PlayerSize, PlayerVelocity,
            PlayerWalkSpeed,
        },
        states::PlayerState,
    },
    tilemap::components::TilemapColliders,
};

pub fn player_movement_input(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<(&mut PlayerState, &mut PlayerDirection), With<Player>>,
) {
    if query.is_empty() {
        return;
    }

    let (mut player_state, mut player_direction) = query
        .get_single_mut()
        .expect("0 or more than 1 `Player` found.");

    if *player_state != PlayerState::Idle && *player_state != PlayerState::Walk {
        return;
    }

    const ARROW_KEYS: [KeyCode; 4] = [KeyCode::Up, KeyCode::Down, KeyCode::Left, KeyCode::Right];

    if keyboard.any_pressed(ARROW_KEYS) && *player_state != PlayerState::Walk {
        *player_state = PlayerState::Walk;
    } else if keyboard.any_just_released(ARROW_KEYS) && !keyboard.any_pressed(ARROW_KEYS) {
        *player_state = PlayerState::Idle;
    }

    let mut new_player_direction = *player_direction;
    if keyboard.pressed(KeyCode::Left) {
        new_player_direction = PlayerDirection::Left;
    } else if keyboard.pressed(KeyCode::Right) {
        new_player_direction = PlayerDirection::Right;
    }
    if keyboard.pressed(KeyCode::Up) {
        if keyboard.pressed(KeyCode::Left) {
            new_player_direction = PlayerDirection::UpLeft;
        } else if keyboard.pressed(KeyCode::Right) {
            new_player_direction = PlayerDirection::UpRight;
        } else {
            new_player_direction = PlayerDirection::Up;
        }
    } else if keyboard.pressed(KeyCode::Down) {
        if keyboard.pressed(KeyCode::Left) {
            new_player_direction = PlayerDirection::DownLeft;
        } else if keyboard.pressed(KeyCode::Right) {
            new_player_direction = PlayerDirection::DownRight;
        } else {
            new_player_direction = PlayerDirection::Down;
        }
    }

    if new_player_direction != *player_direction {
        *player_direction = new_player_direction;
    }
}

pub fn player_movement(
    mut player_query: Query<
        (
            &mut Transform,
            &mut PlayerVelocity,
            &PlayerState,
            &PlayerDirection,
            &PlayerWalkSpeed,
            &PlayerSize,
        ),
        With<Player>,
    >,
    npc_query: Query<(&Transform, &NpcSize), (With<Npc>, Without<Player>)>,
    debuff_slow_walk_query: Query<&PlayerDebuffSlowWalk>,
    tilemap_collider_query: Query<&TilemapColliders>,
    delta: Res<Time>,
) {
    let (
        mut player_transform,
        mut player_velocity,
        player_state,
        player_direction,
        player_walk_speed,
        player_size,
    ) = player_query
        .get_single_mut()
        .expect("0 or more than 1 `Player` found.");

    let debuff_slow_walk = !debuff_slow_walk_query.is_empty();

    if *player_state == PlayerState::Walk {
        let move_vector = match player_direction {
            PlayerDirection::Up => Vec2::new(0.0, 1.0),
            PlayerDirection::Down => Vec2::new(0.0, -1.0),
            PlayerDirection::Left => Vec2::new(-1.0, 0.0),
            PlayerDirection::Right => Vec2::new(1.0, 0.0),
            PlayerDirection::UpLeft => Vec2::new(-1.0, 1.0).normalize(),
            PlayerDirection::UpRight => Vec2::new(1.0, 1.0).normalize(),
            PlayerDirection::DownLeft => Vec2::new(-1.0, -1.0).normalize(),
            PlayerDirection::DownRight => Vec2::new(1.0, -1.0).normalize(),
        };

        // Acceleration.
        let walk_speed = match debuff_slow_walk {
            false => player_walk_speed.0,
            true => player_walk_speed.0 / 2.0,
        };
        player_velocity.0 += move_vector * 2000.0 * delta.delta_seconds();
        player_velocity.0 = player_velocity.0.clamp_length_max(walk_speed);
    } else if player_velocity.0 != Vec2::ZERO {
        // Deceleration.
        let old_signum = player_velocity.0.signum();
        player_velocity.0 =
            (player_velocity.0 - 2000.0 * old_signum * delta.delta_seconds()).clamp_length_min(0.0);

        let new_signum = player_velocity.0.signum();
        if old_signum != new_signum {
            player_velocity.0 = Vec2::ZERO;
        }
    }

    if player_velocity.0 != Vec2::ZERO {
        // Get new position on each axis.
        let mut new_position_horizontal = player_transform.translation;
        new_position_horizontal.x += player_velocity.0.x * delta.delta_seconds();
        let mut new_position_vertical = player_transform.translation;
        new_position_vertical.y += player_velocity.0.y * delta.delta_seconds();

        // Calculate separate bounding-boxes for each axis-movement.
        let new_player_rect_horizontal =
            Rect::from_center_size(new_position_horizontal.truncate(), player_size.0 / 2.0);
        let new_player_rect_vertical =
            Rect::from_center_size(new_position_vertical.truncate(), player_size.0 / 2.0);

        // Check for collision with `Npcs`.
        for (npc_transform, npc_size) in npc_query.iter() {
            let npc_collider_rect =
                Rect::from_center_size(npc_transform.translation.truncate(), npc_size.0);

            if !npc_collider_rect
                .intersect(new_player_rect_horizontal)
                .is_empty()
            {
                player_velocity.0.x = 0.0;
            }

            if !npc_collider_rect
                .intersect(new_player_rect_vertical)
                .is_empty()
            {
                player_velocity.0.y = 0.0;
            }
        }

        // Check for collision with `TilemapColliders`.
        if let Ok(tilemap_colliders) = tilemap_collider_query.get_single() {
            for tilemap_collider_rect in tilemap_colliders.0.iter() {
                if !tilemap_collider_rect
                    .intersect(new_player_rect_horizontal)
                    .is_empty()
                {
                    player_velocity.0.x = 0.0;
                }

                if !tilemap_collider_rect
                    .intersect(new_player_rect_vertical)
                    .is_empty()
                {
                    player_velocity.0.y = 0.0;
                }
            }
        }

        // Moving the player.
        player_transform.translation += player_velocity.0.extend(0.0) * delta.delta_seconds();
    }
}

pub fn player_movement_reset(mut query: Query<&mut PlayerState, With<Player>>) {
    if let Ok(mut player_state) = query.get_single_mut() {
        *player_state = PlayerState::Idle;
    }
}
