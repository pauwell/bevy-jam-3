// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::prelude::{in_state, IntoSystemAppConfig, IntoSystemConfigs, OnEnter, OnUpdate, Plugin};

use crate::states::AppState;

use self::{
    events::{DialogBoxCloseEvent, DialogBoxStartDialogEvent},
    systems::{
        close_dialog_box, display_dialog_box, handle_dialog_box_close_event,
        handle_dialog_box_start_event, setup,
    },
};

use super::game::states::GameState;

pub mod components;
pub mod events;
pub mod systems;

pub struct DialogBoxPlugin;

impl Plugin for DialogBoxPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<DialogBoxStartDialogEvent>()
            .add_event::<DialogBoxCloseEvent>()
            .add_system(setup.in_schedule(OnEnter(AppState::InGame)))
            .add_systems(
                (
                    display_dialog_box,
                    close_dialog_box,
                    handle_dialog_box_start_event,
                    handle_dialog_box_close_event,
                )
                    .in_set(OnUpdate(AppState::InGame))
                    .distributive_run_if(in_state(GameState::Running)),
            );
    }
}
