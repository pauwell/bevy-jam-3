// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use bevy::prelude::{App, IntoSystemConfig, OnUpdate, Plugin};

use crate::states::AppState;

use self::systems::activate_trigger;

pub mod bundles;
pub mod components;
pub mod systems;

pub struct TriggerPlugin;

impl Plugin for TriggerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(activate_trigger.in_set(OnUpdate(AppState::InGame)));
    }
}
