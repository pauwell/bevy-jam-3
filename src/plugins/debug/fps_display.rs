// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::{
        App, Color, Commands, Component, IntoSystemAppConfig, IntoSystemConfig, OnEnter, OnUpdate,
        Plugin, Query, Res, TextBundle, With,
    },
    text::{Text, TextSection, TextStyle},
};

use crate::{assets::GameAssets, states::AppState};

#[derive(Component)]
struct FpsDisplay;

pub struct FpsDisplayPlugin;

impl Plugin for FpsDisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_systems((
                setup.in_schedule(OnEnter(AppState::InGame)),
                update_fps_display.in_set(OnUpdate(AppState::InGame)),
            ));
    }
}

fn setup(mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    font: assets.font_bold.clone(),
                    font_size: 60.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: assets.font_medium.clone(),
                font_size: 60.0,
                color: Color::GOLD,
            }),
        ]),
        FpsDisplay,
    ));
}

fn update_fps_display(
    diagnostics: Res<Diagnostics>,
    mut query: Query<&mut Text, With<FpsDisplay>>,
) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                text.sections[1].value = format!("{value:.2}");
            }
        }
    }
}
