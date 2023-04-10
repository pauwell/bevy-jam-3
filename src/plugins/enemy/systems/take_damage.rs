// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use bevy::{
    prelude::{
        Changed, Color, Commands, Entity, EventWriter, Query, Res, Transform, Vec2, With, Without,
    },
    sprite::TextureAtlasSprite,
    time::Time,
};

use crate::{
    helper::get_percentage_rounded,
    plugins::{
        audio::{events::AudioStartPlayEvent, AudioClip},
        enemy::{
            components::{Enemy, EnemyHealth, EnemyHealthMax, EnemyIsStunnedTimer},
            states::EnemyState,
        },
        health_bar::events::HealthBarFillPercentEvent,
        player::components::{
            PlayerDebuffNoDamageAgainstBlobs, PlayerMeleeAttackDamage, PlayerMeleeAttackHitPosition,
        },
        spell::components::{SpellDamage, SpellIsDead},
    },
};

pub fn enemy_take_magic_damage(
    mut commands: Commands,
    mut enemy_query: Query<(&Transform, &mut EnemyState, &mut EnemyHealth), With<Enemy>>,
    spell_query: Query<(Entity, &Transform, &SpellDamage), (Without<SpellIsDead>, Without<Enemy>)>,
    mut audio_start_play_event_writer: EventWriter<AudioStartPlayEvent>,
) {
    if enemy_query.is_empty() || spell_query.is_empty() {
        return;
    }

    for (enemy_transform, mut enemy_state, mut enemy_health) in enemy_query.iter_mut() {
        if *enemy_state == EnemyState::Stunned {
            continue;
        }

        for (spell_entity, spell_transform, spell_damage) in spell_query.iter() {
            if enemy_transform
                .translation
                .truncate()
                .distance(spell_transform.translation.truncate())
                < 10.0
            {
                enemy_health.0 -= spell_damage.0;
                *enemy_state = EnemyState::Stunned;
                commands.entity(spell_entity).insert(SpellIsDead);
                audio_start_play_event_writer.send(AudioStartPlayEvent(AudioClip::HitEnemy));
            }
        }
    }
}

pub fn enemy_take_melee_damage(
    mut enemy_query: Query<(&Transform, &mut EnemyState, &mut EnemyHealth), With<Enemy>>,
    player_query: Query<
        (&PlayerMeleeAttackHitPosition, &PlayerMeleeAttackDamage),
        Changed<PlayerMeleeAttackHitPosition>,
    >,
    debuff_no_blob_damage_query: Query<&PlayerDebuffNoDamageAgainstBlobs>,
    mut audio_start_play_event_writer: EventWriter<AudioStartPlayEvent>,
) {
    if enemy_query.is_empty() || player_query.is_empty() || !debuff_no_blob_damage_query.is_empty()
    {
        return;
    }

    let (player_melee_attack_hit_position, player_melee_attack_damage) = player_query
        .get_single()
        .expect("0 or more than 1 melee hit position found for player.");

    if player_melee_attack_hit_position.0 == Vec2::ZERO {
        return;
    }

    for (enemy_transform, mut enemy_state, mut enemy_health) in enemy_query.iter_mut() {
        if *enemy_state == EnemyState::Stunned {
            continue;
        }

        if enemy_transform
            .translation
            .truncate()
            .distance(player_melee_attack_hit_position.0)
            < 20.0
        {
            enemy_health.0 -= player_melee_attack_damage.0;
            *enemy_state = EnemyState::Stunned;
            if enemy_health.0 > 0.0 {
                audio_start_play_event_writer.send(AudioStartPlayEvent(AudioClip::HitEnemy));
            }
        }
    }
}

pub fn enemy_is_stunned(
    mut enemy_query: Query<
        (
            &mut EnemyState,
            &mut EnemyIsStunnedTimer,
            &mut TextureAtlasSprite,
        ),
        With<Enemy>,
    >,
    delta: Res<Time>,
) {
    for (mut enemy_state, mut enemy_is_stunned_timer, mut enemy_sprite) in enemy_query.iter_mut() {
        if *enemy_state != EnemyState::Stunned {
            continue;
        }
        enemy_is_stunned_timer.0.tick(delta.delta());
        enemy_sprite.color.set_a(0.7);
        if enemy_is_stunned_timer.0.just_finished() {
            enemy_sprite.color = Color::WHITE;
            *enemy_state = EnemyState::Idle;
            enemy_is_stunned_timer.0.reset();
        }
    }
}
pub fn enemy_update_health_bar(
    query: Query<(Entity, &EnemyHealth, &EnemyHealthMax), Changed<EnemyHealth>>,
    mut percent_filled_event_writer: EventWriter<HealthBarFillPercentEvent>,
) {
    if query.is_empty() {
        return;
    }

    for (enemy_entity, enemy_health, enemy_health_max) in query.iter() {
        percent_filled_event_writer.send(HealthBarFillPercentEvent {
            percent_filled: get_percentage_rounded(enemy_health.0, enemy_health_max.0),
            target: enemy_entity,
        });
    }
}
