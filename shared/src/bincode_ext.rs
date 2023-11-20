use std::io::{Read, Write};

use bincode::{Decode, Encode};

pub trait BincodeStreamReadExt {
    fn read_decoded<D: Decode>(&mut self) -> D;
}

impl<T> BincodeStreamReadExt for T
where
    T: Read,
{
    fn read_decoded<D: Decode>(&mut self) -> D {
        bincode::decode_from_std_read(self, bincode::config::standard()).unwrap()
    }
}

pub trait BincodeStreamWriteExt {
    fn write_encoded<E: Encode>(&mut self, data: &E);
}

impl<T> BincodeStreamWriteExt for T
where
    T: Write,
{
    fn write_encoded<E: Encode>(&mut self, data: &E) {
        bincode::encode_into_std_write(data, self, bincode::config::standard()).unwrap();
    }
}

trait ToBuf {
    fn to_buf(self) -> Vec<u8>;
}

impl<D> ToBuf for &D
where
    D: Encode,
{
    fn to_buf(self) -> Vec<u8> {
        let mut buf = Vec::new();
        bincode::encode_into_slice(self, &mut buf, bincode::config::standard()).unwrap();
        buf
    }
}
