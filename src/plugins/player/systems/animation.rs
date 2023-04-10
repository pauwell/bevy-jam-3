// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::prelude::{Changed, Entity, EventWriter, Or, Query, With};

use crate::plugins::{
    animation::events::AnimationStartEvent,
    player::{
        components::{Player, PlayerDirection},
        states::PlayerState,
    },
};

pub fn player_animation(
    query: Query<
        (Entity, &PlayerState, &PlayerDirection),
        (
            Or<(Changed<PlayerState>, Changed<PlayerDirection>)>,
            With<Player>,
        ),
    >,
    mut animation_start_event_writer: EventWriter<AnimationStartEvent>,
) {
    if query.is_empty() {
        return;
    }

    let (player_entity, player_state, player_direction) = query
        .get_single()
        .expect("0 or more than 1 `Player` found.");

    let state_part_clip_name = match player_state {
        PlayerState::Idle => "idle",
        PlayerState::Walk => "walk",
        PlayerState::UseMelee => "melee",
        PlayerState::UseMagic => "magic",
        _ => "idle",
    };

    let direction_part_clip_name = match player_direction {
        PlayerDirection::Up | PlayerDirection::UpLeft | PlayerDirection::UpRight => "up",
        PlayerDirection::Down | PlayerDirection::DownLeft | PlayerDirection::DownRight => "down",
        PlayerDirection::Left => "left",
        PlayerDirection::Right => "right",
    };

    // Creating the clips name in format: "[state]_[direction]" (e.g: "idle_left", "walk_up")
    let full_clip_name = state_part_clip_name.to_owned() + "_" + direction_part_clip_name;

    animation_start_event_writer.send(AnimationStartEvent {
        target_entity: player_entity,
        clip_name: full_clip_name.into(),
        reset_timer: false,
    })
}
