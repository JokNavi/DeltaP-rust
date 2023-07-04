use super::instruction::{ChunkLength, PushByte, ToBytes};
use std::{
    iter::{Enumerate, Peekable, Zip},
    slice::Iter,
};

pub type CopySize = u8;

#[derive(Debug, Default)]
pub struct Copy {
    chunk_length: CopySize,
}

impl ChunkLength for Copy {
    type Output = CopySize;

    fn length(&self) -> Self::Output {
        self.chunk_length
    }
}

impl PushByte for Copy {
    fn push(&mut self, byte: u8) {
        self.chunk_length += 1;
    }

    fn push_slice(&mut self, slice: &[u8]) {
        todo!();
    }
}

impl ToBytes for Copy {
    const BYTE_SIGN: u8 = 'C' as u8;

    fn to_bytes(&self) -> Vec<u8> {
        vec![Self::BYTE_SIGN, self.chunk_length]
    }
}
