// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use bevy::{
    prelude::{
        Changed, Commands, DespawnRecursiveExt, Entity, EventWriter, NextState, Query, Res, ResMut,
        Transform, With, Without,
    },
    sprite::TextureAtlasSprite,
    time::Time,
};

use crate::{
    helper::get_percentage_rounded,
    plugins::{
        audio::{events::AudioStartPlayEvent, AudioClip},
        enemy::{
            components::{Enemy, EnemyDamage, EnemyHealth, EnemyIsDead},
            states::EnemyState,
        },
        health_bar::events::HealthBarFillPercentEvent,
        player::{
            components::{Player, PlayerHealth, PlayerHealthMax, PlayerIsStunnedTimer},
            states::PlayerState,
        },
    },
    states::AppState,
};

pub fn player_enemy_collision(
    mut enemy_query: Query<
        (&Transform, &mut EnemyState, &mut EnemyHealth, &EnemyDamage),
        (With<Enemy>, Without<EnemyIsDead>),
    >,
    mut player_query: Query<
        (&Transform, &mut PlayerHealth, &mut PlayerState),
        (With<Player>, Without<Enemy>),
    >,
    mut audio_start_play_event_writer: EventWriter<AudioStartPlayEvent>,
) {
    if enemy_query.is_empty() || player_query.is_empty() {
        return;
    }

    let (player_transform, mut player_health, mut player_state) = player_query
        .get_single_mut()
        .expect("0 or more than 1 player found.");

    if *player_state == PlayerState::Talk {
        return;
    }

    for (enemy_transform, mut enemy_state, mut enemy_health, enemy_damage) in enemy_query.iter_mut()
    {
        if *enemy_state == EnemyState::Stunned {
            continue;
        }

        if enemy_transform
            .translation
            .truncate()
            .distance(player_transform.translation.truncate())
            < 7.0
        {
            enemy_health.0 -= 5.0;
            *enemy_state = EnemyState::Stunned;

            player_health.0 -= enemy_damage.0;
            *player_state = PlayerState::Stunned;

            audio_start_play_event_writer.send(AudioStartPlayEvent(AudioClip::HitPlayer));
        }
    }
}

pub fn player_is_stunned(
    mut query: Query<
        (
            &mut PlayerState,
            &mut PlayerIsStunnedTimer,
            &mut TextureAtlasSprite,
        ),
        With<Player>,
    >,
    delta: Res<Time>,
) {
    for (mut player_state, mut player_is_stunned_timer, mut player_sprite) in query.iter_mut() {
        if *player_state != PlayerState::Stunned {
            continue;
        }
        player_is_stunned_timer.0.tick(delta.delta());
        player_sprite.color.set_a(0.7);
        if player_is_stunned_timer.0.just_finished() {
            player_sprite.color.set_a(1.0);
            *player_state = PlayerState::Idle;
            player_is_stunned_timer.0.reset();
        }
    }
}

pub fn player_death(
    mut commands: Commands,
    query: Query<(Entity, &PlayerHealth), Changed<PlayerHealth>>,
    mut app_state: ResMut<NextState<AppState>>,
    mut audio_start_play_event_writer: EventWriter<AudioStartPlayEvent>,
) {
    if query.is_empty() {
        return;
    }

    let (player_entity, player_health) =
        query.get_single().expect("0 or more than 1 player found.");

    if player_health.0 <= 0.0 {
        audio_start_play_event_writer.send(AudioStartPlayEvent(AudioClip::DeathPlayer));
        commands.entity(player_entity).despawn_recursive();
        app_state.set(AppState::GameOver);
    }
}

pub fn player_update_health_bar(
    player_query: Query<(Entity, &PlayerHealth, &PlayerHealthMax), Changed<PlayerHealth>>,
    mut percent_filled_event_writer: EventWriter<HealthBarFillPercentEvent>,
) {
    if player_query.is_empty() {
        return;
    }

    let (player_entity, player_health, player_health_max) = player_query
        .get_single()
        .expect("0 or more than 1 player found.");

    percent_filled_event_writer.send(HealthBarFillPercentEvent {
        percent_filled: get_percentage_rounded(player_health.0, player_health_max.0),
        target: player_entity,
    });
}
