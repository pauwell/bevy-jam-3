// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::{prelude::Component, reflect::Reflect, time::Timer};

#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Spawner;

#[derive(Component, Default, Debug, Clone, PartialEq, Eq, Reflect)]
pub struct SpawnerName(pub String);

#[derive(Component, Default, Debug, Clone)]
pub struct SpawnerTimer(pub Timer);

#[derive(Component, Default, Debug, Clone)]
pub struct SpawnerHealth(pub f32);

#[derive(Component, Default, Debug, Clone)]
pub struct SpawnerHealthMax(pub f32);

#[derive(Component, Default, Debug, Clone)]
pub struct SpawnerIsStunnedTimer(pub Timer);

#[derive(Component, Default, Debug, Clone)]
pub struct SpawnerDeathTimer(pub Timer);

#[derive(Component, Default, Debug, Clone)]
pub struct SpawnerIsDead;
