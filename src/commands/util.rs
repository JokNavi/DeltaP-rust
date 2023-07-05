use super::{copy::CopyCommand, add::AddCommand};

#[derive(Debug, PartialEq)]
pub enum ChunkError {
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
    Add(AddCommand),
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

    pub fn add(&self) -> Option<&AddCommand> {
        match self {
            Command::Add(add) => Some(add),
            _ => None
        }
    }

    //TODO
}

impl ToBytes for Command {

    fn to_bytes(&self) -> Vec<u8> {
        match self {
            Command::Copy(copy) => copy.to_bytes(),
            Command::Add(add) => add.to_bytes(),
            Command::Remove(_) => todo!(),
            Command::Reference(_) => todo!(),
        }
    }
}

