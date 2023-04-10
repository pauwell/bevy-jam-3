// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use bevy::prelude::Entity;

pub struct HealthBarFillPercentEvent {
    pub target: Entity,
    pub percent_filled: f32,
}
