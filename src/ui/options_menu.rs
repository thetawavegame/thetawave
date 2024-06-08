use crate::audio::BackgroundMusicAudioChannel;
use crate::options::apply_game_options_system;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::prelude::{
    in_state, App, Camera, Camera2d, Camera3d, EventWriter, IntoSystemConfigs, NextState, OnEnter,
    Plugin, Query, Res, ResMut, Resource, Update, With, Without,
};
use bevy::utils::tracing::{error, info};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_kira_audio::{AudioChannel, AudioControl};
use leafwing_input_manager::prelude::ActionState;
use thetawave_interface::audio::{PlaySoundEffectEvent, SoundEffectType};
use thetawave_interface::game::options::GameOptions;
use thetawave_interface::input::{MenuAction, MenuExplorer};
use thetawave_interface::states::OptionsMenuOverlay;

/// Exposes an options menu to modify CurrentGameOptions via a maximally simple menu
pub(crate) struct OptionsMenuPlugin;

const MAX_BLOOM_FACTOR: f32 = 2.;
impl Plugin for OptionsMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .init_state::<OptionsMenuOverlay>()
            .add_systems(
                Update,
                (paint_options_menu, exit_options_menu_on_input_action)
                    .run_if(in_state(OptionsMenuOverlay::Enabled)),
            )
            .add_systems(
                OnEnter(OptionsMenuOverlay::Enabled),
                initialize_options_ui_state_from_existing_game_params,
            )
            .insert_resource(OptionsUIState::default());
    }
}

/// The internal representation of the options "form"/menu. Another system needs to sync this state
/// with the actual game params/state. All of the u8s are from 0 to 100 (inclusive) to make things
/// simple for a UI. Conversions to floats are done as close as possible to the call site.
#[derive(Resource, Debug)]
struct OptionsUIState {
    /// 50 == "unaltered"/base. 100 = louder. 0 = silent.
    pub master_volume_level: u8,
    pub background_volume_level: u8,
    pub bloom_intensity: u8,
}

impl Default for OptionsUIState {
    fn default() -> Self {
        Self {
            master_volume_level: 50,
            background_volume_level: 50,
            bloom_intensity: 100,
        }
    }
}
fn initialize_options_ui_state_from_existing_game_params(
    mut options_ui_state: ResMut<OptionsUIState>,
    game_options: Res<GameOptions>,
) {
    // TODO: I don't think this is correct. Why doesnt the audio channel keep this info?
    *options_ui_state = OptionsUIState {
        master_volume_level: 50,
        background_volume_level: 70,
        bloom_intensity: (game_options.bloom_intensity * 100.0) as u8,
    };
}

/// Display a form that updates the OptionsUiState when a button is clicked
fn paint_options_menu(
    mut eg_ctxs: EguiContexts,
    mut options_ui_state: ResMut<OptionsUIState>,
    bg_audio: Res<AudioChannel<BackgroundMusicAudioChannel>>,
    game_options: ResMut<GameOptions>,
    mut options_menu_state: ResMut<NextState<OptionsMenuOverlay>>,
    mut sound_effects_event_writer: EventWriter<PlaySoundEffectEvent>,
    camera_2d_query: Query<(&mut Camera, &mut Tonemapping), (With<Camera2d>, Without<Camera3d>)>,
    camera_3d_query: Query<(&mut Camera, &mut Tonemapping), (With<Camera3d>, Without<Camera2d>)>,
) {
    let ctx = eg_ctxs.ctx_mut();
    let mut apply_settings_button_clicked = false;
    let mut exit_button_clicked = false;
    // layout+style
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.with_layout(
            egui::Layout::top_down_justified(egui::Align::Center)
                .with_cross_align(egui::Align::Center),
            |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Options");
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.label("Master Volume Level: ");
                        ui.add(
                            egui::widgets::Slider::new(
                                &mut options_ui_state.master_volume_level,
                                0..=100,
                            )
                            .show_value(false)
                            .step_by(5.),
                        );
                    });
                    ui.horizontal(|ui| {
                        ui.label("Background Music Volume: ");
                        ui.add(
                            egui::widgets::Slider::new(
                                &mut options_ui_state.background_volume_level,
                                0..=100,
                            )
                            .show_value(false)
                            .step_by(5.),
                        );
                    });
                    ui.horizontal(|ui| {
                        ui.label("Bloom Intensity: ");
                        ui.add(
                            egui::widgets::Slider::new(
                                &mut options_ui_state.bloom_intensity,
                                0..=100,
                            )
                            .show_value(false)
                            .step_by(5.),
                        );
                    });
                    ui.separator();
                    ui.horizontal(|ui| {
                        apply_settings_button_clicked = ui.button("Apply settings").clicked();
                    });
                    ui.horizontal(|ui| {
                        exit_button_clicked = ui.button("Exit").clicked();
                    });
                });
            },
        );
    });
    // behavior+interactivity
    if apply_settings_button_clicked {
        apply_options_settings(
            &options_ui_state,
            bg_audio,
            game_options,
            camera_2d_query,
            camera_3d_query,
        );
        sound_effects_event_writer.send(PlaySoundEffectEvent {
            sound_effect_type: SoundEffectType::MenuInputSuccess,
        });
    }
    if exit_button_clicked {
        options_menu_state.set(OptionsMenuOverlay::Disabled);
    }
}
/// Apply the settings in the OptionsUIState by changing game settings.
fn apply_options_settings(
    ui_state: &OptionsUIState,
    bg_audio: Res<AudioChannel<BackgroundMusicAudioChannel>>,
    mut game_options: ResMut<GameOptions>,
    camera_2d_query: Query<(&mut Camera, &mut Tonemapping), (With<Camera2d>, Without<Camera3d>)>,
    camera_3d_query: Query<(&mut Camera, &mut Tonemapping), (With<Camera3d>, Without<Camera2d>)>,
) {
    info!("Applying options: {:?}", ui_state);
    bg_audio.set_volume(
        percieved_volume_level(
            ui_state.master_volume_level,
            ui_state.background_volume_level,
        ) as f64
            / 100.,
    );
    game_options.bloom_intensity = ui_state.bloom_intensity as f32 / 100. * MAX_BLOOM_FACTOR;
    game_options.bloom_enabled = ui_state.bloom_intensity > 0;
    apply_game_options_system(game_options.into(), camera_2d_query, camera_3d_query);
}

fn exit_options_menu_on_input_action(
    mut next_options_menu_overlay_state: ResMut<NextState<OptionsMenuOverlay>>,
    menu_input_query: Query<&ActionState<MenuAction>, With<MenuExplorer>>,
) {
    if let Ok(menu_action) = menu_input_query.get_single() {
        if menu_action.just_released(&MenuAction::Back) {
            info!("Exiting options menu overlay state");
            next_options_menu_overlay_state.set(OptionsMenuOverlay::Disabled)
        }
    }
}

/// Returns the 0-100-based "percentage of the channel_volume_level", with the ratio given
/// by the master_volume_level percentage. All values are between 0 and 100, inclusive.
fn percieved_volume_level(master_volume_level: u8, channel_volume_level: u8) -> u8 {
    match master_volume_level {
        0 => 0,
        master_volume_level => {
            (channel_volume_level as f32 * (master_volume_level as f32 / 100.0 * 2.)) as u8
        }
    }
}
