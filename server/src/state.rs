use crate::networking::{TcpEvent, UdpEvent};
use crate::player::Player;
use std::collections::HashMap;
use std::sync::mpsc::Receiver;

pub struct ServerState {
    pub tcp_receiver: Receiver<TcpEvent>,
    pub udp_receiver: Receiver<UdpEvent>,
    pub players: HashMap<u32, Player>,
}
