use bevy::ecs::prelude::World;
use bevy::prelude::FromWorld;
use bevy::tasks::AsyncComputeTaskPool;
use crossbeam_channel::{Receiver, SendError, Sender};
use laminar::{Packet, Socket, SocketEvent};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

pub struct NetworkResource {
    server: SocketAddr,
    sender: Sender<Packet>,
    receiver: Receiver<SocketEvent>,
}

impl FromWorld for NetworkResource {
    fn from_world(world: &mut World) -> Self {
        let mut socket = Socket::bind_any().expect("failed to bind socket");

        let (sender, receiver) = (socket.get_packet_sender(), socket.get_event_receiver());

        let task_pool = world.get_resource::<AsyncComputeTaskPool>().unwrap();
        task_pool
            .spawn(async move { socket.start_polling() })
            .detach();

        Self {
            // we will set the actual server ip later
            server: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 12345),
            sender,
            receiver,
        }
    }
}

impl NetworkResource {
    pub fn send(&self, msg: Vec<u8>) -> Result<(), SendError<Packet>> {
        self.sender
            .send(Packet::reliable_unordered(self.server, msg))
    }

    pub fn set_server(&mut self, server_addr: SocketAddr) {
        self.server = server_addr;
    }

    pub fn receiver(&self) -> Receiver<SocketEvent> {
        self.receiver.clone()
    }
}
