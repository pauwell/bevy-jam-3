// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::{
    prelude::{Bundle, Handle, Name, Transform, Vec2},
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
        Player, PlayerDirection, PlayerHealth, PlayerHealthMax, PlayerIsStunnedTimer,
        PlayerMagicAttackDamage, PlayerMagicAttackSpeed, PlayerMagicAttackTimer,
        PlayerMeleeAttackDamage, PlayerMeleeAttackHitPosition, PlayerMeleeAttackTimer, PlayerSize,
        PlayerVelocity, PlayerWalkSpeed,
    },
    states::PlayerState,
};

#[derive(Bundle)]
pub struct PlayerBundle {
    tag: Player,
    actor: Actor,
    name: Name,
    state: PlayerState,
    size: PlayerSize,
    direction: PlayerDirection,
    walk_speed: PlayerWalkSpeed,
    health: PlayerHealth,
    health_max: PlayerHealthMax,
    velocity: PlayerVelocity,
    stunned_timer: PlayerIsStunnedTimer,
    melee_attack_timer: PlayerMeleeAttackTimer,
    melee_attack_damage: PlayerMeleeAttackDamage,
    melee_hit_position: PlayerMeleeAttackHitPosition,
    magic_attack_timer: PlayerMagicAttackTimer,
    magic_attack_damage: PlayerMagicAttackDamage,
    magic_attack_speed: PlayerMagicAttackSpeed,
    health_bar: HealthBar,
    #[bundle]
    animation: AnimationBundle,
    #[bundle]
    sprite: SpriteSheetBundle,
}

impl PlayerBundle {
    pub fn new(
        texture: Handle<TextureAtlas>,
        size: Vec2,
        position: Vec2,
        z_index: f32,
        walk_speed: f32,
        melee_damage: f32,
        magic_damage: f32,
        magic_speed: f32,
        health: f32,
        seconds_stunned: f32,
    ) -> Self {
        PlayerBundle {
            tag: Player,
            actor: Actor,
            name: Name::new("Player"),
            size: PlayerSize(size),
            state: PlayerState::Idle,
            direction: PlayerDirection::Down,
            walk_speed: PlayerWalkSpeed(walk_speed),
            health: PlayerHealth(health),
            health_max: PlayerHealthMax(health),
            velocity: PlayerVelocity::default(),
            stunned_timer: PlayerIsStunnedTimer(Timer::from_seconds(
                seconds_stunned,
                bevy::time::TimerMode::Once,
            )),
            melee_attack_timer: PlayerMeleeAttackTimer(Timer::from_seconds(
                0.5,
                bevy::time::TimerMode::Once,
            )),
            melee_attack_damage: PlayerMeleeAttackDamage(melee_damage),
            melee_hit_position: PlayerMeleeAttackHitPosition(Vec2::ZERO),
            magic_attack_timer: PlayerMagicAttackTimer(Timer::from_seconds(
                0.3,
                bevy::time::TimerMode::Once,
            )),
            magic_attack_damage: PlayerMagicAttackDamage(magic_damage),
            magic_attack_speed: PlayerMagicAttackSpeed(magic_speed),
            animation: AnimationBundle::default()
                .with_clip(&AnimationClip {
                    name: "idle_up".into(),
                    index_first: 0,
                    index_last: 0,
                    playback_speed: 0.2,
                })
                .with_active_clip(&AnimationClip {
                    name: "idle_down".into(),
                    index_first: 4,
                    index_last: 4,
                    playback_speed: 0.2,
                })
                .with_clip(&AnimationClip {
                    name: "idle_left".into(),
                    index_first: 8,
                    index_last: 8,
                    playback_speed: 0.2,
                })
                .with_clip(&AnimationClip {
                    name: "idle_right".into(),
                    index_first: 12,
                    index_last: 12,
                    playback_speed: 0.2,
                })
                .with_clip(&AnimationClip {
                    name: "walk_up".into(),
                    index_first: 0,
                    index_last: 3,
                    playback_speed: 0.2,
                })
                .with_clip(&AnimationClip {
                    name: "walk_down".into(),
                    index_first: 4,
                    index_last: 7,
                    playback_speed: 0.2,
                })
                .with_clip(&AnimationClip {
                    name: "walk_left".into(),
                    index_first: 8,
                    index_last: 11,
                    playback_speed: 0.2,
                })
                .with_clip(&AnimationClip {
                    name: "walk_right".into(),
                    index_first: 12,
                    index_last: 15,
                    playback_speed: 0.2,
                })
                .with_clip(&AnimationClip {
                    name: "melee_up".into(),
                    index_first: 20,
                    index_last: 23,
                    playback_speed: 0.4,
                })
                .with_clip(&AnimationClip {
                    name: "melee_down".into(),
                    index_first: 24,
                    index_last: 27,
                    playback_speed: 0.4,
                })
                .with_clip(&AnimationClip {
                    name: "melee_left".into(),
                    index_first: 28,
                    index_last: 31,
                    playback_speed: 0.4,
                })
                .with_clip(&AnimationClip {
                    name: "melee_right".into(),
                    index_first: 32,
                    index_last: 35,
                    playback_speed: 0.4,
                })
                .with_clip(&AnimationClip {
                    name: "magic_up".into(),
                    index_first: 16,
                    index_last: 16,
                    playback_speed: 0.3,
                })
                .with_clip(&AnimationClip {
                    name: "magic_down".into(),
                    index_first: 17,
                    index_last: 17,
                    playback_speed: 0.3,
                })
                .with_clip(&AnimationClip {
                    name: "magic_left".into(),
                    index_first: 18,
                    index_last: 18,
                    playback_speed: 0.3,
                })
                .with_clip(&AnimationClip {
                    name: "magic_right".into(),
                    index_first: 19,
                    index_last: 19,
                    playback_speed: 0.3,
                }),
            sprite: SpriteSheetBundle {
                sprite: TextureAtlasSprite::new(0),
                texture_atlas: texture.clone(),
                transform: Transform::from_xyz(position.x, position.y, z_index),
                ..default()
            },
            health_bar: HealthBar,
        }
    }
}
