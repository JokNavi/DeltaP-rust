use super::instruction::{PushByte, ByteChunk, ChunkLength, ToBytes};

pub type AddSize = u8;

#[derive(Debug, PartialEq, Default)]
pub struct Add {
    byte_chunk: Vec<u8>,
    chunk_length: AddSize,
}

impl Add {
    pub fn with_capacity(chunk_length: AddSize) -> Self {
        Self {
            byte_chunk: Vec::with_capacity(chunk_length as usize),
            chunk_length: 0,
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
        self.chunk_length += 1; 
    }

    fn push_slice(&mut self, slice: &[u8]) {
        self.byte_chunk.extend_from_slice(slice);
        self.chunk_length += slice.len() as u8;
    }
}

impl ToBytes for Add {
    fn to_bytes(&self) -> Vec<u8> { 
        let mut bytes = vec![b'+', self.chunk_length];
        bytes.extend_from_slice(self.bytes());
        bytes
    }
}
