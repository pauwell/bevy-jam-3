// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::prelude::{Component, Entity, Vec2};

#[derive(Component, Default, Debug, Clone)]
pub struct HealthBar;

#[derive(Component, Default, Debug, Clone)]
pub(super) struct HealthBarReady;

#[derive(Component, Debug, Clone)]
pub(super) struct HealthBarTargetEntity(pub Entity);

#[derive(Component, Default, Debug, Clone)]
pub struct HealthBarSize(pub Vec2);

#[derive(Component, Default, Debug, Clone)]
pub struct HealthBarPercentFilled(pub f32);
