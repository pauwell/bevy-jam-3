// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::prelude::{in_state, App, IntoSystemConfig, IntoSystemConfigs, OnUpdate, Plugin};

use crate::states::AppState;

use self::systems::{
    death::{enemy_check_if_dead, enemy_on_death, enemy_update_death},
    movement::enemy_random_movement,
    ordering::enemy_calculate_z_index,
    take_damage::{
        enemy_is_stunned, enemy_take_magic_damage, enemy_take_melee_damage, enemy_update_health_bar,
    },
};

use super::game::states::GameState;

pub mod bundles;
pub mod components;
pub mod states;
pub mod systems;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                enemy_random_movement,
                enemy_take_melee_damage,
                enemy_take_magic_damage,
                enemy_is_stunned,
                enemy_check_if_dead,
                enemy_on_death,
                enemy_update_death,
                enemy_update_health_bar,
                enemy_calculate_z_index,
            )
                .in_set(OnUpdate(AppState::InGame))
                .distributive_run_if(in_state(GameState::Running)),
        );
    }
}
