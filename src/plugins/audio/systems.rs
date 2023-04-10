// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::{
    prelude::{Audio, AudioSource, EventReader, Handle, PlaybackSettings, Res},
    utils::default,
};

use crate::assets::GameAssets;

use super::{events::AudioStartPlayEvent, AudioClip};

pub fn handle_play_audio_event(
    game_assets: Res<GameAssets>,
    audio: Res<Audio>,
    mut start_play_audio_event_reader: EventReader<AudioStartPlayEvent>,
) {
    for play_audio_event in start_play_audio_event_reader.iter() {
        let audio_source: Handle<AudioSource>;
        let mut repeat = false;

        match play_audio_event.0 {
            AudioClip::Music => {
                audio_source = game_assets.music.clone();
                repeat = true;
            }
            AudioClip::EatMushroom => {
                audio_source = game_assets.sound_eat_mushroom.clone();
            }
            AudioClip::HitEnemy => {
                audio_source = game_assets.sound_enemy_hit.clone();
            }
            AudioClip::MagicAttack => {
                audio_source = game_assets.sound_magic_attack.clone();
            }
            AudioClip::MeleeAttack => {
                audio_source = game_assets.sound_melee_attack.clone();
            }
            AudioClip::StartDialog => {
                audio_source = game_assets.sound_start_dialog.clone();
            }
            AudioClip::DestroySpawner => {
                audio_source = game_assets.sound_spawner_destroy.clone();
            }
            AudioClip::HitSpawner => {
                audio_source = game_assets.sound_spawner_hit.clone();
            }
            AudioClip::DeathPlayer => {
                audio_source = game_assets.sound_player_death.clone();
            }
            AudioClip::HitPlayer => {
                audio_source = game_assets.sound_player_hit.clone();
            }
            AudioClip::DeathEnemy => {
                audio_source = game_assets.sound_enemy_death.clone();
            }
            AudioClip::WinGame => {
                audio_source = game_assets.sound_game_win.clone();
            }
            AudioClip::LoseGame => {
                audio_source = game_assets.sound_game_over.clone();
            }
        }

        audio.play_with_settings(
            audio_source,
            PlaybackSettings {
                repeat,
                ..default()
            },
        );
    }
}
