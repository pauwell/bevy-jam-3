// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use bevy::{
    prelude::{Commands, Name, Query, Res, Transform, Vec2, With},
    time::Time,
};

use crate::plugins::{
    atlas::resources::GameAtlases,
    enemy::{
        bundles::EnemyBundle,
        components::{Enemy, EnemyType},
    },
    player::states::PlayerState,
    spawner::components::{Spawner, SpawnerTimer},
};

pub fn spawner_spawn_enemies(
    mut commands: Commands,
    mut query: Query<(&Transform, &mut SpawnerTimer), With<Spawner>>,
    enemies_query: Query<&Enemy>,
    player_query: Query<&PlayerState>,
    game_atlases: Res<GameAtlases>,
    delta: Res<Time>,
) {
    if player_query.is_empty() {
        return;
    }

    let player_state = player_query
        .get_single()
        .expect("0 or more than 1 `Player` found.");

    if *player_state == PlayerState::Talk {
        return;
    }

    for (spawner_transform, mut spawner_timer) in query.iter_mut() {
        spawner_timer.0.tick(delta.delta());
        if spawner_timer.0.just_finished() {
            if enemies_query.iter().count() < 10 {
                commands
                    .spawn(EnemyBundle::new(
                        game_atlases.enemy.clone(),
                        EnemyType::Blob,
                        Vec2 { x: 15.0, y: 16.0 },
                        spawner_transform.translation.truncate(),
                        14.0,
                        20.0,
                        15.0,
                        45.0,
                        2.0,
                        1.0,
                    ))
                    .insert(Name::from("Enemy"));
            }
        }
    }
}
