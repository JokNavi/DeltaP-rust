use std::{slice::Iter, iter::{Peekable, Zip}};
use super::command_util::{ChunkError, ToBytes, FromBytesError, Command};

const REMOVE_COMMAND_SIGN: u8 = b'-';

#[derive(Debug, PartialEq, Default)]
pub struct RemoveCommand {
    byte_chunk: Vec<u8>,
}

impl RemoveCommand {
    pub fn new(byte_chunk: Vec<u8>) -> Result<Self, ChunkError> {
        if byte_chunk.len() <= u8::MAX.into() {
            return Ok(Self { byte_chunk });
        }
        Err(ChunkError::ChunkLengthOverFlow)
    }

    pub fn chunk_length(&self) -> u8 {
        self.byte_chunk.len() as u8
    }

    pub fn byte_chunk(&self) -> &[u8] {
        self.byte_chunk.as_slice()
    }

    pub fn push(&mut self, byte: u8) -> Result<(), ChunkError>{
        if self.byte_chunk.len() < u8::MAX as usize {
            self.byte_chunk.push(byte);
            return Ok(());
        }
        Err(ChunkError::ChunkLengthOverFlow)
    }

    pub fn extend_from_slice(&mut self, byte: &[u8]) -> Result<(), ChunkError>{
        if self.byte_chunk.len() + byte.len() <= u8::MAX as usize {
            self.byte_chunk.extend_from_slice(byte);
            return Ok(());
        }
        Err(ChunkError::ChunkLengthOverFlow)
    }
}

impl ToBytes for RemoveCommand {
    
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![REMOVE_COMMAND_SIGN, self.chunk_length()];
        bytes.extend(self.byte_chunk.iter());
        bytes        
    }
}

impl TryFrom<&mut Iter<'_, u8>> for RemoveCommand {
    type Error = FromBytesError;

    fn try_from(value: &mut Iter<'_, u8>) -> Result<Self, Self::Error> {
        let chunk_length = value.next().ok_or(FromBytesError::ExpectedChunkLength)?;
        let chunk: Vec<u8> = value.take(*chunk_length as usize).copied().collect();
        Ok(RemoveCommand::new(chunk).unwrap())
    }
}


impl From<RemoveCommand> for Command {
    fn from(value: RemoveCommand) -> Self {
        Command::Remove(value)
    }
}


#[cfg(test)]
mod add_command_tests {
    use crate::commands::{command_util::{ToBytes, Command}, remove_command::REMOVE_COMMAND_SIGN};
    use super::RemoveCommand;


    #[test]
    fn new() {
        assert!(RemoveCommand::new(vec![0;u8::MAX as usize]).is_ok());
        assert!(RemoveCommand::new(vec![0;u8::MAX as usize + 1]).is_err());
    }

    #[test]
    fn push() {
        let mut add = RemoveCommand::new(vec![0;u8::MAX as usize - 1]).unwrap();
        assert!(add.push(b'X').is_ok());
        assert_eq!(add.chunk_length(), u8::MAX);
        assert!(add.push(b'X').is_err());
        assert_eq!(add.chunk_length(), u8::MAX);
    } 

    #[test]
    fn extend_from_slice() {
        let mut add = RemoveCommand::new(vec![0;u8::MAX as usize - 3]).unwrap();
        assert!(add.extend_from_slice(b"XX").is_ok());
        assert_eq!(add.chunk_length(), u8::MAX - 1);
        assert!(add.extend_from_slice(b"XXX").is_err());
        assert_eq!(add.chunk_length(), u8::MAX - 1);
    } 

    #[test]
    fn to_bytes() {
        let mut add = RemoveCommand::default();
        assert_eq!(add.to_bytes(), vec![REMOVE_COMMAND_SIGN, 0]);
        add.extend_from_slice(b"XXX").unwrap();
        let mut expected_bytes = vec![REMOVE_COMMAND_SIGN, 3];
        expected_bytes.extend_from_slice(b"XXX");
        assert_eq!(add.to_bytes(), expected_bytes);
    }

    #[test]
    fn try_from_iter_bytes() {
        let bytes = RemoveCommand::default().to_bytes();
        let add = RemoveCommand::try_from(&mut bytes[1..].iter());
        assert!(add.is_ok());
        assert_eq!(add.unwrap().to_bytes(), bytes);
    }

    #[test]
    fn from() {
        let _: Command = RemoveCommand::default().into();
    }
}