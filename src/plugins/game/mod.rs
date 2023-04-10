// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use crate::states::AppState;
use bevy::prelude::{
    in_state, App, IntoSystemAppConfig, IntoSystemConfig, OnEnter, OnUpdate, Plugin,
};

use self::{
    resources::GameProgress,
    states::{GameState, LevelState, StoryState},
    systems::{game_cleanup_on_enter, game_pause, game_setup, game_unpause},
};

use super::{
    animation::AnimationPlugin, audio::AudioPlugin, camera::CameraPlugin, debug::DebugPlugin,
    dialog_box::DialogBoxPlugin, dungeon::DungeonPlugin, enemy::EnemyPlugin,
    health_bar::HealthBarPlugin, mushroom::MushroomPlugin, npc::NpcPlugin,
    overworld::OverworldPlugin, pause::PausePlugin, player::PlayerPlugin, spawner::SpawnerPlugin,
    spell::SpellPlugin, tilemap::TilemapPlugin, trigger::TriggerPlugin,
};

pub mod resources;
pub mod states;
mod systems;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_state::<LevelState>()
            .add_state::<StoryState>()
            .insert_resource(GameProgress::default())
            .add_plugin(AnimationPlugin)
            .add_plugin(AudioPlugin)
            .add_plugin(CameraPlugin)
            .add_plugin(DebugPlugin)
            .add_plugin(DialogBoxPlugin)
            .add_plugin(DungeonPlugin)
            .add_plugin(EnemyPlugin)
            .add_plugin(HealthBarPlugin)
            .add_plugin(NpcPlugin)
            .add_plugin(OverworldPlugin)
            .add_plugin(PausePlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(MushroomPlugin)
            .add_plugin(SpawnerPlugin)
            .add_plugin(SpellPlugin)
            .add_plugin(TilemapPlugin)
            .add_plugin(TriggerPlugin)
            .add_system(
                game_cleanup_on_enter
                    .before(game_setup)
                    .in_schedule(OnEnter(AppState::InGame)),
            )
            .add_system(game_setup.in_schedule(OnEnter(AppState::InGame)))
            .add_system(
                game_pause
                    .in_set(OnUpdate(AppState::InGame))
                    .run_if(in_state(GameState::Running)),
            )
            .add_system(
                game_unpause
                    .in_set(OnUpdate(AppState::InGame))
                    .run_if(in_state(GameState::Paused)),
            );
    }
}
