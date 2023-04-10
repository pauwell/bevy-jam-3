// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::prelude::{in_state, App, IntoSystemConfig, IntoSystemConfigs, OnUpdate, Plugin};

use crate::states::AppState;

use self::systems::{spell_movement, spell_on_death, spell_update_death};

use super::game::states::GameState;

pub mod bundles;
pub mod components;
pub mod systems;

pub struct SpellPlugin;

impl Plugin for SpellPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (spell_movement, spell_on_death, spell_update_death)
                .in_set(OnUpdate(AppState::InGame))
                .distributive_run_if(in_state(GameState::Running)),
        );
    }
}
