// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::prelude::{in_state, App, IntoSystemConfig, IntoSystemConfigs, OnUpdate, Plugin};

use crate::states::AppState;

use self::systems::{
    death::{spawner_check_if_dead, spawner_on_death, spawner_update_death},
    spawn::spawner_spawn_enemies,
    take_damage::{spawner_is_stunned, spawner_take_melee_damage, spawner_update_health_bar},
};

use super::game::states::GameState;

pub mod bundles;
pub mod components;
pub mod states;
pub mod systems;

pub struct SpawnerPlugin;

impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                spawner_spawn_enemies,
                spawner_take_melee_damage,
                spawner_is_stunned,
                spawner_check_if_dead,
                spawner_on_death,
                spawner_update_death,
                spawner_update_health_bar,
            )
                .in_set(OnUpdate(AppState::InGame))
                .distributive_run_if(in_state(GameState::Running)),
        );
    }
}
