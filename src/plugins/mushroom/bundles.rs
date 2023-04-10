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

use super::components::{
    Mushroom, MushroomDespawnTimer, MushroomEffect, MushroomEffectActiveTimer,
};

#[derive(Bundle)]
pub struct MushroomBundle {
    tag: Mushroom,
    despawn_timer: MushroomDespawnTimer,
    active_timer: MushroomEffectActiveTimer,
    effect: MushroomEffect,
    #[bundle]
    sprite: SpriteSheetBundle,
}

impl MushroomBundle {
    pub fn new(
        texture: Handle<TextureAtlas>,
        effect: MushroomEffect,
        position: Vec2,
        despawn_after_seconds: f32,
        effect_duration_seconds: f32,
        sprite_index: usize,
        z_index: f32,
    ) -> Self {
        let despawn_timer = MushroomDespawnTimer(Timer::from_seconds(
            despawn_after_seconds,
            bevy::time::TimerMode::Once,
        ));

        let active_timer = MushroomEffectActiveTimer(Timer::from_seconds(
            effect_duration_seconds,
            bevy::time::TimerMode::Once,
        ));

        MushroomBundle {
            tag: Mushroom,
            despawn_timer,
            active_timer,
            effect,
            sprite: SpriteSheetBundle {
                sprite: TextureAtlasSprite::new(sprite_index),
                texture_atlas: texture.clone(),
                transform: Transform::from_xyz(position.x, position.y, z_index),
                ..default()
            },
        }
    }
}
