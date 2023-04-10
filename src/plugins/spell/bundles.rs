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

use crate::plugins::animation::{bundles::AnimationBundle, components::AnimationClip};

use super::components::{Spell, SpellDamage, SpellTimer, SpellVelocity};

#[derive(Bundle, Default)]
pub struct SpellBundle {
    tag: Spell,
    velocity: SpellVelocity,
    damage: SpellDamage,
    timer: SpellTimer,
    #[bundle]
    animation: AnimationBundle,
    #[bundle]
    sprite: SpriteSheetBundle,
}

impl SpellBundle {
    pub fn new(
        texture: Handle<TextureAtlas>,
        position: Vec2,
        velocity: Vec2,
        damage: f32,
        seconds_alive: f32,
        z_index: f32,
    ) -> Self {
        let timer = SpellTimer(Timer::from_seconds(
            seconds_alive,
            bevy::time::TimerMode::Once,
        ));

        SpellBundle {
            tag: Spell,
            velocity: SpellVelocity(velocity),
            damage: SpellDamage(damage),
            timer,
            animation: AnimationBundle::default()
                .with_active_clip(&AnimationClip {
                    name: "idle".into(),
                    index_first: 0,
                    index_last: 2,
                    playback_speed: 0.4,
                })
                .with_clip(&AnimationClip {
                    name: "death".into(),
                    index_first: 3,
                    index_last: 5,
                    playback_speed: 0.4,
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
