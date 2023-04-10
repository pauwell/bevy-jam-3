// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::{
    prelude::{Bundle, Handle, Transform, Vec2},
    sprite::{SpriteSheetBundle, TextureAtlas, TextureAtlasSprite},
    utils::default,
};

use crate::plugins::{
    actor::components::Actor,
    animation::{bundles::AnimationBundle, components::AnimationClip},
};

use super::components::{Npc, NpcDialog, NpcDirection, NpcName, NpcProfession, NpcSize, NpcState};

#[derive(Bundle, Default)]
pub struct NpcBundle {
    tag: Npc,
    actor: Actor,
    name: NpcName,
    size: NpcSize,
    dialog: NpcDialog,
    profession: NpcProfession,
    state: NpcState,
    direction: NpcDirection,
    #[bundle]
    animation: AnimationBundle,
    #[bundle]
    sprite: SpriteSheetBundle,
}

impl NpcBundle {
    pub fn new(
        texture: Handle<TextureAtlas>,
        name: &str,
        dialog: &str,
        profession: NpcProfession,
        size: Vec2,
        position: Vec2,
        z_index: f32,
    ) -> Self {
        NpcBundle {
            tag: Npc,
            actor: Actor,
            name: NpcName(name.into()),
            size: NpcSize(size),
            dialog: NpcDialog(dialog.into()),
            profession,
            state: NpcState::default(),
            direction: NpcDirection::default(),
            animation: AnimationBundle::default().with_active_clip(&AnimationClip {
                name: "idle_down".into(),
                index_first: 0,
                index_last: 1,
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
