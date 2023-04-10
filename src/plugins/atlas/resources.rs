// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::{
    prelude::{Handle, Resource},
    sprite::TextureAtlas,
};

#[derive(Resource)]
pub struct GameAtlases {
    pub player: Handle<TextureAtlas>,
    pub npc_piet: Handle<TextureAtlas>,
    pub npc_marcus: Handle<TextureAtlas>,
    pub npc_andrea: Handle<TextureAtlas>,
    pub npc_cat: Handle<TextureAtlas>,
    pub enemy: Handle<TextureAtlas>,
    pub tileset: Handle<TextureAtlas>,
    pub spell: Handle<TextureAtlas>,
    pub spawner: Handle<TextureAtlas>,
}
