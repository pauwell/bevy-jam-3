// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::{
    prelude::{
        Commands, DespawnRecursiveExt, Entity, EventReader, Input, KeyCode, NextState, Query, Res,
        ResMut, Vec2, Without,
    },
    window::{Window, WindowFocused},
};

use crate::plugins::{
    atlas::resources::GameAtlases,
    camera::{bundles::GameCameraBundle, components::GameCameraState},
    player::bundles::PlayerBundle,
};

use super::{
    resources::GameProgress,
    states::{GameState, LevelState},
};

pub fn game_setup(
    mut commands: Commands,
    game_atlases: Res<GameAtlases>,
    mut game_state: ResMut<NextState<GameState>>,
    mut level_state: ResMut<NextState<LevelState>>,
    mut game_progress: ResMut<GameProgress>,
) {
    *game_progress = GameProgress::default();

    commands.spawn(GameCameraBundle::new(0.2, GameCameraState::FollowPlayer));

    commands.spawn(PlayerBundle::new(
        game_atlases.player.clone(),
        Vec2 { x: 32.0, y: 32.0 },
        Vec2 { x: 80.0, y: 40.0 },
        15.0,
        65.0,
        18.0,
        10.0,
        90.0,
        150.0,
        0.2,
    ));

    game_state.set(GameState::Running);
    level_state.set(LevelState::Overworld);
}

pub fn game_cleanup_on_enter(mut commands: Commands, query: Query<Entity, Without<Window>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn game_pause(
    mut game_state: ResMut<NextState<GameState>>,
    mut window_focus_event_reader: EventReader<WindowFocused>,
    mut keyboard: ResMut<Input<KeyCode>>,
) {
    for window_focus_event in window_focus_event_reader.iter() {
        if window_focus_event.focused == false {
            game_state.set(GameState::Paused);
            keyboard.reset_all();
            return;
        }
    }

    if keyboard.just_pressed(KeyCode::P) {
        game_state.set(GameState::Paused);
        keyboard.reset_all();
    }
}

pub fn game_unpause(
    mut game_state: ResMut<NextState<GameState>>,
    mut keyboard: ResMut<Input<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::P) {
        game_state.set(GameState::Running);
        keyboard.reset_all();
    }
}
