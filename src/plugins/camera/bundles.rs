// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::{Bundle, Camera, Camera2d, Camera2dBundle, Color, OrthographicProjection},
    utils::default,
};

use super::components::{GameCamera, GameCameraState};

#[derive(Bundle)]
pub struct GameCameraBundle {
    tag: GameCamera,
    state: GameCameraState,
    #[bundle]
    camera2d: Camera2dBundle,
}

impl GameCameraBundle {
    pub fn new(projection_scale: f32, camera_state: GameCameraState) -> Self {
        let mut projection = OrthographicProjection::default();
        projection.scale = projection_scale;

        GameCameraBundle {
            tag: GameCamera,
            state: camera_state,
            camera2d: Camera2dBundle {
                camera: Camera {
                    hdr: true,
                    ..default()
                },
                projection,
                camera_2d: Camera2d {
                    clear_color: ClearColorConfig::Custom(Color::rgba_u8(3, 18, 6, 255)),
                },
                ..default()
            },
        }
    }
}
