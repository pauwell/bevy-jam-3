// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::prelude::Component;

#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnemyState {
    #[default]
    Idle,
    Stunned,
}
