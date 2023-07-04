use crate::instructions::instruction::{ByteChunk, ChunkLength, ToBytes};

pub type AddSize = u8;
const ADD_BYTE_SIGN: u8 = '+' as u8;

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

impl Default for Add {
    fn default() -> Self {
        Self {
            byte_chunk: Default::default(),
            chunk_length: Default::default(),
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

    fn push(&mut self, byte: u8) {
        self.byte_chunk.push(byte);
    }
}

impl ToBytes for Add {
    fn to_bytes(&self) -> Vec<u8> { 
        let mut bytes = vec![ADD_BYTE_SIGN, self.chunk_length];
        bytes.extend_from_slice(self.bytes());
        bytes
    }
}
