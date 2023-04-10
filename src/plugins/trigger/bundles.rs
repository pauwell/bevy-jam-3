// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::{
    prelude::{Bundle, Transform, Vec2, Vec3},
    sprite::{Sprite, SpriteBundle},
    utils::default,
};

use crate::plugins::game::states::LevelState;

use super::components::{Trigger, TriggerTargetLevel};

#[derive(Bundle)]
pub struct LevelTriggerBundle {
    tag: Trigger,
    target: TriggerTargetLevel,
    #[bundle]
    sprite: SpriteBundle,
}

impl LevelTriggerBundle {
    pub fn new(position: Vec2, target: LevelState) -> Self {
        LevelTriggerBundle {
            tag: Trigger,
            target: TriggerTargetLevel(target),
            sprite: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(5.0, 5.0)),
                    ..default()
                },
                visibility: bevy::prelude::Visibility::Hidden,
                transform: Transform::from_translation(position.extend(15.0))
                    .with_scale(Vec3::new(2.0, 2.0, 2.0)),
                ..default()
            },
        }
    }
}
