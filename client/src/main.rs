use bevy::app::ScheduleRunnerSettings;
use bevy::prelude::*;
use bevy::utils::Duration;
use laminar::SocketEvent;

use crate::network_resource::NetworkResource;
use crate::stdin::StdinReceiver;

mod network_resource;
mod stdin;

// Change this to your servers IP and Port
const SERVER_IP: &str = "127.0.0.1:12345";

fn main() {
    App::build()
        .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
            1.0 / 60.0,
        )))
        .add_plugins(MinimalPlugins)
        .init_resource::<NetworkResource>()
        .add_startup_system(setup.system())
        .add_system(message_sender.system())
        .add_system(message_handler.system())
        .run();
}

fn setup(mut net: ResMut<NetworkResource>) {
    net.set_server(SERVER_IP.parse().unwrap());
}

fn message_sender(stdin: Local<StdinReceiver>, net: Res<NetworkResource>) {
    for input in stdin.try_iter() {
        if let Err(e) = net.send(input.into()) {
            eprintln!("failed to send packet: {}", e);
        }
    }
}

fn message_handler(net: Res<NetworkResource>) {
    for msg in net.receiver().try_iter() {
        match msg {
            SocketEvent::Packet(packet) => {
                let msg = String::from_utf8_lossy(packet.payload());
                println!("{}", msg);
            }
            _ => {}
        }
    }
}
