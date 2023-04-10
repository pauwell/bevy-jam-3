// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use bevy::prelude::{resource_exists, App, IntoSystemConfig, Plugin};

use crate::assets::GameAssets;

use self::{events::AudioStartPlayEvent, systems::handle_play_audio_event};

pub mod events;
pub mod systems;

pub enum AudioClip {
    Music,
    EatMushroom,
    HitEnemy,
    MagicAttack,
    MeleeAttack,
    StartDialog,
    DestroySpawner,
    HitSpawner,
    DeathPlayer,
    HitPlayer,
    DeathEnemy,
    WinGame,
    LoseGame,
}

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AudioStartPlayEvent>()
            .add_system(handle_play_audio_event.run_if(resource_exists::<GameAssets>()));
    }
}
