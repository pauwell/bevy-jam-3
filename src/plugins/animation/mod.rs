// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::prelude::*;

use crate::states::AppState;

use self::{
    events::AnimationStartEvent,
    systems::{handle_animation_start_event, update_active_animation_clips},
};

use super::game::states::GameState;

pub mod bundles;
pub mod components;
pub mod events;
pub mod systems;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AnimationStartEvent>().add_systems(
            (update_active_animation_clips, handle_animation_start_event)
                .in_set(OnUpdate(AppState::InGame))
                .distributive_run_if(in_state(GameState::Running)),
        );
    }
}
