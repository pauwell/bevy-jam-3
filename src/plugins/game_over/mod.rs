// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::prelude::{App, IntoSystemAppConfig, IntoSystemConfig, OnEnter, OnUpdate, Plugin};

use crate::states::AppState;

use self::systems::{game_over_message, game_over_start_new_game};

pub mod systems;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((
            game_over_message.in_schedule(OnEnter(AppState::GameOver)),
            game_over_start_new_game.in_set(OnUpdate(AppState::GameOver)),
        ));
    }
}
