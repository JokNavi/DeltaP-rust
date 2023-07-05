
#[derive(Debug, PartialEq)]
pub enum ChunkLengthError {
    ChunkLengthOverFlow,
}

#[derive(Debug, PartialEq)]
pub enum FromBytesError {
    InvalidSign,
    ExpectedChunkLength,
    ExpectedChunk,
    ExpectedIndex,
}

pub trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
}