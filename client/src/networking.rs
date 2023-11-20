use std::hash::Hasher;
use std::io::Write;
use std::sync::mpsc::{channel, Receiver, Sender};

use tracing::info;

use quinn::{ClientConfig, Endpoint};
use shared::packet_ext::{PacketStreamReadExt, PacketStreamWriteExt};
use shared::protocol::{PacketAction, ReliablePacket, UnreliablePacket};
use std::net::{TcpStream, UdpSocket};
use std::sync::Arc;

pub enum TcpEvent {
    PacketReceived { packet: ReliablePacket },
}

pub enum UdpEvent {
    PacketReceived { packet: UnreliablePacket },
}

pub async fn init() {
    let endpoint = create_client();
    let connection = endpoint
        .connect(shared::TCP_ADDRESS.parse().unwrap(), "localhost")
        .unwrap()
        .await
        .unwrap();

    let (mut handshake_send, mut handshake_recv) = connection.open_bi().await.unwrap();

    handshake_send.write("hello".as_bytes()).await.unwrap();

    loop {}

    // (id, tcp_receiver, udp_receiver, packet_action_sender)
}

fn create_client() -> Endpoint {
    let client_config = rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_custom_certificate_verifier(SkipServerVerification::new())
        .with_no_client_auth();

    let client_config = ClientConfig::new(Arc::new(client_config));

    let mut endpoint = Endpoint::client("127.0.0.1:0".parse().unwrap()).unwrap();
    endpoint.set_default_client_config(client_config);
    endpoint
}

struct SkipServerVerification;

impl SkipServerVerification {
    fn new() -> Arc<Self> {
        Arc::new(Self)
    }
}

impl rustls::client::ServerCertVerifier for SkipServerVerification {
    fn verify_server_cert(
        &self,
        _end_entity: &rustls::Certificate,
        _intermediates: &[rustls::Certificate],
        _server_name: &rustls::ServerName,
        _scts: &mut dyn Iterator<Item = &[u8]>,
        _ocsp_response: &[u8],
        _now: std::time::SystemTime,
    ) -> Result<rustls::client::ServerCertVerified, rustls::Error> {
        Ok(rustls::client::ServerCertVerified::assertion())
    }
}
