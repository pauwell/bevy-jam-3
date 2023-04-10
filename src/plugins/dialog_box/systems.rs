// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::prelude::{
    Commands, Entity, EventReader, EventWriter, Input, KeyCode, Name, Query, Res, With, Without,
};
use bevy_egui::{egui, EguiContexts};

use crate::plugins::audio::{events::AudioStartPlayEvent, AudioClip};

use super::{
    components::{DialogBox, DialogBoxContent, DialogBoxIsActive, DialogBoxTitle},
    events::{DialogBoxCloseEvent, DialogBoxStartDialogEvent},
};

pub fn setup(mut commands: Commands) {
    commands
        .spawn_empty()
        .insert(Name::new("Dialog Box"))
        .insert(DialogBox)
        .insert(DialogBoxTitle::default())
        .insert(DialogBoxContent::default());
}

pub fn display_dialog_box(
    query: Query<(&DialogBoxTitle, &DialogBoxContent), With<DialogBoxIsActive>>,
    mut contexts: EguiContexts,
) {
    if query.is_empty() {
        return;
    }

    let (dialog_box_title, dialog_box_content) = query
        .get_single()
        .expect("0 or more than 1 dialog-box found.");

    let mut dialog_box_style: egui::Style = (*contexts.ctx_mut().style()).clone();
    dialog_box_style.spacing.item_spacing = egui::vec2(10.0, 20.0);
    dialog_box_style.visuals.override_text_color = Some(egui::Color32::LIGHT_GREEN);
    dialog_box_style.visuals.resize_corner_size = 0.0;
    contexts.ctx_mut().set_style(dialog_box_style);
    egui::Window::new("DialogBox")
        .collapsible(false)
        .title_bar(false)
        .fixed_pos((1024.0 / 2.0 - 150.0, 768.0 / 1.8))
        .show(contexts.ctx_mut(), |ui| {
            ui.set_width(450.0);
            ui.set_height(100.0);
            ui.heading(dialog_box_title.0.clone());
            ui.separator();
            ui.label(
                egui::RichText::new(dialog_box_content.0.clone())
                    .font(egui::FontId::proportional(17.0)),
            )
        });
}

pub fn close_dialog_box(
    mut query: Query<(Entity, &mut DialogBoxTitle, &mut DialogBoxContent), With<DialogBoxIsActive>>,
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
) {
    if query.is_empty() {
        return;
    }

    if keyboard.just_pressed(KeyCode::Space) {
        let (dialog_box_entity, mut dialog_box_title, mut dialog_box_content) = query
            .get_single_mut()
            .expect("0 or more than 1 dialog-box found.");

        dialog_box_title.0 = String::new();
        dialog_box_content.0 = String::new();

        commands
            .entity(dialog_box_entity)
            .remove::<DialogBoxIsActive>();
    }
}

pub fn handle_dialog_box_start_event(
    mut commands: Commands,
    mut query: Query<
        (Entity, &mut DialogBoxTitle, &mut DialogBoxContent),
        Without<DialogBoxIsActive>,
    >,
    mut start_dialog_event_reader: EventReader<DialogBoxStartDialogEvent>,
    mut audio_start_play_event_writer: EventWriter<AudioStartPlayEvent>,
) {
    for start_dialog_event in start_dialog_event_reader.iter() {
        if query.is_empty() {
            return;
        }

        let (dialog_box_entity, mut dialog_box_title, mut dialog_box_content) = query
            .get_single_mut()
            .expect("0 or more than 1 dialog-box found.");

        dialog_box_title.0 = start_dialog_event.title.0.clone();
        dialog_box_content.0 = start_dialog_event.content.0.clone();

        commands.entity(dialog_box_entity).insert(DialogBoxIsActive);
        audio_start_play_event_writer.send(AudioStartPlayEvent(AudioClip::StartDialog));
    }
}

pub fn handle_dialog_box_close_event(
    mut commands: Commands,
    mut query: Query<(Entity, &mut DialogBoxTitle, &mut DialogBoxContent), With<DialogBoxIsActive>>,
    mut close_dialog_event_reader: EventReader<DialogBoxCloseEvent>,
) {
    for _ in close_dialog_event_reader.iter() {
        if query.is_empty() {
            return;
        }

        let (dialog_box_entity, mut dialog_box_title, mut dialog_box_content) = query
            .get_single_mut()
            .expect("0 or more than 1 dialog-box found.");

        dialog_box_title.0.clear();
        dialog_box_content.0.clear();

        commands
            .entity(dialog_box_entity)
            .remove::<DialogBoxIsActive>();
    }
}
