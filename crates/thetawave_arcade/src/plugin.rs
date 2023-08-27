use bevy::prelude::*;
use bevy_serialport::{
    DataBits, FlowControl, Parity, SerialPortPlugin, SerialPortRuntime, SerialPortSetting,
    SerialResource, StopBits,
};

use derive_more::{Deref, DerefMut, From};
use serialport::{available_ports, SerialPortType};
use thetawave_interface::states;

/// Environment variable name of the serial port that handles lights.
pub const THETAWAVE_ARCADE_LIGHT_SERIAL_PORT_NAME: &'static str =
    "THETAWAVE_ARCADE_LIGHT_SERIAL_PORT_NAME";

/// The port for the Arduino that controls the lights.
#[derive(Resource, Deref, DerefMut, From, Debug)]
pub struct ArduinoSerialPort(String);
impl ArduinoSerialPort {
    fn first_port_matching_manufacturer_product() -> Option<Self> {
        first_usb_port_name_matching_str("arduino").map(Self)
    }
    fn from_envvar() -> Option<Self> {
        std::env::var(THETAWAVE_ARCADE_LIGHT_SERIAL_PORT_NAME)
            .ok()
            .map(Self)
    }
}

fn first_usb_port_name_matching_str(lowercase_pattern: &str) -> Option<String> {
    match available_ports() {
        Ok(ports) => ports
            .into_iter()
            .find(|x| match &x.port_type {
                SerialPortType::UsbPort(x) => format!(
                    "{} {}",
                    x.manufacturer.clone().unwrap_or_default().to_lowercase(),
                    x.product.clone().unwrap_or_default().to_lowercase()
                )
                .contains(lowercase_pattern),
                _ => false,
            })
            .map(|x| x.port_name.clone()),
        Err(e) => {
            error!("Failed to list serial ports. {}", e);
            None
        }
    }
}
use bytes::Bytes;
use thetawave_interface::character_selection::PlayerJoinEvent;

/// The entrypoint for accepting game input from an arcade machine.
pub struct ArcadePlugin;

impl Plugin for ArcadePlugin {
    fn build(&self, app: &mut App) {
        if let Some(res) = ArduinoSerialPort::first_port_matching_manufacturer_product()
            .or_else(ArduinoSerialPort::from_envvar)
        {
            info!("arduino serial port: {:?}", &res);
            app.add_plugins(SerialPortPlugin).insert_resource(res);

            app.add_systems(
                Startup,
                (setup_serial_system, enter_main_menu_button_leds_system),
            );

            app.add_systems(
                OnEnter(states::AppStates::MainMenu),
                enter_main_menu_button_leds_system,
            );

            app.add_systems(
                OnEnter(states::AppStates::CharacterSelection),
                enter_character_selection_button_leds_system,
            );

            app.add_systems(
                Update,
                character_selection_button_leds_system
                    .run_if(in_state(states::AppStates::CharacterSelection)),
            );

            app.add_systems(
                OnEnter(thetawave_interface::states::AppStates::Game),
                enter_game_button_leds_system,
            );

            app.add_systems(
                OnEnter(states::GameStates::Paused),
                enter_pause_button_leds_system,
            );

            app.add_systems(
                OnExit(states::GameStates::Paused),
                enter_game_button_leds_system,
            );

            app.add_systems(
                OnEnter(states::AppStates::Victory),
                enter_victory_button_leds_system,
            );

            app.add_systems(
                OnEnter(states::AppStates::GameOver),
                enter_gameover_button_leds_system,
            );
        } else {
            error!("Failed to find the arduino port for arcade lighting. no-op for the plugin. Enter the environment variable {} or compile without the --arcade feature", THETAWAVE_ARCADE_LIGHT_SERIAL_PORT_NAME);
        }
    }
}
fn setup_serial_system(
    mut serial_resource: ResMut<SerialResource>,
    arduino_port: Res<ArduinoSerialPort>,
    runtime: Res<SerialPortRuntime>,
) {
    let serial_setting = SerialPortSetting {
        port_name: arduino_port.clone(),
        baud_rate: 115200,
        data_bits: DataBits::Eight,
        flow_control: FlowControl::None,
        parity: Parity::None,
        stop_bits: StopBits::One,
        timeout: Default::default(),
    };
    serial_resource
        .open_with_setting(runtime.clone(), serial_setting)
        .expect(&format!("Error opening serial port {:?}", &arduino_port));
}

enum ButtonLEDByte {
    EndMarker = 255,
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

fn enter_main_menu_button_leds_system(
    mut serial_resource: ResMut<SerialResource>,
    arduino_port: Res<ArduinoSerialPort>,
) {
    serial_resource.send_message(&(*arduino_port), ButtonLEDByte::enter_main_menu());
}

fn enter_character_selection_button_leds_system(
    mut serial_resource: ResMut<SerialResource>,
    arduino_port: Res<ArduinoSerialPort>,
) {
    serial_resource.send_message(&(*arduino_port), ButtonLEDByte::enter_character_selection());
}

fn character_selection_button_leds_system(
    mut serial_resource: ResMut<SerialResource>,
    arduino_port: Res<ArduinoSerialPort>,
    mut player_join_event: EventReader<PlayerJoinEvent>,
) {
    for event in player_join_event.iter() {
        if event.0 == 0 {
            serial_resource.send_message(&(*arduino_port), ButtonLEDByte::player_one_joined());
        } else if event.0 == 1 {
            serial_resource.send_message(&(*arduino_port), ButtonLEDByte::player_two_joined());
        }
    }
}

fn enter_game_button_leds_system(
    mut serial_resource: ResMut<SerialResource>,
    arduino_port: Res<ArduinoSerialPort>,
) {
    serial_resource.send_message(&(*arduino_port), ButtonLEDByte::enter_game());
}

fn enter_pause_button_leds_system(
    mut serial_resource: ResMut<SerialResource>,
    arduino_port: Res<ArduinoSerialPort>,
) {
    serial_resource.send_message(&(*arduino_port), ButtonLEDByte::enter_pause());
}

fn enter_gameover_button_leds_system(
    mut serial_resource: ResMut<SerialResource>,
    arduino_port: Res<ArduinoSerialPort>,
) {
    serial_resource.send_message(&(*arduino_port), ButtonLEDByte::enter_gameover());
}

fn enter_victory_button_leds_system(
    mut serial_resource: ResMut<SerialResource>,
    arduino_port: Res<ArduinoSerialPort>,
) {
    serial_resource.send_message(&(*arduino_port), ButtonLEDByte::enter_victory());
}
