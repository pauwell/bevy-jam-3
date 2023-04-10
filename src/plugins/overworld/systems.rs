// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::{
    prelude::{
        Added, Camera2d, Commands, DespawnRecursiveExt, Entity, EventWriter, Name, NextState,
        Query, Res, ResMut, Transform, Vec2, With, Without,
    },
    window::Window,
};

use crate::plugins::{
    atlas::resources::GameAtlases,
    dialog_box::{
        components::{DialogBox, DialogBoxContent, DialogBoxTitle},
        events::DialogBoxStartDialogEvent,
    },
    game::states::{GameState, LevelState},
    health_bar::components::HealthBar,
    npc::{
        bundles::NpcBundle,
        components::{Npc, NpcDialog, NpcName, NpcProfession},
    },
    player::{
        components::{Player, PlayerDirection},
        states::PlayerState,
    },
    tilemap::bundles::TilemapBundle,
    trigger::bundles::LevelTriggerBundle,
};

pub fn overworld_setup(
    mut commands: Commands,
    mut player_query: Query<(&mut Transform, &mut PlayerDirection), With<Player>>,
    game_atlases: Res<GameAtlases>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    game_state.set(GameState::Running);

    if let Ok((mut player_transform, mut player_direction)) = player_query.get_single_mut() {
        player_transform.translation.y = 285.0;
        *player_direction = PlayerDirection::Down;
    }

    commands
        .spawn(TilemapBundle::new(
            game_atlases.tileset.clone(),
            (16.0, 16.0).into(),
            (10, 20).into(),
            vec![
                vec![16, 15, 15, 16, 15, 16, 15, 15, 15, 16],
                vec![11, 10, 11, 12, 10, 10, 12, 11, 11, 10],
                vec![12, 15, 4, 9, 9, 4, 4, 4, 4, 10],
                vec![12, 10, 4, 4, 4, 14, 25, 4, 4, 11],
                vec![12, 4, 9, 9, 4, 4, 9, 4, 12, 12],
                vec![10, 4, 4, 4, 4, 4, 4, 4, 4, 11],
                vec![12, 9, 4, 4, 19, 4, 9, 26, 4, 12],
                vec![12, 4, 14, 4, 4, 4, 4, 27, 28, 11],
                vec![12, 4, 4, 4, 4, 4, 4, 4, 4, 10],
                vec![11, 25, 4, 14, 14, 4, 4, 9, 4, 12],
                vec![12, 4, 4, 9, 9, 4, 4, 4, 4, 10],
                vec![12, 4, 9, 4, 4, 4, 14, 19, 4, 11],
                vec![12, 4, 29, 4, 4, 4, 4, 4, 4, 12],
                vec![11, 9, 4, 22, 22, 4, 4, 4, 4, 10],
                vec![10, 12, 9, 20, 21, 4, 25, 4, 9, 10],
                vec![12, 4, 4, 4, 4, 25, 4, 9, 4, 11],
                vec![12, 4, 9, 4, 4, 4, 14, 4, 4, 11],
                vec![12, 4, 14, 4, 9, 4, 4, 9, 4, 12],
                vec![10, 15, 16, 15, 9, 9, 15, 15, 16, 10],
                vec![12, 10, 11, 12, 23, 24, 11, 11, 12, 10],
            ],
            vec![0, 1, 2, 3, 5, 6, 7, 8, 10, 11, 12, 20, 21, 27, 28],
            None,
        ))
        .insert(Name::from("OverworldTilemap"));

    commands
        .spawn(NpcBundle::new(
            game_atlases.npc_piet.clone(),
            "Piet",
            "",
            NpcProfession::None,
            Vec2 { x: 16.0, y: 10.0 },
            Vec2 { x: 50.0, y: 185.0 },
            14.0,
        ))
        .insert(Name::from("Npc [Piet]"));

    commands
        .spawn(NpcBundle::new(
            game_atlases.npc_marcus.clone(),
            "Marcus",
            "",
            NpcProfession::Healer,
            Vec2 { x: 16.0, y: 10.0 },
            Vec2 { x: 125.0, y: 40.0 },
            14.0,
        ))
        .insert(Name::from("Npc [Marcus]"));

    commands
        .spawn(NpcBundle::new(
            game_atlases.npc_andrea.clone(),
            "Andrea",
            "",
            NpcProfession::None,
            Vec2 { x: 16.0, y: 10.0 },
            Vec2 { x: 50.0, y: 100.0 },
            14.0,
        ))
        .insert(Name::from("Npc [Andrea]"));

    commands
        .spawn(NpcBundle::new(
            game_atlases.npc_cat.clone(),
            "Cat",
            "",
            NpcProfession::None,
            Vec2 { x: 16.0, y: 10.0 },
            Vec2 { x: 20.0, y: 275.0 },
            14.0,
        ))
        .insert(Name::from("Npc [Cat]"));

    commands
        .spawn(LevelTriggerBundle::new(
            Vec2::new(72.0, 300.0),
            LevelState::Dungeon,
        ))
        .insert(Name::from("LevelTrigger"));
}

pub fn overworld_changes_on_first_visit(
    mut npc_query: Query<(&NpcName, &mut NpcDialog), Added<Npc>>,
    mut player_query: Query<(&mut Transform, &mut PlayerState), Added<Player>>,
    mut dialog_box_start_dialog_event_writer: EventWriter<DialogBoxStartDialogEvent>,
) {
    if let Ok((mut player_transform, mut player_state)) = player_query.get_single_mut() {
        player_transform.translation.x = 40.0;
        player_transform.translation.y = 75.0;

        *player_state = PlayerState::Talk;
        dialog_box_start_dialog_event_writer.send(DialogBoxStartDialogEvent {
            title: DialogBoxTitle("Welcome!".into()),
            content: DialogBoxContent(
                "Controls:
                \n[Arrow Keys] - Walk\n[Space] - Talk\n[D] - Magic\n[S] - Melee\n[P] - Pause/ Controls\n[Escape] - Quit\n
                Press [Space] To Continue.".into(),
            ),
        })
    }

    for (npc_name, mut npc_dialog) in npc_query.iter_mut() {
        npc_dialog.0 = match npc_name.0.as_str() {
            "Piet" => "Welcome, brave one.\nWould you be willing to rid our forest of those slimy beings?",
            "Marcus" => "Be wary of the slimy creature nests in the north forest. If you return to me, I can heal any wounds you may have.",
            "Andrea" => "Watch your step in the forest, friend!\nShould you need healing, feel free to return and seek Marcus' aid.",
            "Cat" => "Meow! [Mice to meet you, let's get this quest started!]",
            _ => "",
        }.into();
    }
}

pub fn overworld_changes_on_middle_game(
    mut npc_query: Query<(&NpcName, &mut NpcDialog), Added<Npc>>,
) {
    for (npc_name, mut npc_dialog) in npc_query.iter_mut() {
        npc_dialog.0 = match npc_name.0.as_str() {
            "Piet" => {
                "Hey there! Just a heads up, if you defeat slimes, they sometimes drop mushrooms. These mushrooms are actually quite useful for destroying slime nests. However, be cautious as they can have some peculiar side effects."
            }
            "Marcus" => "Come back to me if you need my healing powers!",
            "Andrea" => "Please help us defeat those slimes so we can go back home!",
            "Cat" => "Meow, meow! [Paw-some to be fighting alongside you!]",
            _ => "",
        }
        .into();
    }
}
pub fn overworld_changes_on_win(mut npc_query: Query<(&NpcName, &mut NpcDialog), Added<Npc>>) {
    for (npc_name, mut npc_dialog) in npc_query.iter_mut() {
        npc_dialog.0 = match npc_name.0.as_str() {
            "Piet" => "You did it! Thank you so much.",
            "Marcus" => "Now we can go back home. You are a hero!",
            "Andrea" => "I am so glad it's finally over. Thanks to you!",
            "Cat" => "MEEOWW!",
            _ => "",
        }
        .into();
    }
}

pub fn overworld_clear(
    mut commands: Commands,
    query: Query<
        Entity,
        (
            Without<Window>,
            Without<Camera2d>,
            Without<Player>,
            Without<DialogBox>,
            Without<HealthBar>,
        ),
    >,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
