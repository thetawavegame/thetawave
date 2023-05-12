use std::time::Duration;

use bevy::{app::ScheduleRunnerSettings, log::LogPlugin, prelude::*};
use bevy_serialport::{
    DataBits, FlowControl, Parity, SerialPortPlugin, SerialPortRuntime, SerialPortSetting,
    SerialResource, StopBits,
};

use crate::{player::PlayersResource, states, ui::PlayerJoinEvent};

pub struct ArcadePlugin;

impl Plugin for ArcadePlugin {
    fn build(&self, app: &mut App) {
        //app.insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_millis(10)));

        app.add_plugin(SerialPortPlugin)
            .add_startup_system(setup_serial_system)
            .add_startup_system(enter_main_menu_button_leds_system);

        app.add_systems(
            (enter_main_menu_button_leds_system,).in_schedule(OnEnter(states::AppStates::MainMenu)),
        );

        app.add_systems(
            (enter_character_selection_button_leds_system,)
                .in_schedule(OnEnter(states::AppStates::CharacterSelection)),
        );

        app.add_systems(
            (character_selection_button_leds_system,)
                .in_set(OnUpdate(states::AppStates::CharacterSelection)),
        );

        app.add_systems(
            (enter_game_button_leds_system,).in_schedule(OnEnter(states::AppStates::Game)),
        );

        app.add_systems(
            (enter_pause_button_leds_system,).in_schedule(OnEnter(states::GameStates::Paused)),
        );

        app.add_systems(
            (enter_game_button_leds_system,).in_schedule(OnExit(states::GameStates::Paused)),
        );

        app.add_systems(
            (enter_gameover_button_leds_system,).in_schedule(OnEnter(states::AppStates::Victory)),
        );

        app.add_systems(
            (enter_gameover_button_leds_system,).in_schedule(OnEnter(states::AppStates::GameOver)),
        );
    }
}

fn setup_serial_system(
    mut serial_resource: ResMut<SerialResource>,
    runtime: Res<SerialPortRuntime>,
) {
    let serial_setting = SerialPortSetting {
        port_name: "COM3".to_string(),
        baud_rate: 115200,
        data_bits: DataBits::Eight,
        flow_control: FlowControl::None,
        parity: Parity::None,
        stop_bits: StopBits::One,
        timeout: Default::default(),
    };

    serial_resource
        .open_with_setting(runtime.clone(), serial_setting)
        .expect("Error opening serial port");
}

fn send_test_data_system(mut serial_res: ResMut<SerialResource>) {
    serial_res.send_message("COM3", vec![1, 1, 1, 1, 2, 255].into());
}

fn enter_main_menu_button_leds_system(mut serial_resource: ResMut<SerialResource>) {
    serial_resource.send_message("COM3", vec![0, 0, 0, 0, 2, 255].into());
}

fn enter_character_selection_button_leds_system(mut serial_resource: ResMut<SerialResource>) {
    serial_resource.send_message("COM3", vec![2, 0, 0, 0, 0, 255].into());
}

fn character_selection_button_leds_system(
    mut serial_resource: ResMut<SerialResource>,
    mut player_join_event: EventReader<PlayerJoinEvent>,
) {
    for event in player_join_event.iter() {
        if event.0 == 0 {
            serial_resource.send_message("COM3", vec![0, 0, 2, 0, 2, 255].into());
        } else if event.0 == 1 {
            serial_resource.send_message("COM3", vec![0, 0, 0, 0, 2, 255].into());
        }
    }
}

fn enter_game_button_leds_system(mut serial_resource: ResMut<SerialResource>) {
    serial_resource.send_message("COM3", vec![0, 0, 0, 0, 1, 255].into());
}

fn enter_pause_button_leds_system(mut serial_resource: ResMut<SerialResource>) {
    serial_resource.send_message("COM3", vec![0, 2, 0, 2, 2, 255].into());
}

fn enter_gameover_button_leds_system(mut serial_resource: ResMut<SerialResource>) {
    serial_resource.send_message("COM3", vec![0, 2, 0, 2, 0, 255].into());
}
