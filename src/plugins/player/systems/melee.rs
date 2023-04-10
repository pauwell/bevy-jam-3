// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::{
    prelude::{EventWriter, Input, KeyCode, Query, Res, Transform, Vec2, With},
    time::Time,
};

use crate::plugins::{
    audio::{events::AudioStartPlayEvent, AudioClip},
    player::{
        components::{
            Player, PlayerDirection, PlayerMeleeAttackHitPosition, PlayerMeleeAttackTimer,
        },
        states::PlayerState,
    },
};

pub fn player_melee_attack(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<
        (
            &Transform,
            &mut PlayerState,
            &PlayerDirection,
            &mut PlayerMeleeAttackTimer,
            &mut PlayerMeleeAttackHitPosition,
        ),
        With<Player>,
    >,
    delta: Res<Time>,
    mut audio_start_play_event_writer: EventWriter<AudioStartPlayEvent>,
) {
    if query.is_empty() {
        return;
    }

    let (
        player_transform,
        mut player_state,
        player_direction,
        mut player_melee_attack_timer,
        mut player_melee_attack_hit_position,
    ) = query
        .get_single_mut()
        .expect("0 or more than 1 player found.");

    if *player_state != PlayerState::Idle
        && *player_state != PlayerState::Walk
        && *player_state != PlayerState::UseMelee
    {
        return;
    }

    if *player_state == PlayerState::UseMelee {
        player_melee_attack_timer.0.tick(delta.delta());
        if player_melee_attack_timer.0.just_finished() {
            *player_state = PlayerState::Idle;
            player_melee_attack_hit_position.0 = Vec2::ZERO;
        }
    } else if keyboard.just_pressed(KeyCode::S) {
        *player_state = PlayerState::UseMelee;
        player_melee_attack_timer.0.reset();
        let player_position = player_transform.translation.truncate();
        player_melee_attack_hit_position.0 = match *player_direction {
            PlayerDirection::Up => player_position + Vec2 { x: 0.0, y: 1.0 },
            PlayerDirection::Down => player_position + Vec2 { x: 0.0, y: -1.0 },
            PlayerDirection::Left => player_position + Vec2 { x: -1.0, y: 0.0 },
            PlayerDirection::Right => player_position + Vec2 { x: 1.0, y: 0.0 },
            PlayerDirection::UpLeft => player_position + Vec2 { x: -1.0, y: 1.0 },
            PlayerDirection::UpRight => player_position + Vec2 { x: 1.0, y: 1.0 },
            PlayerDirection::DownLeft => player_position + Vec2 { x: -1.0, y: -1.0 },
            PlayerDirection::DownRight => player_position + Vec2 { x: 1.0, y: -1.0 },
        };
        audio_start_play_event_writer.send(AudioStartPlayEvent(AudioClip::MeleeAttack));
    }
}
