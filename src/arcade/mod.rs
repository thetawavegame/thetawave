use std::time::Duration;

use bevy::{app::ScheduleRunnerSettings, log::LogPlugin, prelude::*};
use bevy_serialport::{
    DataBits, FlowControl, Parity, SerialPortPlugin, SerialPortRuntime, SerialPortSetting,
    SerialResource, StopBits,
};

pub struct ArcadePlugin;

impl Plugin for ArcadePlugin {
    fn build(&self, app: &mut App) {
        //app.insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_millis(10)));

        app.add_plugin(SerialPortPlugin)
            .add_startup_system(setup_serial_system)
            .add_system(send_test_data_system);
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
    serial_res.send_message("COM3", vec![10].into());
}
