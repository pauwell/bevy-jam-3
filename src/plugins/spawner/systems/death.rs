// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::{
    prelude::{
        Added, Changed, Commands, DespawnRecursiveExt, Entity, EventWriter, NextState, Query, Res,
        ResMut, With, Without,
    },
    time::Time,
};

use crate::{
    plugins::{
        animation::events::AnimationStartEvent,
        audio::{events::AudioStartPlayEvent, AudioClip},
        game::resources::GameProgress,
        spawner::components::{SpawnerDeathTimer, SpawnerHealth, SpawnerIsDead, SpawnerName},
    },
    states::AppState,
};

pub fn spawner_check_if_dead(
    mut commands: Commands,
    query: Query<(Entity, &SpawnerHealth), (Changed<SpawnerHealth>, Without<SpawnerIsDead>)>,
) {
    for (spawner_entity, spawner_health) in query.iter() {
        if spawner_health.0 <= 0.0 {
            commands.entity(spawner_entity).insert(SpawnerIsDead);
        }
    }
}

pub fn spawner_on_death(
    mut query: Query<(Entity, &mut SpawnerDeathTimer), Added<SpawnerIsDead>>,
    mut animation_start_event_writer: EventWriter<AnimationStartEvent>,
    mut audio_start_play_event_writer: EventWriter<AudioStartPlayEvent>,
) {
    for (spawner_entity, mut spawner_death_timer) in query.iter_mut() {
        spawner_death_timer.0.reset();
        animation_start_event_writer.send(AnimationStartEvent {
            target_entity: spawner_entity,
            clip_name: "death".into(),
            reset_timer: true,
        });
        audio_start_play_event_writer.send(AudioStartPlayEvent(AudioClip::DestroySpawner));
    }
}

pub fn spawner_update_death(
    mut commands: Commands,
    mut query: Query<(Entity, &mut SpawnerDeathTimer, &SpawnerName), With<SpawnerIsDead>>,
    mut game_progress: ResMut<GameProgress>,
    mut app_state: ResMut<NextState<AppState>>,
    delta: Res<Time>,
) {
    for (spawner_entity, mut spawner_timer, spawner_name) in query.iter_mut() {
        spawner_timer.0.tick(delta.delta());
        if spawner_timer.0.just_finished() {
            game_progress
                .destroyed_spawners
                .insert(spawner_name.0.clone());

            if game_progress.destroyed_spawners.len() == 3 {
                app_state.set(AppState::GameWin);
            }
            commands.entity(spawner_entity).despawn_recursive();
        }
    }
}
