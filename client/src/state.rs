use crate::networking::{TcpEvent, UdpEvent};
use crate::player::Player;
use crate::renderer::Renderer;
use shared::protocol::{PacketAction, ReliablePacket, UnreliablePacket};
use std::sync::mpsc::{Receiver, Sender};
use winit::window::Window;

pub struct ClientState {
    pub renderer: Renderer,
    pub player: Player,
    pub window: Window,

    pub tcp_receiver: Receiver<TcpEvent>,
    pub udp_receiver: Receiver<UdpEvent>,
    pub packet_action_sender: Sender<PacketAction>,
}

impl ClientState {
    pub fn send_packet_action(&self, packet_action: PacketAction) {
        self.packet_action_sender.send(packet_action).unwrap();
    }

    pub fn send_reliable_packet(&self, reliable_packet: ReliablePacket) {
        self.send_packet_action(PacketAction::Reliable(reliable_packet))
    }

    pub fn send_unreliable_packet(&self, unreliable_packet: UnreliablePacket) {
        self.send_packet_action(PacketAction::Unreliable(unreliable_packet))
    }
}
