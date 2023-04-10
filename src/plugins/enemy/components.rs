// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::{
    prelude::{Component, Vec2},
    reflect::Reflect,
    time::Timer,
};

#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub struct Enemy;

#[derive(Component, Default, Debug, Clone, Reflect)]
pub struct EnemyHealth(pub f32);

#[derive(Component, Default, Debug, Clone, Reflect)]
pub struct EnemyHealthMax(pub f32);

#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Reflect)]
pub struct EnemySize(pub Vec2);

#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Reflect)]
pub struct EnemyWalkSpeed(pub f32);

#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Reflect)]
pub struct EnemyDamage(pub f32);

#[derive(Component, Default, Debug, Clone, Reflect)]
pub struct EnemyChangeDirectionTimer(pub Timer);

#[derive(Component, Default, Debug, Clone, Reflect)]
pub struct EnemyIsStunnedTimer(pub Timer);

#[derive(Component, Default, Debug, Clone, Reflect)]
pub struct EnemyDeathTimer(pub Timer);

#[derive(Component, Default, Debug, Clone)]
pub struct EnemyIsDead;

#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum EnemyType {
    #[default]
    Blob,
}

#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum EnemyDirection {
    Up,
    #[default]
    Down,
    Left,
    Right,
}
