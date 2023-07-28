use std::time::Duration;

use bevy::{app::ScheduleRunnerSettings, log::LogPlugin, prelude::*};
use bevy_serialport::{
    DataBits, FlowControl, Parity, SerialPortPlugin, SerialPortRuntime, SerialPortSetting,
    SerialResource, StopBits,
};

use crate::{player::PlayersResource, states, ui::PlayerJoinEvent};

use bytes::Bytes;

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
            (enter_victory_button_leds_system,).in_schedule(OnEnter(states::AppStates::Victory)),
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

enum ButtonLEDByte {
    EndMarker = 255,
    Prefix = 254,
    Off = 0,
    On = 1,
    Fade = 2,
}

impl ButtonLEDByte {
    fn enter_main_menu() -> Bytes {
        vec![
            ButtonLEDByte::Off as u8,
            ButtonLEDByte::Off as u8,
            ButtonLEDByte::Off as u8,
            ButtonLEDByte::Off as u8,
            ButtonLEDByte::Fade as u8,
            ButtonLEDByte::EndMarker as u8,
        ]
        .into()
    }

    fn enter_character_selection() -> Bytes {
        vec![
            ButtonLEDByte::Fade as u8,
            ButtonLEDByte::Off as u8,
            ButtonLEDByte::Off as u8,
            ButtonLEDByte::Off as u8,
            ButtonLEDByte::Off as u8,
            ButtonLEDByte::EndMarker as u8,
        ]
        .into()
    }

    fn player_one_joined() -> Bytes {
        vec![
            ButtonLEDByte::Off as u8,
            ButtonLEDByte::Off as u8,
            ButtonLEDByte::Fade as u8,
            ButtonLEDByte::Off as u8,
            ButtonLEDByte::Fade as u8,
            ButtonLEDByte::EndMarker as u8,
        ]
        .into()
    }

    fn player_two_joined() -> Bytes {
        vec![
            ButtonLEDByte::Off as u8,
            ButtonLEDByte::Off as u8,
            ButtonLEDByte::Off as u8,
            ButtonLEDByte::Off as u8,
            ButtonLEDByte::Fade as u8,
            ButtonLEDByte::EndMarker as u8,
        ]
        .into()
    }

    fn enter_game() -> Bytes {
        vec![
            ButtonLEDByte::Off as u8,
            ButtonLEDByte::Off as u8,
            ButtonLEDByte::Off as u8,
            ButtonLEDByte::Off as u8,
            ButtonLEDByte::On as u8,
            ButtonLEDByte::EndMarker as u8,
        ]
        .into()
    }

    fn enter_pause() -> Bytes {
        vec![
            ButtonLEDByte::Off as u8,
            ButtonLEDByte::Fade as u8,
            ButtonLEDByte::Off as u8,
            ButtonLEDByte::Fade as u8,
            ButtonLEDByte::Fade as u8,
            ButtonLEDByte::EndMarker as u8,
        ]
        .into()
    }

    fn enter_gameover() -> Bytes {
        vec![
            ButtonLEDByte::Off as u8,
            ButtonLEDByte::Fade as u8,
            ButtonLEDByte::Off as u8,
            ButtonLEDByte::Fade as u8,
            ButtonLEDByte::Off as u8,
            ButtonLEDByte::EndMarker as u8,
        ]
        .into()
    }

    fn enter_victory() -> Bytes {
        vec![
            ButtonLEDByte::Off as u8,
            ButtonLEDByte::Fade as u8,
            ButtonLEDByte::Off as u8,
            ButtonLEDByte::Fade as u8,
            ButtonLEDByte::Off as u8,
            ButtonLEDByte::EndMarker as u8,
        ]
        .into()
    }
}

enum NeopixelStripByte {
    EndMarker = 255,
    RightPrefix = 252,
    LeftPrefix = 253,
    Off = 0,
}

fn enter_main_menu_button_leds_system(mut serial_resource: ResMut<SerialResource>) {
    serial_resource.send_message("COM3", ButtonLEDByte::enter_main_menu());
}

fn enter_character_selection_button_leds_system(mut serial_resource: ResMut<SerialResource>) {
    serial_resource.send_message("COM3", ButtonLEDByte::enter_character_selection());
}

fn character_selection_button_leds_system(
    mut serial_resource: ResMut<SerialResource>,
    mut player_join_event: EventReader<PlayerJoinEvent>,
) {
    for event in player_join_event.iter() {
        if event.0 == 0 {
            serial_resource.send_message("COM3", ButtonLEDByte::player_one_joined());
        } else if event.0 == 1 {
            serial_resource.send_message("COM3", ButtonLEDByte::player_two_joined());
        }
    }
}

fn enter_game_button_leds_system(mut serial_resource: ResMut<SerialResource>) {
    serial_resource.send_message("COM3", ButtonLEDByte::enter_game());
}

fn enter_pause_button_leds_system(mut serial_resource: ResMut<SerialResource>) {
    serial_resource.send_message("COM3", ButtonLEDByte::enter_pause());
}

fn enter_gameover_button_leds_system(mut serial_resource: ResMut<SerialResource>) {
    serial_resource.send_message("COM3", ButtonLEDByte::enter_gameover());
}

fn enter_victory_button_leds_system(mut serial_resource: ResMut<SerialResource>) {
    serial_resource.send_message("COM3", ButtonLEDByte::enter_victory());
}
