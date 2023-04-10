// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::{
    prelude::{Color, Component, Handle, Rect, UVec2, Vec2},
    sprite::TextureAtlas,
};

#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tilemap;

#[derive(Component, Default, Debug, Clone)]
pub struct TilemapTile;

#[derive(Component, Default, Debug, Clone)]
pub struct TilemapTextureHandle(pub Handle<TextureAtlas>);

#[derive(Component, Default, Debug, Clone, Copy)]
pub struct TilemapTileColor(pub Color);

#[derive(Component, Default, Debug, Clone, Copy)]
pub struct TilemapTileSize(pub Vec2);

#[derive(Component, Default, Debug, Clone, Copy)]
pub struct TilemapMapSize(pub UVec2);

#[derive(Component, Default, Debug, Clone, PartialEq, Eq)]
pub struct TilemapTextureIndices(pub Vec<Vec<u32>>);

#[derive(Component, Default, Debug, Clone, PartialEq, Eq)]
pub struct TilemapSolidTextureIndices(pub Vec<u32>);

#[derive(Component, Default, Debug, Clone, PartialEq)]
pub struct TilemapColliders(pub Vec<Rect>);
