use std::net::SocketAddr;
use std::sync::atomic::AtomicU32;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::lookup_host;

use tracing::{info, warn};

use shared::protocol::ReliablePacket;

use crate::game_loop::server_game_loop;
use crate::networking::TcpEvent;
use crate::player::Player;
use crate::state::ServerState;

mod game_loop;
mod networking;
mod player;
mod state;

const INTERVAL: Duration = Duration::from_millis(1000 / 60);

#[tokio::main]
async fn main() {
    shared::tracing::init();

    let ids = Arc::new(AtomicU32::new(0));
    networking::init(ids).await;

    loop {}

    //
    // let state = ServerState {
    //     tcp_receiver,
    //     udp_receiver,
    //     players: Default::default(),
    // };
    //
    // server_game_loop(state, update, fixed_update, INTERVAL);
}

fn update(state: &mut ServerState, _dt: &Duration) {}

fn fixed_update(state: &mut ServerState, dt: &Duration) {
    receive_packets(state, dt);
}

fn receive_packets(state: &mut ServerState, dt: &Duration) {
    receive_tcp_packets(state, dt);
}

fn receive_tcp_packets(state: &mut ServerState, _dt: &Duration) {
    while let Ok(packet) = state.tcp_receiver.try_recv() {
        match packet {
            TcpEvent::NewConnection {
                id,
                tcp_addr,
                udp_addr,
                packet_action_sender,
            } => {
                let player = Player::new(id, tcp_addr, udp_addr, packet_action_sender);
                state.players.insert(id, player);
                info!("new connection: addr: {tcp_addr}, player_id: {id}");
            }
            TcpEvent::PacketReceived { addr, packet } => {
                handle_tcp_packet_received(state, addr, packet)
            }
        }
    }
}

fn handle_tcp_packet_received(_state: &mut ServerState, addr: SocketAddr, packet: ReliablePacket) {
    match packet {
        _ => {
            warn!("{addr} sent an invalid packet")
        }
    }
    info!("{addr} sent {packet:?}")
}
