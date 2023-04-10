// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::prelude::{Query, Transform, Vec3, With, Without};

use crate::plugins::player::components::Player;

use super::components::{GameCamera, GameCameraState};

pub fn update_game_camera(
    mut game_camera_query: Query<(&mut Transform, &GameCameraState), With<GameCamera>>,
    player_query: Query<&Transform, (With<Player>, Without<GameCamera>)>,
) {
    if game_camera_query.is_empty() || player_query.is_empty() {
        return;
    }

    let (mut game_camera_transform, game_camera_state) = game_camera_query
        .get_single_mut()
        .expect("0 or more than 1 game-camera found.");

    match *game_camera_state {
        GameCameraState::Static => return,
        GameCameraState::FollowPlayer => {
            let player_transform = player_query
                .get_single()
                .expect("0 or more than 1 player found.");

            game_camera_transform.translation = Vec3::new(
                player_transform.translation.x,
                player_transform.translation.y,
                game_camera_transform.translation.z,
            );
        }
    }
}
