// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::prelude::{in_state, IntoSystemConfig, OnUpdate, Plugin};

use crate::states::AppState;

use self::systems::update_game_camera;

use super::game::states::GameState;

pub mod bundles;
pub mod components;
pub mod systems;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(
            update_game_camera
                .in_set(OnUpdate(AppState::InGame))
                .run_if(in_state(GameState::Running)),
        );
    }
}
