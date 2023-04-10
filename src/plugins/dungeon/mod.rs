// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::prelude::{App, IntoSystemAppConfig, OnEnter, OnExit, Plugin};

use self::systems::{dungeon_clear, dungeon_setup};

use super::game::states::LevelState;

pub mod systems;

pub struct DungeonPlugin;

impl Plugin for DungeonPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(dungeon_setup.in_schedule(OnEnter(LevelState::Dungeon)))
            .add_system(dungeon_clear.in_schedule(OnExit(LevelState::Dungeon)));
    }
}
