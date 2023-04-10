// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::prelude::Resource;
use std::collections::HashSet;

#[derive(Debug, Resource, Default)]
pub struct GameProgress {
    pub destroyed_spawners: HashSet<String>,
    pub mushroom_tutorial_finished: bool,
}
