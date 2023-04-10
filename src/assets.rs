// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    // Images.
    #[asset(path = "images/player.png")]
    pub image_player: Handle<Image>,
    #[asset(path = "images/npc_piet.png")]
    pub image_npc_piet: Handle<Image>,
    #[asset(path = "images/npc_marcus.png")]
    pub image_npc_marcus: Handle<Image>,
    #[asset(path = "images/npc_andrea.png")]
    pub image_npc_andrea: Handle<Image>,
    #[asset(path = "images/npc_cat.png")]
    pub image_npc_cat: Handle<Image>,
    #[asset(path = "images/enemy.png")]
    pub image_enemy: Handle<Image>,
    #[asset(path = "images/tileset.png")]
    pub image_tileset: Handle<Image>,
    #[asset(path = "images/spell.png")]
    pub image_spell: Handle<Image>,
    #[asset(path = "images/spawner.png")]
    pub image_spawner: Handle<Image>,

    // Fonts.
    #[asset(path = "fonts/FiraMono-Medium.ttf")]
    pub font_medium: Handle<Font>,
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub font_bold: Handle<Font>,

    // Music.
    #[asset(path = "sounds/music.ogg")]
    pub music: Handle<AudioSource>,

    // Sounds.
    #[asset(path = "sounds/sound_player_hit.ogg")]
    pub sound_player_hit: Handle<AudioSource>,
    #[asset(path = "sounds/sound_player_death.ogg")]
    pub sound_player_death: Handle<AudioSource>,
    #[asset(path = "sounds/sound_enemy_hit.ogg")]
    pub sound_enemy_hit: Handle<AudioSource>,
    #[asset(path = "sounds/sound_enemy_death.ogg")]
    pub sound_enemy_death: Handle<AudioSource>,
    #[asset(path = "sounds/sound_spawner_hit.ogg")]
    pub sound_spawner_hit: Handle<AudioSource>,
    #[asset(path = "sounds/sound_spawner_destroy.ogg")]
    pub sound_spawner_destroy: Handle<AudioSource>,
    #[asset(path = "sounds/sound_melee_attack.ogg")]
    pub sound_melee_attack: Handle<AudioSource>,
    #[asset(path = "sounds/sound_magic_attack.ogg")]
    pub sound_magic_attack: Handle<AudioSource>,
    #[asset(path = "sounds/sound_start_dialog.ogg")]
    pub sound_start_dialog: Handle<AudioSource>,
    #[asset(path = "sounds/sound_eat_mushroom.ogg")]
    pub sound_eat_mushroom: Handle<AudioSource>,
    #[asset(path = "sounds/sound_game_win.ogg")]
    pub sound_game_win: Handle<AudioSource>,
    #[asset(path = "sounds/sound_game_over.ogg")]
    pub sound_game_over: Handle<AudioSource>,
}
