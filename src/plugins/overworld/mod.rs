// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::prelude::{
    in_state, App, IntoSystemAppConfig, IntoSystemConfig, OnEnter, OnExit, OnUpdate, Plugin,
};

use self::systems::{
    overworld_changes_on_first_visit, overworld_changes_on_middle_game, overworld_changes_on_win,
    overworld_clear, overworld_setup,
};

use super::game::states::{LevelState, StoryState};

pub mod systems;

pub struct OverworldPlugin;

impl Plugin for OverworldPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(overworld_setup.in_schedule(OnEnter(LevelState::Overworld)))
            .add_systems((
                overworld_changes_on_first_visit
                    .in_set(OnUpdate(LevelState::Overworld))
                    .run_if(in_state(StoryState::FirstVisit)),
                overworld_changes_on_middle_game
                    .after(overworld_changes_on_first_visit)
                    .in_set(OnUpdate(LevelState::Overworld))
                    .run_if(in_state(StoryState::MiddleGame)),
                overworld_changes_on_win
                    .in_set(OnUpdate(LevelState::Overworld))
                    .run_if(in_state(StoryState::Win)),
            ))
            .add_system(overworld_clear.in_schedule(OnExit(LevelState::Overworld)));
    }
}
