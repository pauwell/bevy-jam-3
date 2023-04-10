// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::{
    prelude::{Added, Query, Rect, Transform, With},
    sprite::TextureAtlasSprite,
};

use crate::plugins::tilemap::components::{
    Tilemap, TilemapColliders, TilemapSolidTextureIndices, TilemapTile, TilemapTileSize,
};

pub fn spawn_colliders(
    mut tilemap_query: Query<
        (
            &mut TilemapColliders,
            &TilemapSolidTextureIndices,
            &TilemapTileSize,
        ),
        With<Tilemap>,
    >,
    added_tiles_query: Query<(&Transform, &TextureAtlasSprite), Added<TilemapTile>>,
) {
    for (tile_transform, tile_sprite) in added_tiles_query.iter() {
        let (mut tilemap_colliders, tilemap_solid_indices, tilemap_tile_size) = tilemap_query
            .get_single_mut()
            .expect("0 or more than 1 tilemap found.");

        let tile_texture_index = tile_sprite.index as u32;
        if tilemap_solid_indices
            .0
            .iter()
            .find(|&&e| e == tile_texture_index)
            .is_some()
        {
            let tile_center_in_world = tile_transform.translation.truncate();

            tilemap_colliders.0.push(Rect::from_center_size(
                tile_center_in_world,
                tilemap_tile_size.0,
            ));

            // Draw the colliders in debug-mode.
            /* #[cfg(debug_assertions)]
            commands
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::RED.with_a(100.0),
                        custom_size: Some(Vec2::from(tilemap_tile_size.0)),
                        ..default()
                    },
                    transform: Transform::from_translation(tile_center_in_world.extend(50.0)),
                    ..default()
                })
                .insert(Name::from("Collider")); */
        }
    }
}
