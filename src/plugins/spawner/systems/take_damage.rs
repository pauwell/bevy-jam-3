// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use bevy::{
    prelude::{Changed, Color, Entity, EventWriter, Query, Res, Transform, Vec2, With},
    sprite::TextureAtlasSprite,
    time::Time,
};

use crate::{
    helper::get_percentage_rounded,
    plugins::{
        audio::{events::AudioStartPlayEvent, AudioClip},
        health_bar::events::HealthBarFillPercentEvent,
        player::components::{
            PlayerConsumedMushroom, PlayerMeleeAttackDamage, PlayerMeleeAttackHitPosition,
        },
        spawner::{
            components::{Spawner, SpawnerHealth, SpawnerHealthMax, SpawnerIsStunnedTimer},
            states::SpawnerState,
        },
    },
};

pub fn spawner_take_melee_damage(
    mut spawner_query: Query<(&Transform, &mut SpawnerState, &mut SpawnerHealth), With<Spawner>>,
    player_query: Query<
        (&PlayerMeleeAttackHitPosition, &PlayerMeleeAttackDamage),
        (
            Changed<PlayerMeleeAttackHitPosition>,
            With<PlayerConsumedMushroom>,
        ),
    >,
    mut audio_start_play_event_writer: EventWriter<AudioStartPlayEvent>,
) {
    if spawner_query.is_empty() || player_query.is_empty() {
        return;
    }

    let (player_melee_attack_hit_position, player_melee_attack_damage) = player_query
        .get_single()
        .expect("0 or more than 1 melee hit position found for player.");

    if player_melee_attack_hit_position.0 == Vec2::ZERO {
        return;
    }

    for (spawner_transform, mut spawner_state, mut spawner_health) in spawner_query.iter_mut() {
        if *spawner_state == SpawnerState::Stunned {
            continue;
        }

        if spawner_transform
            .translation
            .truncate()
            .distance(player_melee_attack_hit_position.0)
            < 20.0
        {
            spawner_health.0 -= player_melee_attack_damage.0;
            *spawner_state = SpawnerState::Stunned;
            audio_start_play_event_writer.send(AudioStartPlayEvent(AudioClip::HitSpawner));
        }
    }
}

pub fn spawner_is_stunned(
    mut spawner_query: Query<
        (
            &mut SpawnerState,
            &mut SpawnerIsStunnedTimer,
            &mut TextureAtlasSprite,
        ),
        With<Spawner>,
    >,
    delta: Res<Time>,
) {
    for (mut spawner_state, mut spawner_is_stunned_timer, mut spawner_sprite) in
        spawner_query.iter_mut()
    {
        if *spawner_state != SpawnerState::Stunned {
            continue;
        }
        spawner_is_stunned_timer.0.tick(delta.delta());
        spawner_sprite.color.set_a(0.7);
        if spawner_is_stunned_timer.0.just_finished() {
            spawner_sprite.color = Color::WHITE;
            *spawner_state = SpawnerState::Idle;
            spawner_is_stunned_timer.0.reset();
        }
    }
}
pub fn spawner_update_health_bar(
    query: Query<(Entity, &SpawnerHealth, &SpawnerHealthMax), Changed<SpawnerHealth>>,
    mut percent_filled_event_writer: EventWriter<HealthBarFillPercentEvent>,
) {
    if query.is_empty() {
        return;
    }

    for (spawner_entity, spawner_health, spawner_health_max) in query.iter() {
        percent_filled_event_writer.send(HealthBarFillPercentEvent {
            percent_filled: get_percentage_rounded(spawner_health.0, spawner_health_max.0),
            target: spawner_entity,
        });
    }
}
