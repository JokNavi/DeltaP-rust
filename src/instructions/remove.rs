use crate::instructions::instruction::{ByteChunk, ChunkLength, ToBytes};

pub type RemoveSize = u8;

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

impl Default for Remove {
    fn default() -> Self {
        Self {
            byte_chunk: Default::default(),
            chunk_length: Default::default(),
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

    fn push(&mut self, byte: u8) {
        self.byte_chunk.push(byte);
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
