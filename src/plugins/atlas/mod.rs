// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::prelude::{IntoSystemAppConfig, OnEnter, Plugin};

use crate::states::AppState;

use self::systems::build_atlases;

pub mod resources;
pub mod systems;

pub struct AtlasPlugin;

impl Plugin for AtlasPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(build_atlases.in_schedule(OnEnter(AppState::BuildingAtlases)));
    }
}
