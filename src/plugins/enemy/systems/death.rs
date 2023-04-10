// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::{
    prelude::{
        Added, Changed, Commands, DespawnRecursiveExt, Entity, EventWriter, Query, Res, With,
        Without,
    },
    time::Time,
};

use crate::plugins::{
    animation::events::AnimationStartEvent,
    audio::{events::AudioStartPlayEvent, AudioClip},
    enemy::components::{EnemyDeathTimer, EnemyHealth, EnemyIsDead},
};

pub fn enemy_check_if_dead(
    mut commands: Commands,
    mut query: Query<(Entity, &EnemyHealth), (Changed<EnemyHealth>, Without<EnemyIsDead>)>,
) {
    for (enemy_entity, enemy_health) in query.iter_mut() {
        if enemy_health.0 <= 0.0 {
            commands.entity(enemy_entity).insert(EnemyIsDead);
        }
    }
}

pub fn enemy_on_death(
    mut query: Query<(Entity, &mut EnemyDeathTimer), Added<EnemyIsDead>>,
    mut animation_start_event_writer: EventWriter<AnimationStartEvent>,
    mut audio_start_play_event_writer: EventWriter<AudioStartPlayEvent>,
) {
    for (enemy_entity, mut enemy_death_timer) in query.iter_mut() {
        enemy_death_timer.0.reset();
        animation_start_event_writer.send(AnimationStartEvent {
            target_entity: enemy_entity,
            clip_name: "death".into(),
            reset_timer: true,
        });
        audio_start_play_event_writer.send(AudioStartPlayEvent(AudioClip::DeathEnemy));
    }
}

pub fn enemy_update_death(
    mut commands: Commands,
    mut query: Query<(Entity, &mut EnemyDeathTimer), With<EnemyIsDead>>,
    delta: Res<Time>,
) {
    for (enemy_entity, mut enemy_timer) in query.iter_mut() {
        enemy_timer.0.tick(delta.delta());
        if enemy_timer.0.just_finished() {
            commands.entity(enemy_entity).despawn_recursive();
        }
    }
}
