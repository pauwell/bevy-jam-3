// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use bevy::{
    prelude::{Component, Vec2},
    time::Timer,
};

#[derive(Component, Default, Debug, Clone)]
pub struct Spell;

#[derive(Component, Default, Debug, Clone)]
pub struct SpellVelocity(pub Vec2);

#[derive(Component, Default, Debug, Clone)]
pub struct SpellDamage(pub f32);

#[derive(Component, Default, Debug, Clone)]
pub struct SpellTimer(pub Timer);

#[derive(Component, Default, Debug, Clone)]
pub struct SpellIsDead;
