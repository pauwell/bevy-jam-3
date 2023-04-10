// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::prelude::{App, IntoSystemAppConfig, IntoSystemConfig, OnEnter, OnUpdate, Plugin};

use crate::states::AppState;

use self::systems::{game_win_message, game_win_start_new_game};

pub mod systems;

pub struct GameWinPlugin;

impl Plugin for GameWinPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((
            game_win_message.in_schedule(OnEnter(AppState::GameWin)),
            game_win_start_new_game.in_set(OnUpdate(AppState::GameWin)),
        ));
    }
}
