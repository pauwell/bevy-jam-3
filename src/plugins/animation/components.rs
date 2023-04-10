// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::{prelude::*, utils::HashMap};

#[derive(Debug, Default, Clone)]
pub struct AnimationClip {
    pub name: String,
    pub index_first: usize,
    pub index_last: usize,
    pub playback_speed: f32,
}

#[derive(Component, Default, Debug, Clone)]
pub struct AnimationClips(pub HashMap<String, AnimationClip>);

#[derive(Component, Default, Debug, Clone)]
pub struct ActiveAnimationClipName(pub String);

#[derive(Component, Default, Debug, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);
