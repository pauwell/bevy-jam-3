// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use super::{
    components::{ActiveAnimationClipName, AnimationClips, AnimationTimer},
    events::AnimationStartEvent,
};
use bevy::{
    prelude::{Entity, EventReader, Query, Res},
    sprite::TextureAtlasSprite,
    time::Time,
};
use std::time::Duration;

pub fn update_active_animation_clips(
    time: Res<Time>,
    mut query: Query<(
        &AnimationClips,
        &ActiveAnimationClipName,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (all_clips, active_clip_name, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            // Update the texture-index of the sprite each time the `AnimationTimer` finishes.
            if let Some(active_clip) = all_clips.0.get_key_value(&active_clip_name.0) {
                sprite.index = if sprite.index >= active_clip.1.index_last {
                    active_clip.1.index_first
                } else {
                    sprite.index + 1
                };
            }
        }
    }
}

pub fn handle_animation_start_event(
    mut query: Query<(
        Entity,
        &AnimationClips,
        &mut ActiveAnimationClipName,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
    mut start_event_reader: EventReader<AnimationStartEvent>,
) {
    for start_event in start_event_reader.iter() {
        for (entity, all_clips, mut active_clip_name, mut animation_timer, mut sprite) in &mut query
        {
            if start_event.target_entity != entity {
                continue;
            }

            // Find the new active clip with the clip-name provided by the event and activate it.
            if let Some(new_active_clip) = all_clips.0.get_key_value(&start_event.clip_name) {
                active_clip_name.0 = start_event.clip_name.clone();
                sprite.index = new_active_clip.1.index_first;

                animation_timer
                    .0
                    .set_duration(Duration::from_secs_f32(new_active_clip.1.playback_speed));

                if start_event.reset_timer {
                    animation_timer.0.reset();
                }
            };
        }
    }
}
