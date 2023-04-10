// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::{
    prelude::{Bundle, Handle, Transform, Vec2},
    sprite::{SpriteSheetBundle, TextureAtlas, TextureAtlasSprite},
    time::Timer,
    utils::default,
};

use crate::plugins::{
    animation::{bundles::AnimationBundle, components::AnimationClip},
    health_bar::components::HealthBar,
};

use super::{
    components::{
        Spawner, SpawnerDeathTimer, SpawnerHealth, SpawnerHealthMax, SpawnerIsStunnedTimer,
        SpawnerName, SpawnerTimer,
    },
    states::SpawnerState,
};

#[derive(Bundle)]
pub struct SpawnerBundle {
    tag: Spawner,
    name: SpawnerName,
    spawn_timer: SpawnerTimer,
    state: SpawnerState,
    health: SpawnerHealth,
    health_max: SpawnerHealthMax,
    health_bar: HealthBar,
    stunned_timer: SpawnerIsStunnedTimer,
    death_timer: SpawnerDeathTimer,
    #[bundle]
    animation: AnimationBundle,
    #[bundle]
    sprite: SpriteSheetBundle,
}

impl SpawnerBundle {
    pub fn new(
        name: &str,
        texture: Handle<TextureAtlas>,
        health: f32,
        position: Vec2,
        seconds_between: f32,
        seconds_stunned: f32,
        z_index: f32,
    ) -> Self {
        let mut spawn_timer = SpawnerTimer(Timer::from_seconds(
            seconds_between,
            bevy::time::TimerMode::Repeating,
        ));
        spawn_timer.0.set_elapsed(spawn_timer.0.duration());

        let stunned_timer = SpawnerIsStunnedTimer(Timer::from_seconds(
            seconds_stunned,
            bevy::time::TimerMode::Once,
        ));
        let death_timer = SpawnerDeathTimer(Timer::from_seconds(0.5, bevy::time::TimerMode::Once));

        SpawnerBundle {
            tag: Spawner,
            name: SpawnerName(name.into()),
            spawn_timer,
            state: SpawnerState::default(),
            health: SpawnerHealth(health),
            health_max: SpawnerHealthMax(health),
            health_bar: HealthBar,
            stunned_timer,
            death_timer,
            animation: AnimationBundle::default()
                .with_active_clip(&AnimationClip {
                    name: "idle".into(),
                    index_first: 0,
                    index_last: 3,
                    playback_speed: 0.2,
                })
                .with_clip(&AnimationClip {
                    name: "death".into(),
                    index_first: 4,
                    index_last: 7,
                    playback_speed: 0.5,
                }),
            sprite: SpriteSheetBundle {
                sprite: TextureAtlasSprite::new(0),
                texture_atlas: texture.clone(),
                transform: Transform::from_xyz(position.x, position.y, z_index),
                ..default()
            },
        }
    }
}
