// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::prelude::{Changed, NextState, Query, ResMut, Transform, With, Without};

use crate::plugins::{game::states::LevelState, player::components::Player};

use super::components::{Trigger, TriggerTargetLevel};

pub fn activate_trigger(
    trigger_query: Query<(&Transform, &TriggerTargetLevel), (With<Trigger>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Changed<Transform>)>,
    mut level_state: ResMut<NextState<LevelState>>,
) {
    if player_query.is_empty() {
        return;
    }

    let player_transform = player_query
        .get_single()
        .expect("0 or more than 1 `Player` found.");

    for (trigger_transform, trigger_target_level) in trigger_query.iter() {
        if player_transform
            .translation
            .distance(trigger_transform.translation)
            < 8.0
        {
            level_state.set(trigger_target_level.0);
        }
    }
}
