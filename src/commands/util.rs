use super::{copy::CopyCommand, add::AddCommand, remove::RemoveCommand};

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
    Remove(RemoveCommand),
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

    pub fn remove(&self) -> Option<&RemoveCommand> {
        match self {
            Command::Remove(remove) => Some(remove),
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
            Command::Remove(remove) => remove.to_bytes(),
            Command::Reference(_) => todo!(),
        }
    }
}

