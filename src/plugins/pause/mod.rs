// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use bevy::prelude::{in_state, App, IntoSystemConfig, OnUpdate, Plugin};

use crate::states::AppState;

use self::systems::display_pause_text;

use super::game::states::GameState;

pub mod systems;

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            display_pause_text
                .in_set(OnUpdate(AppState::InGame))
                .run_if(in_state(GameState::Paused)),
        );
    }
}
