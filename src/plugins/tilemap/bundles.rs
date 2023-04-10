// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::{
    prelude::{Bundle, Color, Handle, SpatialBundle, UVec2, Vec2},
    sprite::TextureAtlas,
};

use super::components::{
    Tilemap, TilemapColliders, TilemapMapSize, TilemapSolidTextureIndices, TilemapTextureHandle,
    TilemapTextureIndices, TilemapTileColor, TilemapTileSize,
};

#[derive(Bundle)]
pub struct TilemapBundle {
    tag: Tilemap,
    texture: TilemapTextureHandle,
    tile_size: TilemapTileSize,
    map_size: TilemapMapSize,
    texture_indices: TilemapTextureIndices,
    solid_texture_indices: TilemapSolidTextureIndices,
    colliders: TilemapColliders,
    tile_color: TilemapTileColor,
    #[bundle]
    spatial: SpatialBundle,
}

impl TilemapBundle {
    pub fn new(
        texture: Handle<TextureAtlas>,
        tile_size: Vec2,
        map_size: UVec2,
        texture_index_map: Vec<Vec<u32>>,
        solid_texture_indices: Vec<u32>,
        custom_tile_color: Option<Color>,
    ) -> Self {
        assert!(!texture_index_map.is_empty());
        assert_eq!(map_size.y, texture_index_map.len() as u32);
        assert_eq!(map_size.x, texture_index_map.get(0).unwrap().len() as u32);

        let mut tile_color = Color::WHITE;
        if let Some(custom_tile_color_some) = custom_tile_color {
            tile_color = custom_tile_color_some;
        }

        TilemapBundle {
            tag: Tilemap,
            texture: TilemapTextureHandle(texture),
            tile_size: TilemapTileSize(tile_size),
            map_size: TilemapMapSize(map_size),
            texture_indices: TilemapTextureIndices(texture_index_map),
            solid_texture_indices: TilemapSolidTextureIndices(solid_texture_indices),
            colliders: TilemapColliders(vec![]),
            spatial: SpatialBundle::INHERITED_IDENTITY,
            tile_color: TilemapTileColor(tile_color),
        }
    }
}
