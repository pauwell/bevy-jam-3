// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::{
    prelude::{
        default, App, Color, Commands, DespawnRecursiveExt, Entity, Input, IntoSystemAppConfig,
        IntoSystemConfig, KeyCode, NextState, OnEnter, OnExit, OnUpdate, Plugin, Query, Res,
        ResMut, TextBundle, Vec2, Without,
    },
    text::{TextAlignment, TextSection, TextStyle},
    ui::{AlignSelf, PositionType, Size, Style, UiRect},
    window::Window,
};

use crate::{
    assets::GameAssets,
    plugins::{
        camera::{bundles::GameCameraBundle, components::GameCameraState},
        npc::{bundles::NpcBundle, components::NpcProfession},
    },
    states::AppState,
};

use super::atlas::resources::GameAtlases;

pub struct IntroPlugin;

impl Plugin for IntroPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((
            intro_setup.in_schedule(OnEnter(AppState::Introduction)),
            intro_start_game.in_set(OnUpdate(AppState::Introduction)),
            intro_cleanup.in_schedule(OnExit(AppState::Introduction)),
        ));
    }
}

pub fn intro_setup(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    game_atlases: Res<GameAtlases>,
) {
    commands.spawn(GameCameraBundle::new(0.2, GameCameraState::Static));

    commands.spawn(NpcBundle::new(
        game_atlases.npc_andrea.clone(),
        "Andrea",
        "",
        NpcProfession::None,
        Vec2 { x: 16.0, y: 10.0 },
        Vec2 { x: 0.0, y: 0.0 },
        14.0,
    ));

    commands.spawn((TextBundle::from_sections([
        TextSection::new(
            "Fungal Fury!",
            TextStyle {
                font: game_assets.font_bold.clone(),
                font_size: 100.0,
                color: Color::DARK_GREEN,
            },
        ),
        TextSection::new(
            "\nYou're wandering through an unfamiliar, dense forest
            when you come across a group of people who are in dire need
            of your assistance. Strange, slimy creatures have taken over
            the path to their home. They explain that you must eliminate
            the creatures' nests, of which there are three in total.
            However, the only way to destroy the nests is by using some
            peculiar mushrooms that are consumed by those beings.
            You must defeat the creatures by utilizing their own abilities
            against them, but be cautious of the mushrooms' consequences...\n",
            TextStyle {
                font: game_assets.font_bold.clone(),
                font_size: 30.0,
                color: Color::GRAY,
            },
        ),
        TextSection::new(
            "\nPress [Space] To Start",
            TextStyle {
                font: game_assets.font_bold.clone(),
                font_size: 60.0,
                color: Color::SEA_GREEN,
            },
        ),
    ])
    .with_text_alignment(TextAlignment::Center)
    .with_style(Style {
        size: Size {
            width: bevy::ui::Val::Percent(100.0),
            ..default()
        },
        position: UiRect::left(bevy::ui::Val::Px(85.0)),
        position_type: PositionType::Relative,
        align_self: AlignSelf::Center,
        ..default()
    }),));
}

pub fn intro_start_game(
    mut next_state: ResMut<NextState<AppState>>,
    keyboard: Res<Input<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        next_state.set(AppState::InGame);
    }
}

pub fn intro_cleanup(mut commands: Commands, query: Query<Entity, Without<Window>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
