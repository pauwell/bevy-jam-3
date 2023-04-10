// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::{prelude::States, reflect::Reflect};

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States, Reflect)]
pub enum AppState {
    #[default]
    LoadingAssets,
    BuildingAtlases,
    Introduction,
    InGame,
    GameOver,
    GameWin,
}
