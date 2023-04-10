// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy_egui::{egui, EguiContexts};

pub fn display_pause_text(mut contexts: EguiContexts) {
    let mut dialog_box_style: egui::Style = (*contexts.ctx_mut().style()).clone();
    dialog_box_style.spacing.item_spacing = egui::vec2(10.0, 20.0);
    dialog_box_style.visuals.override_text_color = Some(egui::Color32::LIGHT_GREEN);
    dialog_box_style.visuals.resize_corner_size = 0.0;
    contexts.ctx_mut().set_style(dialog_box_style);
    egui::Window::new("PauseText")
        .collapsible(false)
        .title_bar(false)
        .fixed_pos((1024.0 / 2.0 - 150.0, 768.0 / 1.8))
        .show(contexts.ctx_mut(), |ui| {
            ui.set_width(450.0);
            ui.set_height(100.0);
            ui.heading("Paused");
            ui.separator();
            ui.label(
                egui::RichText::new("Controls:
            \n[Arrow Keys] - Walk\n[Space] - Talk\n[D] - Magic\n[S] - Melee\n[P] - Pause/ Controls\n[Escape] - Quit\n
            Press [P] to resume").font(egui::FontId::proportional(17.0)),
            );
        });
}
