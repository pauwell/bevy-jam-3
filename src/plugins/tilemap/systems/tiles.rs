// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::{
    prelude::{Added, BuildChildren, Commands, Entity, Query, Transform},
    sprite::{SpriteSheetBundle, TextureAtlasSprite},
    utils::default,
};

use crate::plugins::tilemap::components::{
    Tilemap, TilemapMapSize, TilemapTextureHandle, TilemapTextureIndices, TilemapTile,
    TilemapTileColor, TilemapTileSize,
};

pub fn spawn_tiles(
    mut commands: Commands,
    tilemap_query: Query<
        (
            Entity,
            &TilemapTextureHandle,
            &TilemapMapSize,
            &TilemapTileSize,
            &TilemapTextureIndices,
            &TilemapTileColor,
        ),
        Added<Tilemap>,
    >,
) {
    for (
        tilemap_entity,
        tilemap_texture_handle,
        tilemap_size,
        tilemap_tile_size,
        tilemap_texture_indices,
        tilemap_tile_color,
    ) in tilemap_query.iter()
    {
        for y in 0..tilemap_size.0.y {
            for x in 0..tilemap_size.0.x {
                let texture_index = tilemap_texture_indices
                    .0
                    .get(y as usize)
                    .expect("No texture index found.")
                    .get(x as usize)
                    .expect("No texture index found.");

                let mut sprite = TextureAtlasSprite::new(*texture_index as usize);
                sprite.color = tilemap_tile_color.0;

                let tile_entity = commands
                    .spawn(SpriteSheetBundle {
                        sprite,
                        texture_atlas: tilemap_texture_handle.0.clone(),
                        transform: Transform::from_xyz(
                            x as f32 * tilemap_tile_size.0.x,
                            y as f32 * tilemap_tile_size.0.y,
                            3.0,
                        ),
                        ..default()
                    })
                    .insert(TilemapTile)
                    .id();

                commands.entity(tilemap_entity).add_child(tile_entity);
            }
        }
    }
}
