use super::copy::CopyCommand;

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

pub enum Command {
    Copy(CopyCommand),
    Add(()),
    Remove(()),
    Reference(()),
}

impl Command {
    pub fn copy(&self) -> Option<&CopyCommand> {
        match self {
            Command::Copy(copy) => Some(copy),
            _ => None
        }
    }

    //TODO
}

impl ToBytes for Command {

    fn to_bytes(&self) -> Vec<u8> {
        match self {
            Command::Copy(copy) => copy.to_bytes(),
            Command::Add(_) => todo!(),
            Command::Remove(_) => todo!(),
            Command::Reference(_) => todo!(),
        }
    }
}

