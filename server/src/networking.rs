use std::net::SocketAddr;
use std::sync::atomic::AtomicU32;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;

use quinn::{Endpoint, ServerConfig};
use tracing::info;

use shared::protocol::{PacketAction, ReliablePacket};

pub enum TcpEvent {
    NewConnection {
        id: u32,
        tcp_addr: SocketAddr,
        udp_addr: SocketAddr,
        packet_action_sender: Sender<PacketAction>,
    },

    PacketReceived {
        addr: SocketAddr,
        packet: ReliablePacket,
    },
}

pub enum UdpEvent {}

pub async fn init(ids: Arc<AtomicU32>) {
    let server = create_server();

    tokio::spawn(async move {
        loop {
            match server.accept().await {
                None => {}
                Some(incoming_connection) => {
                    info!("incoming {}", incoming_connection.remote_address());
                    match incoming_connection.await {
                        Ok(connection) => {
                            info!("{} connected successfully", connection.remote_address());
                            let (handshake_send, mut handshake_recv) =
                                connection.accept_bi().await.unwrap();

                            let mut buf = Vec::new();
                            handshake_recv.read(&mut buf).await.unwrap();

                            let data = String::from_utf8_lossy(&buf[..]);
                            info!("{} sent {}", connection.remote_address(), data);
                        }
                        Err(err) => {
                            dbg!(&err);
                        }
                    }
                }
            }
        }
    });
    // (tcp_receiver, udp_receiver)
}

fn create_server() -> Endpoint {
    let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()]).unwrap();
    let cert_der = cert.serialize_der().unwrap();
    let priv_key = cert.serialize_private_key_der();
    let priv_key = rustls::PrivateKey(priv_key);
    let cert_chain = vec![rustls::Certificate(cert_der.clone())];

    let mut server_config = ServerConfig::with_single_cert(cert_chain, priv_key).unwrap();
    let transport_config = Arc::get_mut(&mut server_config.transport).unwrap();
    transport_config.max_concurrent_uni_streams(0_u8.into());

    Endpoint::server(server_config, shared::TCP_ADDRESS.parse().unwrap()).unwrap()
}
