use assets::GameAssets;
use bevy::{
    prelude::{App, EventWriter, ImagePlugin, IntoSystemAppConfig, Msaa, OnExit, PluginGroup},
    window::{MonitorSelection, Window, WindowPlugin, WindowPosition, WindowResolution},
    DefaultPlugins,
};
use bevy_asset_loader::prelude::{LoadingState, LoadingStateAppExt};
use bevy_egui::EguiPlugin;
use plugins::{
    atlas::AtlasPlugin,
    audio::{events::AudioStartPlayEvent, AudioClip},
    game::GamePlugin,
    game_over::GameOverPlugin,
    game_win::GameWinPlugin,
    intro::IntroPlugin,
};
use states::AppState;

mod assets;
pub mod helper;
mod plugins;
mod states;

fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .add_state::<AppState>()
        .add_loading_state(
            LoadingState::new(AppState::LoadingAssets).continue_to_state(AppState::BuildingAtlases),
        )
        .add_collection_to_loading_state::<_, GameAssets>(AppState::LoadingAssets)
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(1024.0, 768.0),
                        resizable: false,
                        decorations: false,
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_system(bevy::window::close_on_esc)
        .add_plugin(EguiPlugin)
        .add_plugin(IntroPlugin)
        .add_plugin(AtlasPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(GameOverPlugin)
        .add_plugin(GameWinPlugin)
        .add_system(start_main_music_loop.in_schedule(OnExit(AppState::BuildingAtlases)))
        .run();
}

pub fn start_main_music_loop(mut audio_start_play_event_writer: EventWriter<AudioStartPlayEvent>) {
    audio_start_play_event_writer.send(AudioStartPlayEvent(AudioClip::Music));
}
