// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::prelude::{in_state, App, IntoSystemConfig, OnUpdate, Plugin};

use crate::states::AppState;

use self::systems::calculate_z_index;

use super::game::states::GameState;

pub mod bundles;
pub mod components;
pub mod systems;

pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            calculate_z_index
                .in_set(OnUpdate(AppState::InGame))
                .run_if(in_state(GameState::Running)),
        );
    }
}
