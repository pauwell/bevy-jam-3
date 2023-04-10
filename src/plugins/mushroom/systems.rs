// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::{
    prelude::{info, Added, Commands, DespawnRecursiveExt, Entity, Query, Res, Transform},
    time::Time,
};
use rand::{thread_rng, Rng};

use crate::plugins::{
    atlas::resources::GameAtlases, enemy::components::EnemyIsDead,
    mushroom::bundles::MushroomBundle,
};

use super::components::{MushroomDespawnTimer, MushroomEffect};

pub fn mushroom_spawn_on_enemy_death(
    mut commands: Commands,
    query: Query<&Transform, Added<EnemyIsDead>>,
    game_atlases: Res<GameAtlases>,
) {
    for enemy_transform in query.iter() {
        let mut rng = thread_rng();

        // Chance to not spawn the mushroom.
        match rng.gen_range(0..10) {
            0 | 1 | 2 | 3 => (),
            _ => continue,
        }

        // Adding random effect to the mushroom.
        let random_effect = match rng.gen_range(0..8) {
            0 => MushroomEffect::LowHealth,
            1 => MushroomEffect::HealthBonus,
            2 | 3 => MushroomEffect::SlowWalk,
            4 | 5 => MushroomEffect::NoMagic,
            6 | 7 => MushroomEffect::NoDamageAgainstBlobs,
            _ => MushroomEffect::HealthBonus,
        };

        // Adding random effect duration to the mushroom.
        let random_effect_duration = rng.gen_range(5..15) as f32;

        commands.spawn(MushroomBundle::new(
            game_atlases.tileset.clone(),
            random_effect,
            enemy_transform.translation.truncate(),
            20.0,
            random_effect_duration,
            17,
            13.0,
        ));
    }
}

pub fn mushroom_despawn(
    mut commands: Commands,
    mut query: Query<(Entity, &mut MushroomDespawnTimer)>,
    delta: Res<Time>,
) {
    for (mushroom_entity, mut mushroom_despawn_timer) in query.iter_mut() {
        mushroom_despawn_timer.0.tick(delta.delta());
        if mushroom_despawn_timer.0.just_finished() {
            commands.entity(mushroom_entity).despawn_recursive();
            info!("Despawning mushroom");
        }
    }
}
