// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::prelude::{in_state, App, IntoSystemConfig, IntoSystemConfigs, OnUpdate, Plugin};

use crate::states::AppState;

use self::systems::{mushroom_despawn, mushroom_spawn_on_enemy_death};

use super::game::states::GameState;

pub mod bundles;
pub mod components;
pub mod systems;

pub struct MushroomPlugin;

impl Plugin for MushroomPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (mushroom_spawn_on_enemy_death, mushroom_despawn)
                .in_set(OnUpdate(AppState::InGame))
                .distributive_run_if(in_state(GameState::Running)),
        );
    }
}
