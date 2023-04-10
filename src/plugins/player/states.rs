// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use bevy::{prelude::Component, reflect::Reflect};

#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum PlayerState {
    #[default]
    Idle,
    Walk,
    Talk,
    Stunned,
    UseMelee,
    UseMagic,
    UsePotion,
}
