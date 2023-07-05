use super::instruction::{ChunkLength, PushByte, ToBytes};
use std::{
    iter::{Enumerate, Peekable, Zip},
    slice::Iter,
};

pub type CopySize = u8;

#[derive(Debug, PartialEq, Default)]
pub struct Copy {
    chunk_length: CopySize,
}

impl Copy {
    pub fn new(chunk_length: CopySize) -> Self {
        Self { chunk_length }
    }
}

impl ChunkLength for Copy {
    type Output = CopySize;

    fn length(&self) -> Self::Output {
        self.chunk_length
    }
}

impl PushByte for Copy {
    fn push(&mut self, _: u8) {
        self.chunk_length += 1;
    }

    fn push_slice(&mut self, slice: &[u8]) {
        todo!();
    }
}

impl ToBytes for Copy {

    fn to_bytes(&self) -> Vec<u8> {
        vec![b'C', self.chunk_length]
    }
}
