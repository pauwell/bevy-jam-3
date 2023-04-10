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
    actor::components::Actor,
    animation::{bundles::AnimationBundle, components::AnimationClip},
    health_bar::components::HealthBar,
};

use super::{
    components::{
        Enemy, EnemyChangeDirectionTimer, EnemyDamage, EnemyDeathTimer, EnemyDirection,
        EnemyHealth, EnemyHealthMax, EnemyIsStunnedTimer, EnemySize, EnemyType, EnemyWalkSpeed,
    },
    states::EnemyState,
};

#[derive(Bundle, Default)]
pub struct EnemyBundle {
    tag: Enemy,
    actor: Actor,
    size: EnemySize,
    direction_timer: EnemyChangeDirectionTimer,
    stunned_timer: EnemyIsStunnedTimer,
    death_timer: EnemyDeathTimer,
    enemy_type: EnemyType,
    state: EnemyState,
    direction: EnemyDirection,
    health: EnemyHealth,
    health_max: EnemyHealthMax,
    health_bar: HealthBar,
    walk_speed: EnemyWalkSpeed,
    damage: EnemyDamage,
    #[bundle]
    animation: AnimationBundle,
    #[bundle]
    sprite: SpriteSheetBundle,
}

impl EnemyBundle {
    pub fn new(
        texture: Handle<TextureAtlas>,
        enemy_type: EnemyType,
        size: Vec2,
        position: Vec2,
        z_index: f32,
        walk_speed: f32,
        damage: f32,
        health: f32,
        seconds_new_direction: f32,
        seconds_stunned: f32,
    ) -> Self {
        let direction_timer = EnemyChangeDirectionTimer(Timer::from_seconds(
            seconds_new_direction,
            bevy::time::TimerMode::Repeating,
        ));
        let stunned_timer = EnemyIsStunnedTimer(Timer::from_seconds(
            seconds_stunned,
            bevy::time::TimerMode::Once,
        ));
        let death_timer = EnemyDeathTimer(Timer::from_seconds(0.5, bevy::time::TimerMode::Once));

        EnemyBundle {
            tag: Enemy,
            actor: Actor,
            size: EnemySize(size),
            direction_timer,
            stunned_timer,
            death_timer,
            enemy_type,
            state: EnemyState::default(),
            direction: EnemyDirection::default(),
            health: EnemyHealth(health),
            health_max: EnemyHealthMax(health),
            health_bar: HealthBar,
            walk_speed: EnemyWalkSpeed(walk_speed),
            damage: EnemyDamage(damage),
            animation: AnimationBundle::default()
                .with_active_clip(&AnimationClip {
                    name: "idle_down".into(),
                    index_first: 0,
                    index_last: 5,
                    playback_speed: 0.1,
                })
                .with_clip(&AnimationClip {
                    name: "death".into(),
                    index_first: 6,
                    index_last: 11,
                    playback_speed: 0.1,
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
