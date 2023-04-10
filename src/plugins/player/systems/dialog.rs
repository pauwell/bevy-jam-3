// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::prelude::{EventWriter, Input, KeyCode, Query, ResMut, Transform, With};

use crate::plugins::{
    dialog_box::{
        components::{DialogBoxContent, DialogBoxTitle},
        events::{DialogBoxCloseEvent, DialogBoxStartDialogEvent},
    },
    npc::components::{Npc, NpcDialog, NpcName, NpcProfession},
    player::{
        components::{Player, PlayerHealth, PlayerHealthMax},
        states::PlayerState,
    },
};

pub fn player_start_npc_dialog(
    mut player_query: Query<
        (
            &Transform,
            &mut PlayerState,
            &mut PlayerHealth,
            &PlayerHealthMax,
        ),
        With<Player>,
    >,
    npc_query: Query<(&Transform, &NpcName, &NpcDialog, &NpcProfession), With<Npc>>,
    mut dialog_box_start_dialog_event_writer: EventWriter<DialogBoxStartDialogEvent>,
    mut keyboard: ResMut<Input<KeyCode>>,
) {
    if player_query.is_empty() || npc_query.is_empty() || !keyboard.just_pressed(KeyCode::Space) {
        return;
    }

    let (player_transform, mut player_state, mut player_health, player_health_max) = player_query
        .get_single_mut()
        .expect("0 or more than 1 `Player` found.");

    if *player_state != PlayerState::Idle && *player_state != PlayerState::Walk {
        return;
    }

    for (npc_transform, npc_name, npc_dialog, npc_profession) in npc_query.iter() {
        if player_transform
            .translation
            .distance(npc_transform.translation)
            < 25.0
        {
            *player_state = PlayerState::Talk;
            dialog_box_start_dialog_event_writer.send(DialogBoxStartDialogEvent {
                title: DialogBoxTitle(npc_name.0.clone()),
                content: DialogBoxContent(npc_dialog.0.clone()),
            });
            if *npc_profession == NpcProfession::Healer {
                player_health.0 = player_health_max.0;
            }
            keyboard.clear_just_pressed(KeyCode::Space);
        }
    }
}

pub fn player_close_npc_dialog(
    mut query: Query<&mut PlayerState>,
    mut dialog_box_close_event_writer: EventWriter<DialogBoxCloseEvent>,
    mut keyboard: ResMut<Input<KeyCode>>,
) {
    if query.is_empty() || !keyboard.just_pressed(KeyCode::Space) {
        return;
    }

    let mut player_state = query
        .get_single_mut()
        .expect("0 or more than 1 `Player` found.");

    if *player_state == PlayerState::Talk {
        *player_state = PlayerState::Idle;
        dialog_box_close_event_writer.send(DialogBoxCloseEvent);
        keyboard.clear_just_pressed(KeyCode::Space);
    }
}
