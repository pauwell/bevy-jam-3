// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::{prelude::States, reflect::Reflect};

#[derive(Clone, Eq, PartialEq, Debug, Copy, Hash, Default, States, Reflect)]
pub enum GameState {
    #[default]
    Inactive,
    Running,
    Paused,
    GameOver,
    GameWon,
}

#[derive(Clone, Eq, PartialEq, Debug, Copy, Hash, Default, States, Reflect)]
pub enum LevelState {
    #[default]
    None,
    Overworld,
    Dungeon,
}

#[derive(Clone, Eq, PartialEq, Debug, Copy, Hash, Default, States, Reflect)]
pub enum StoryState {
    #[default]
    FirstVisit,
    MiddleGame,
    Win,
}
