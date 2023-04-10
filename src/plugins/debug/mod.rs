// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::prelude::Plugin;
use bevy_inspector_egui::quick::{StateInspectorPlugin, WorldInspectorPlugin};

use crate::states::AppState;

use self::fps_display::FpsDisplayPlugin;

pub mod fps_display;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        #[cfg(debug_assertions)]
        app.add_plugin(WorldInspectorPlugin::new())
            .add_plugin(StateInspectorPlugin::<AppState>::default())
            .add_plugin(FpsDisplayPlugin);
    }
}
