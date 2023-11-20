use std::net::SocketAddr;
use std::sync::mpsc::Sender;

use shared::protocol::{PacketAction, ReliablePacket, UnreliablePacket};

pub struct Player {
    id: u32,
    tcp_addr: SocketAddr,
    udp_addr: SocketAddr,
    packet_action_sender: Sender<PacketAction>,
}

impl Player {
    pub fn new(
        id: u32,
        tcp_addr: SocketAddr,
        udp_addr: SocketAddr,
        packet_action_sender: Sender<PacketAction>,
    ) -> Self {
        Self {
            id,
            tcp_addr,
            udp_addr,
            packet_action_sender,
        }
    }

    pub fn tcp_addr(&self) -> &SocketAddr {
        &self.tcp_addr
    }

    pub fn udp_addr(&self) -> &SocketAddr {
        &self.udp_addr
    }

    pub fn send_packet_action(&self, action: PacketAction) {
        self.packet_action_sender.send(action).unwrap();
    }

    pub async fn send_reliable_packet(&self, message: ReliablePacket) {
        self.send_packet_action(PacketAction::Reliable(message))
    }

    pub async fn send_unreliable_packet(&self, message: UnreliablePacket) {
        self.send_packet_action(PacketAction::Unreliable(message))
    }
}
