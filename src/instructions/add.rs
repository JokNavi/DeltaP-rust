use super::instruction::{PushByte, ByteChunk, ChunkLength, ToBytes};
use std::{iter::Peekable};
use std::{slice::Iter};
use itertools::ZipLongest;

pub type AddSize = u8;

#[derive(Debug, PartialEq, Default)]
pub struct Add {
    byte_chunk: Vec<u8>,
    chunk_length: AddSize,
}

impl Add {
    pub fn new(chunk_length: AddSize) -> Self {
        Self {
            byte_chunk: Vec::with_capacity(chunk_length as usize),
            chunk_length: chunk_length,
        }
    }
}

impl ChunkLength for Add {
    type Output = AddSize;

    fn length(&self) -> Self::Output {
        self.chunk_length
    }
}

impl ByteChunk for Add {
    fn bytes(&self) -> &[u8] {
        self.byte_chunk.as_slice()
    }

}

impl PushByte for Add {
    fn push(&mut self, byte: u8) {
        self.byte_chunk.push(byte);
    }

    fn push_slice(&mut self, slice: &[u8]) {
        todo!()
    }
}

impl ToBytes for Add {
    const BYTE_SIGN: u8 = '+' as u8;
    fn to_bytes(&self) -> Vec<u8> { 
        let mut bytes = vec![Self::BYTE_SIGN, self.chunk_length];
        bytes.extend_from_slice(self.bytes());
        bytes
    }
}
