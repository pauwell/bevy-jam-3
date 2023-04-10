// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use bevy::prelude::Component;

use crate::plugins::game::states::LevelState;

#[derive(Component, Default, Debug, Clone)]
pub struct Trigger;

#[derive(Component, Default, Debug, Clone)]
pub struct TriggerTargetLevel(pub LevelState);
