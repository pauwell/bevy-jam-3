// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::{
    prelude::Bundle,
    time::{Timer, TimerMode},
    utils::HashMap,
};
use std::time::Duration;

use super::components::{ActiveAnimationClipName, AnimationClip, AnimationClips, AnimationTimer};

#[derive(Bundle, Debug)]
pub struct AnimationBundle {
    clips: AnimationClips,
    active_clip_name: ActiveAnimationClipName,
    timer: AnimationTimer,
}

impl Default for AnimationBundle {
    fn default() -> Self {
        AnimationBundle {
            clips: AnimationClips(HashMap::new()),
            active_clip_name: ActiveAnimationClipName::default(),
            timer: AnimationTimer(Timer::from_seconds(0.0, TimerMode::Repeating)),
        }
    }
}

impl AnimationBundle {
    pub fn with_clip(mut self, clip: &AnimationClip) -> Self {
        if !self.clips.0.contains_key(&clip.name) {
            self.clips.0.insert(clip.name.clone(), clip.clone());
        }
        self
    }

    pub fn with_active_clip(mut self, clip: &AnimationClip) -> Self {
        if !self.clips.0.contains_key(&clip.name) {
            self.clips.0.insert(clip.name.clone(), clip.clone());
            self.active_clip_name.0 = clip.name.clone();
            self.timer
                .0
                .set_duration(Duration::from_secs_f32(clip.playback_speed));
        }
        self
    }
}
