use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream, UdpSocket};

use crate::bincode_ext::{BincodeStreamReadExt, BincodeStreamWriteExt};
use crate::protocol::{ReliablePacket, UnreliablePacket};

pub trait PacketStreamReadExt {
    fn read_reliable(&mut self) -> ReliablePacket;
    fn read_unreliable(&mut self) -> UnreliablePacket;
    fn read_unreliable_from(&mut self) -> (UnreliablePacket, SocketAddr);
}

impl PacketStreamReadExt for TcpStream {
    fn read_reliable(&mut self) -> ReliablePacket {
        let mut buf = [0u8; 4];
        self.read_exact(&mut buf).unwrap();
        let size = u32::from_be_bytes(buf);

        let mut buf = vec![0u8; size as usize];
        self.read_exact(&mut buf).unwrap();
        (&buf[..]).read_decoded()
    }

    fn read_unreliable(&mut self) -> UnreliablePacket {
        panic!("unreliable packets should not be received through TCP.")
    }

    fn read_unreliable_from(&mut self) -> (UnreliablePacket, SocketAddr) {
        panic!("unreliable packets should not be received through TCP.")
    }
}

impl PacketStreamReadExt for UdpSocket {
    fn read_reliable(&mut self) -> ReliablePacket {
        panic!("reliable packets should not be received through UDP.")
    }

    fn read_unreliable(&mut self) -> UnreliablePacket {
        let mut buf = [0u8; 4];
        self.recv(&mut buf).unwrap();
        let size = u32::from_be_bytes(buf);

        let mut buf = Vec::new();
        self.recv(&mut buf).unwrap();
        (&buf[..]).read_decoded()
    }

    fn read_unreliable_from(&mut self) -> (UnreliablePacket, SocketAddr) {
        let mut buf = [0u8; 4];
        let (_, addr) = self.recv_from(&mut buf).unwrap();
        let size = u32::from_be_bytes(buf);

        let mut buf = Vec::new();
        self.recv(&mut buf).unwrap();
        ((&buf[..]).read_decoded(), addr)
    }
}

pub trait PacketStreamWriteExt {
    fn write_reliable(&mut self, packet: &ReliablePacket);
    fn write_unreliable(&mut self, packet: &UnreliablePacket);
}

impl PacketStreamWriteExt for TcpStream {
    fn write_reliable(&mut self, packet: &ReliablePacket) {
        let mut buf = Vec::new();
        buf.write_encoded(packet);
        self.write_all(buf.len().to_be_bytes().as_slice()).unwrap();
        self.write_all(&buf[..]).unwrap();
    }

    fn write_unreliable(&mut self, _packet: &UnreliablePacket) {
        panic!("unreliable packets should not be sent through TCP.")
    }
}

impl PacketStreamWriteExt for UdpSocket {
    fn write_reliable(&mut self, _packet: &ReliablePacket) {
        panic!("reliable packets should not be sent through UDP.")
    }

    fn write_unreliable(&mut self, packet: &UnreliablePacket) {
        let mut buf = Vec::new();
        buf.write_encoded(packet);
        buf.reserve(4);
        let mut v = buf.split_off(0);
        buf.extend_from_slice(buf.len().to_be_bytes().as_slice());
        buf.append(&mut v);
        self.send(&buf[..]).unwrap();
    }
}
