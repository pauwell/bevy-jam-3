// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::{
    prelude::{Commands, EventWriter, Input, KeyCode, Query, Res, Transform, Vec2, With},
    time::Time,
};

use crate::plugins::{
    atlas::resources::GameAtlases,
    audio::{events::AudioStartPlayEvent, AudioClip},
    player::{
        components::{
            Player, PlayerDebuffNoMagic, PlayerDirection, PlayerMagicAttackDamage,
            PlayerMagicAttackSpeed, PlayerMagicAttackTimer,
        },
        states::PlayerState,
    },
    spell::bundles::SpellBundle,
};

pub fn player_magic_attack(
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<
        (
            &Transform,
            &mut PlayerState,
            &PlayerDirection,
            &mut PlayerMagicAttackTimer,
            &PlayerMagicAttackDamage,
            &PlayerMagicAttackSpeed,
        ),
        With<Player>,
    >,
    debuff_no_magic_query: Query<&PlayerDebuffNoMagic>,
    delta: Res<Time>,
    game_atlases: Res<GameAtlases>,
    mut audio_start_play_event_writer: EventWriter<AudioStartPlayEvent>,
) {
    if query.is_empty() || !debuff_no_magic_query.is_empty() {
        return;
    }

    let (
        player_transform,
        mut player_state,
        player_direction,
        mut player_magic_attack_timer,
        player_magic_attack_damage,
        player_magic_attack_speed,
    ) = query
        .get_single_mut()
        .expect("0 or more than 1 player found.");

    if *player_state != PlayerState::Idle
        && *player_state != PlayerState::Walk
        && *player_state != PlayerState::UseMagic
    {
        return;
    }

    if *player_state == PlayerState::UseMagic {
        player_magic_attack_timer.0.tick(delta.delta());
        if player_magic_attack_timer.0.just_finished() {
            *player_state = PlayerState::Idle;
        }
    } else if keyboard.just_pressed(KeyCode::D) {
        *player_state = PlayerState::UseMagic;
        player_magic_attack_timer.0.reset();
        let player_position = player_transform.translation.truncate();

        let spell_velocity = match *player_direction {
            PlayerDirection::Up => Vec2 { x: 0.0, y: 1.0 },
            PlayerDirection::Down => Vec2 { x: 0.0, y: -1.0 },
            PlayerDirection::Left => Vec2 { x: -1.0, y: 0.0 },
            PlayerDirection::Right => Vec2 { x: 1.0, y: 0.0 },
            PlayerDirection::UpLeft => Vec2 { x: -1.0, y: 1.0 }.normalize(),
            PlayerDirection::UpRight => Vec2 { x: 1.0, y: 1.0 }.normalize(),
            PlayerDirection::DownLeft => Vec2 { x: -1.0, y: -1.0 }.normalize(),
            PlayerDirection::DownRight => Vec2 { x: 1.0, y: -1.0 }.normalize(),
        } * player_magic_attack_speed.0;

        audio_start_play_event_writer.send(AudioStartPlayEvent(AudioClip::MagicAttack));
        commands.spawn(SpellBundle::new(
            game_atlases.spell.clone(),
            player_position,
            spell_velocity,
            player_magic_attack_damage.0,
            0.8,
            13.0,
        ));
    }
}
