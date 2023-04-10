// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use bevy::{
    prelude::{
        Added, Commands, DespawnRecursiveExt, Entity, EventWriter, Query, Res, Transform, With,
        Without,
    },
    time::Time,
};

use crate::plugins::{
    animation::events::AnimationStartEvent, tilemap::components::TilemapColliders,
};

use super::components::{SpellIsDead, SpellTimer, SpellVelocity};

pub fn spell_movement(
    mut commands: Commands,
    mut spell_query: Query<
        (Entity, &mut Transform, &SpellVelocity, &mut SpellTimer),
        Without<SpellIsDead>,
    >,
    tilemap_collider_query: Query<&TilemapColliders>,
    delta: Res<Time>,
) {
    for (spell_entity, mut spell_transform, spell_velocity, mut spell_timer) in
        spell_query.iter_mut()
    {
        spell_timer.0.tick(delta.delta());
        if spell_timer.0.just_finished() {
            commands.entity(spell_entity).insert(SpellIsDead);
        } else {
            spell_transform.translation.x += spell_velocity.0.x * delta.delta_seconds();
            spell_transform.translation.y += spell_velocity.0.y * delta.delta_seconds();

            if let Ok(tilemap_colliders) = tilemap_collider_query.get_single() {
                for tilemap_collider_rect in tilemap_colliders.0.iter() {
                    if tilemap_collider_rect.contains(spell_transform.translation.truncate()) {
                        commands.entity(spell_entity).insert(SpellIsDead);
                    }
                }
            }
        }
    }
}

pub fn spell_on_death(
    mut query: Query<(Entity, &mut SpellTimer), Added<SpellIsDead>>,
    mut animation_start_event_writer: EventWriter<AnimationStartEvent>,
) {
    for (spell_entity, mut spell_timer) in query.iter_mut() {
        spell_timer.0.reset();
        animation_start_event_writer.send(AnimationStartEvent {
            target_entity: spell_entity,
            clip_name: "death".into(),
            reset_timer: true,
        });
    }
}

pub fn spell_update_death(
    mut commands: Commands,
    mut query: Query<(Entity, &mut SpellTimer), With<SpellIsDead>>,
    delta: Res<Time>,
) {
    for (spell_entity, mut spell_timer) in query.iter_mut() {
        spell_timer.0.tick(delta.delta());
        if spell_timer.0.just_finished() {
            commands.entity(spell_entity).despawn_recursive();
        }
    }
}
