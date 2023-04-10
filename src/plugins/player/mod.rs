// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::prelude::{
    in_state, IntoSystemAppConfig, IntoSystemConfig, IntoSystemConfigs, OnEnter, OnUpdate, Plugin,
};

use crate::states::AppState;

use self::systems::{
    animation::player_animation,
    dialog::{player_close_npc_dialog, player_start_npc_dialog},
    magic::player_magic_attack,
    melee::player_melee_attack,
    movement::{player_movement, player_movement_input, player_movement_reset},
    mushroom::{player_eat_mushroom, player_update_mushroom_effect},
    take_damage::{
        player_death, player_enemy_collision, player_is_stunned, player_update_health_bar,
    },
};

use super::game::states::GameState;

pub mod bundles;
pub mod components;
pub mod states;
pub mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(player_movement_reset.in_schedule(OnEnter(GameState::Running)))
            .add_systems(
                (
                    player_movement_input.before(player_movement),
                    player_movement,
                    player_enemy_collision,
                    player_is_stunned,
                    player_death,
                    player_update_health_bar,
                    player_melee_attack,
                    player_magic_attack,
                    player_eat_mushroom,
                    player_update_mushroom_effect,
                    player_start_npc_dialog,
                    player_close_npc_dialog,
                    player_animation,
                )
                    .in_set(OnUpdate(AppState::InGame))
                    .distributive_run_if(in_state(GameState::Running)),
            );
    }
}
