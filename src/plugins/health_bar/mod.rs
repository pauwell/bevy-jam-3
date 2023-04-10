// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::prelude::{in_state, App, IntoSystemConfig, IntoSystemConfigs, OnUpdate, Plugin};

use crate::states::AppState;

use self::{
    events::HealthBarFillPercentEvent,
    systems::{
        health_bar_handle_percent_filled_event, health_bar_spawn, health_bar_update_percent_filled,
    },
};

use super::game::states::GameState;

pub mod bundles;
pub mod components;
pub mod events;
pub mod systems;

pub struct HealthBarPlugin;

impl Plugin for HealthBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<HealthBarFillPercentEvent>().add_systems(
            (
                health_bar_spawn,
                health_bar_handle_percent_filled_event,
                health_bar_update_percent_filled,
            )
                .in_set(OnUpdate(AppState::InGame))
                .distributive_run_if(in_state(GameState::Running)),
        );
    }
}
