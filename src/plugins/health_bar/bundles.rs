// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::{
    prelude::{Bundle, Color, Entity, Transform, Vec2},
    sprite::{Sprite, SpriteBundle},
    utils::default,
};

use super::components::{HealthBar, HealthBarPercentFilled, HealthBarSize, HealthBarTargetEntity};

#[derive(Bundle)]
pub struct HealthBarBundle {
    tag: HealthBar,
    target_entity: HealthBarTargetEntity,
    percent_filled: HealthBarPercentFilled,
    size: HealthBarSize,
    #[bundle]
    sprite: SpriteBundle,
}

impl HealthBarBundle {
    pub fn new(parent: Entity, position: Vec2, size: Vec2, z_index: f32) -> Self {
        HealthBarBundle {
            tag: HealthBar,
            target_entity: HealthBarTargetEntity(parent),
            percent_filled: HealthBarPercentFilled(100.0),
            size: HealthBarSize(size),
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::RED.with_a(0.8),
                    custom_size: Some(size),
                    ..default()
                },
                transform: Transform::from_translation(position.extend(z_index)),
                ..default()
            },
        }
    }
}
