use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::run::RunResource;

pub fn game_debug_ui(mut egui_context: ResMut<EguiContext>, run_resource: Res<RunResource>) {
    egui::Window::new("Run Tracker")
        .default_pos([550.0, 16.0])
        .show(egui_context.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                ui.label("Level");
                ui.monospace(format!("{:?}", run_resource.level_type));
            });

            if let Some(level) = &run_resource.level {
                ui.horizontal(|ui| {
                    ui.label("Objective");
                    ui.monospace(format!("{:?}", level.objective));
                });
                ui.horizontal(|ui| {
                    ui.label("Phase");
                    ui.monospace(format!(
                        "{}[{}] {}/{}",
                        level.get_phase_name(),
                        level.get_phase_number(),
                        level.phase_timer.as_ref().unwrap().elapsed_secs() as usize,
                        level.phase_timer.as_ref().unwrap().duration().as_secs() as usize,
                    ));
                });
            }
        });
}
