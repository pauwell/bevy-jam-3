// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::prelude::{Changed, Query, Transform, With, Without};

use crate::plugins::{enemy::components::Enemy, player::components::PlayerVelocity};

pub fn enemy_calculate_z_index(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    player_query: Query<&Transform, (Changed<PlayerVelocity>, Without<Enemy>)>,
) {
    if player_query.is_empty() {
        return;
    }

    let player_transform = player_query
        .get_single()
        .expect("0 or more than 1 player found.");

    for mut npc_transform in enemy_query.iter_mut() {
        if npc_transform.translation.y > player_transform.translation.y {
            npc_transform.translation.z = player_transform.translation.z - 1.0;
        } else {
            npc_transform.translation.z = player_transform.translation.z + 1.0;
        }
    }
}
