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
    pub fn with_capacity(chunk_length: RemoveSize) -> Self {
        Self {
            byte_chunk: Vec::with_capacity(chunk_length as usize),
            chunk_length: 0,
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
        self.chunk_length += 1; 
    }

    fn push_slice(&mut self, slice: &[u8]) {
        self.byte_chunk.extend_from_slice(slice);
        self.chunk_length += slice.len() as u8;
    }
}

impl ToBytes for Remove {
    fn to_bytes(&self) -> Vec<u8> { 
        let mut bytes = vec![b'-', self.chunk_length];
        bytes.extend_from_slice(self.bytes());
        bytes
    }
}
