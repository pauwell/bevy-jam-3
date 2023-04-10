// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::{
    prelude::{
        default, Color, Commands, EventWriter, Input, KeyCode, NextState, Res, ResMut, TextBundle,
    },
    text::{TextAlignment, TextSection, TextStyle},
    ui::{AlignSelf, PositionType, Size, Style, UiRect},
};

use crate::{
    assets::GameAssets,
    plugins::{
        audio::{events::AudioStartPlayEvent, AudioClip},
        game::states::StoryState,
    },
    states::AppState,
};

pub fn game_win_message(
    mut commands: Commands,
    game_assets: Res<GameAssets>,

    mut audio_start_play_event_writer: EventWriter<AudioStartPlayEvent>,
) {
    audio_start_play_event_writer.send(AudioStartPlayEvent(AudioClip::WinGame));
    commands.spawn((TextBundle::from_sections([
        TextSection::new(
            "You won!",
            TextStyle {
                font: game_assets.font_bold.clone(),
                font_size: 100.0,
                color: Color::GOLD,
            },
        ),
        TextSection::new(
            "\n\nPress [Space] To Start A New Game!",
            TextStyle {
                font: game_assets.font_bold.clone(),
                font_size: 60.0,
                color: Color::RED,
            },
        ),
    ])
    .with_text_alignment(TextAlignment::Center)
    .with_style(Style {
        position: UiRect::left(bevy::ui::Val::Percent(10.0)),
        size: Size {
            width: bevy::ui::Val::Percent(100.0),
            ..default()
        },
        position_type: PositionType::Absolute,
        align_self: AlignSelf::Center,
        ..default()
    }),));
}

pub fn game_win_start_new_game(
    keyboard: Res<Input<KeyCode>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_story_state: ResMut<NextState<StoryState>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        next_app_state.set(AppState::InGame);
        next_story_state.set(StoryState::FirstVisit);
    }
}
