use crate::bincode_ext::BincodeStreamWriteExt;

pub enum PacketAction {
    Reliable(ReliablePacket),
    Unreliable(UnreliablePacket),
}

#[derive(bincode::Decode, bincode::Encode, Clone, Debug)]
pub enum ReliablePacket {
    Handshake {
        udp_port: u16,
    },
    HandshakeRes {
        player_id: u32,
    },
    MovementInput {
        directions: [bool; 6],
        rotations: [f32; 2],
    },
}

impl ReliablePacket {
    pub fn to_buf(self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.write_encoded(&self);
        buf
    }
}

#[derive(bincode::Decode, bincode::Encode, Clone, Debug)]
pub enum UnreliablePacket {}

impl UnreliablePacket {
    pub fn to_buf(self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.write_encoded(&self);
        buf
    }
}
