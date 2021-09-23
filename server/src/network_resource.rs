use bevy::prelude::*;
use bevy::tasks::AsyncComputeTaskPool;
use bevy::utils::HashSet;
use crossbeam_channel::{Receiver, SendError, Sender};
use laminar::{Packet, Socket, SocketEvent};
use std::net::SocketAddr;

pub struct NetworkResource {
    clients: HashSet<SocketAddr>,
    sender: Sender<Packet>,
    receiver: Receiver<SocketEvent>,
}

impl FromWorld for NetworkResource {
    fn from_world(world: &mut World) -> Self {
        let addr = "0.0.0.0:12345";
        let mut socket = Socket::bind(addr).expect("failed to bind socket");
        println!("listening on '{}'", addr);

        let (sender, receiver) = (socket.get_packet_sender(), socket.get_event_receiver());

        let task_pool = world.get_resource::<AsyncComputeTaskPool>().unwrap();
        let _task = task_pool
            .spawn(async move {
                socket.start_polling();
            })
            .detach();

        return Self {
            clients: HashSet::default(),
            sender,
            receiver,
        };
    }
}

impl NetworkResource {
    pub fn broadcast(&self, msg: Vec<u8>) {
        for client in self.clients.iter() {
            if let Err(e) = self.send(*client, msg.clone()) {
                error!("failed to send packet: {:#?}", e.0);
            }
        }
    }

    pub fn send(&self, addr: SocketAddr, msg: Vec<u8>) -> Result<(), SendError<Packet>> {
        self.sender.send(Packet::reliable_unordered(addr, msg))
    }

    pub fn receiver(&self) -> Receiver<SocketEvent> {
        self.receiver.clone()
    }

    pub fn insert_client(&mut self, addr: SocketAddr) {
        self.clients.insert(addr);
    }

    #[allow(dead_code)]
    pub fn remove_client(&mut self, addr: &SocketAddr) {
        self.clients.remove(addr);
    }
}
