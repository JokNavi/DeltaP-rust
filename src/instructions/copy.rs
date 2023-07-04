use crate::instructions::instruction::{ChunkLength, ToBytes};

pub type CopySize = u8;

pub struct Copy {
    chunk_length: CopySize,
}

impl ChunkLength for Copy {
    type Output = CopySize;

    fn length(&self) -> Self::Output {
        self.chunk_length
    }
}


impl ToBytes for Copy {
    const BYTE_SIGN: u8 = 'C' as u8;

    fn to_bytes(&self) -> Vec<u8> { 
        vec![Self::BYTE_SIGN, self.chunk_length]
    }
}