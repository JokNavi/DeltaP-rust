pub trait ChunkLength {
    type Output;

    fn length(&self) -> Self::Output;
}

pub trait ByteChunk {
    fn bytes(&self) -> &[u8];
    fn push(&mut self, byte: u8) -> ();
}

pub trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
}

