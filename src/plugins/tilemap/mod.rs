// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::prelude::{in_state, App, IntoSystemConfigs, OnUpdate, Plugin};

use crate::states::AppState;

use self::systems::{colliders::spawn_colliders, tiles::spawn_tiles};

use super::game::states::GameState;

pub mod bundles;
pub mod components;
pub mod systems;

pub struct TilemapPlugin;

impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (spawn_tiles, spawn_colliders)
                .in_set(OnUpdate(AppState::InGame))
                .distributive_run_if(in_state(GameState::Running)),
        );
    }
}
