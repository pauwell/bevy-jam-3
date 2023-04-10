// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::{
    prelude::{
        Camera2d, Commands, DespawnRecursiveExt, Entity, Name, NextState, Query, Res, ResMut,
        Transform, Vec2, With, Without,
    },
    window::Window,
};

use crate::plugins::{
    atlas::resources::GameAtlases,
    dialog_box::components::DialogBox,
    game::{
        resources::GameProgress,
        states::{GameState, LevelState, StoryState},
    },
    health_bar::components::HealthBar,
    player::components::{Player, PlayerDirection},
    spawner::bundles::SpawnerBundle,
    tilemap::bundles::TilemapBundle,
    trigger::bundles::LevelTriggerBundle,
};

pub fn dungeon_setup(
    mut commands: Commands,
    mut player_query: Query<(&mut Transform, &mut PlayerDirection), With<Player>>,
    mut story_state: ResMut<NextState<StoryState>>,
    mut game_state: ResMut<NextState<GameState>>,
    game_atlases: Res<GameAtlases>,
    game_progress: Res<GameProgress>,
) {
    game_state.set(GameState::Running);
    story_state.set(StoryState::MiddleGame);

    if let Ok((mut player_transform, mut player_direction)) = player_query.get_single_mut() {
        player_transform.translation.y = 15.0;
        *player_direction = PlayerDirection::Up;
    }

    commands
        .spawn(TilemapBundle::new(
            game_atlases.tileset.clone(),
            (16.0, 16.0).into(),
            (15, 20).into(),
            vec![
                vec![5, 6, 6, 6, 23, 24, 6, 6, 6, 6, 6, 6, 6, 6, 7],
                vec![3, 4, 4, 4, 4, 4, 29, 9, 4, 4, 4, 4, 9, 4, 8],
                vec![3, 4, 9, 4, 26, 9, 4, 4, 4, 26, 4, 4, 9, 4, 8],
                vec![3, 16, 4, 4, 4, 4, 4, 9, 4, 4, 4, 4, 9, 4, 8],
                vec![3, 11, 9, 14, 9, 4, 4, 4, 9, 4, 9, 4, 9, 4, 8],
                vec![3, 4, 4, 19, 4, 4, 9, 4, 4, 4, 9, 4, 9, 4, 8],
                vec![3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 9, 4, 9, 4, 8],
                vec![3, 4, 4, 4, 9, 4, 4, 4, 4, 4, 4, 4, 9, 4, 8],
                vec![3, 4, 15, 4, 4, 9, 18, 4, 4, 29, 4, 4, 9, 4, 8],
                vec![3, 4, 10, 4, 4, 4, 13, 4, 4, 4, 4, 4, 9, 4, 8],
                vec![3, 4, 4, 4, 4, 4, 4, 4, 4, 25, 4, 4, 9, 4, 8],
                vec![3, 4, 4, 4, 25, 9, 4, 4, 4, 4, 4, 4, 9, 4, 8],
                vec![3, 4, 9, 4, 4, 4, 4, 19, 4, 4, 4, 4, 9, 4, 8],
                vec![3, 4, 4, 18, 4, 9, 4, 4, 4, 4, 4, 25, 9, 4, 8],
                vec![3, 9, 4, 13, 4, 4, 9, 4, 12, 4, 4, 4, 9, 4, 8],
                vec![3, 4, 29, 4, 9, 4, 25, 4, 4, 4, 4, 4, 9, 4, 8],
                vec![3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 9, 4, 8],
                vec![3, 12, 9, 4, 9, 4, 4, 9, 4, 26, 4, 4, 9, 4, 8],
                vec![3, 4, 9, 4, 29, 4, 4, 4, 4, 4, 4, 4, 9, 4, 8],
                vec![0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2],
            ],
            vec![0, 1, 2, 3, 5, 6, 7, 8, 10, 11, 12, 13, 18, 20, 21],
            None,
        ))
        .insert(Name::from("OverworldTilemap"));

    if game_progress.destroyed_spawners.contains("Spawner A") == false {
        commands.spawn(SpawnerBundle::new(
            "Spawner A",
            game_atlases.spawner.clone(),
            500.0,
            Vec2::new(80.0, 250.0),
            10.0,
            0.3,
            11.0,
        ));
    }

    if game_progress.destroyed_spawners.contains("Spawner B") == false {
        commands.spawn(SpawnerBundle::new(
            "Spawner B",
            game_atlases.spawner.clone(),
            500.0,
            Vec2::new(150.0, 230.0),
            14.0,
            0.3,
            11.0,
        ));
    }

    if game_progress.destroyed_spawners.contains("Spawner C") == false {
        commands.spawn(SpawnerBundle::new(
            "Spawner C",
            game_atlases.spawner.clone(),
            500.0,
            Vec2::new(120.0, 130.0),
            10.0,
            0.3,
            11.0,
        ));
    }

    commands
        .spawn(LevelTriggerBundle::new(
            Vec2::new(72.0, 0.0),
            LevelState::Overworld,
        ))
        .insert(Name::from("LevelTrigger"));
}

pub fn dungeon_clear(
    mut commands: Commands,
    query: Query<
        Entity,
        (
            Without<Window>,
            Without<Camera2d>,
            Without<Player>,
            Without<DialogBox>,
            Without<HealthBar>,
        ),
    >,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
