// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::{
    prelude::{Assets, Commands, NextState, Res, ResMut, Vec2},
    sprite::TextureAtlas,
};

use crate::{assets::GameAssets, plugins::atlas::resources::GameAtlases, states::AppState};

pub(super) fn build_atlases(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let texture_atlas_player = TextureAtlas::from_grid(
        game_assets.image_player.clone(),
        Vec2::new(32.0, 32.0),
        4,
        9,
        None,
        None,
    );

    let texture_atlas_npc_piet = TextureAtlas::from_grid(
        game_assets.image_npc_piet.clone(),
        Vec2::new(16.0, 20.0),
        2,
        1,
        None,
        None,
    );

    let texture_atlas_npc_marcus = TextureAtlas::from_grid(
        game_assets.image_npc_marcus.clone(),
        Vec2::new(16.0, 20.0),
        2,
        1,
        None,
        None,
    );

    let texture_atlas_npc_andrea = TextureAtlas::from_grid(
        game_assets.image_npc_andrea.clone(),
        Vec2::new(16.0, 20.0),
        2,
        1,
        None,
        None,
    );

    let texture_atlas_npc_cat = TextureAtlas::from_grid(
        game_assets.image_npc_cat.clone(),
        Vec2::new(16.0, 16.0),
        2,
        1,
        None,
        None,
    );

    let texture_atlas_enemy = TextureAtlas::from_grid(
        game_assets.image_enemy.clone(),
        Vec2::new(15.0, 16.0),
        6,
        2,
        None,
        None,
    );

    let texture_atlas_tileset = TextureAtlas::from_grid(
        game_assets.image_tileset.clone(),
        Vec2::new(16.0, 16.0),
        5,
        6,
        None,
        None,
    );

    let texture_atlas_spell = TextureAtlas::from_grid(
        game_assets.image_spell.clone(),
        Vec2::new(16.0, 16.0),
        3,
        2,
        None,
        None,
    );

    let texture_atlas_spawner = TextureAtlas::from_grid(
        game_assets.image_spawner.clone(),
        Vec2::new(16.0, 16.0),
        4,
        2,
        None,
        None,
    );

    commands.insert_resource(GameAtlases {
        player: texture_atlases.add(texture_atlas_player),
        npc_piet: texture_atlases.add(texture_atlas_npc_piet),
        npc_marcus: texture_atlases.add(texture_atlas_npc_marcus),
        npc_andrea: texture_atlases.add(texture_atlas_npc_andrea),
        npc_cat: texture_atlases.add(texture_atlas_npc_cat),
        enemy: texture_atlases.add(texture_atlas_enemy),
        tileset: texture_atlases.add(texture_atlas_tileset),
        spell: texture_atlases.add(texture_atlas_spell),
        spawner: texture_atlases.add(texture_atlas_spawner),
    });

    next_state.set(AppState::Introduction);
}
