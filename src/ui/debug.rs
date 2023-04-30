use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::run::RunResource;

/// Shows information about the run
pub fn game_debug_ui(mut egui_contexts: EguiContexts, run_resource: Res<RunResource>) {
    egui::Window::new("Run Tracker")
        .default_pos([550.0, 16.0])
        .show(egui_contexts.ctx_mut(), |ui| {
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
                    ui.label(format!("Phase[{}]", level.get_phase_number()));
                    ui.monospace(format!(
                        "{} {}/{}",
                        level.get_phase_name(),
                        if let Some(phase_timer) = &level.phase_timer {
                            phase_timer.elapsed_secs() as usize
                        } else {
                            0
                        },
                        if let Some(phase_timer) = &level.phase_timer {
                            phase_timer.duration().as_secs() as usize
                        } else {
                            0
                        },
                    ));
                });
            }
        });
}
