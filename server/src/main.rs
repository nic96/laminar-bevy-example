use bevy::app::ScheduleRunnerSettings;
use bevy::prelude::*;
use bevy::utils::Duration;
use laminar::SocketEvent;

use crate::network_resource::NetworkResource;

mod network_resource;

fn main() {
    // this app loops forever at 60 fps
    App::build()
        .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
            1.0 / 60.0,
        )))
        .add_plugins(MinimalPlugins)
        .init_resource::<NetworkResource>()
        .add_system(message_handler.system())
        .run();
}

fn message_handler(mut net: ResMut<NetworkResource>) {
    for msg in net.receiver().try_iter() {
        match msg {
            SocketEvent::Packet(packet) => {
                // might not a be a good solution, but we're just inserting a
                // client for every packet we receive
                net.insert_client(packet.addr());
                let msg = String::from_utf8_lossy(packet.payload());
                let response = format!("[{}]: {}", packet.addr(), msg);
                println!("{}", response);
                net.broadcast(response.into());
            }
            // // connect event only happens if we respond immediately
            // SocketEvent::Connect(new_client) => {
            //     net.insert_client(new_client);
            // }
            // // this will disconnect a client (remove it from our client list)
            // // if we don't receive a message after the `idle_connection_timeout`
            // // the default is 5 seconds
            // SocketEvent::Disconnect(client) => {
            //     net.remove_client(&client);
            // }
            _ => {}
        }
    }
}
