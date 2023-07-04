use crate::instructions::instruction::{ByteChunk, ChunkLength, ToBytes};
use std::{iter::{Peekable, Zip}};
use std::slice::Iter;
use std::iter::Enumerate;

use super::instruction::PushByte;

pub type RemoveSize = u8;

#[derive(Debug, PartialEq, Default)]
pub struct Remove {
    byte_chunk: Vec<u8>,
    chunk_length: RemoveSize,
}

impl Remove {
    pub fn new(chunk_length: RemoveSize) -> Self {
        Self {
            byte_chunk: Vec::with_capacity(chunk_length as usize),
            chunk_length: chunk_length,
        }
    }
}

impl ChunkLength for Remove {
    type Output = RemoveSize;

    fn length(&self) -> Self::Output {
        self.chunk_length
    }
}

impl ByteChunk for Remove {
    fn bytes(&self) -> &[u8] {
        self.byte_chunk.as_slice()
    }
}

impl PushByte for Remove {
    
    fn push(&mut self, byte: u8) {
        self.byte_chunk.push(byte);
    }

    fn push_slice(&mut self, slice: &[u8]) {
        todo!()
    }
}

impl ToBytes for Remove {
    const BYTE_SIGN: u8 = '-' as u8;
    fn to_bytes(&self) -> Vec<u8> { 
        let mut bytes = vec![Self::BYTE_SIGN, self.chunk_length];
        bytes.extend_from_slice(self.bytes());
        bytes
    }
}
