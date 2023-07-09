use std::{slice::Iter, iter::{Peekable, Zip}};
use super::command_util::{ChunkError, FromBytesError, ToBytes, Command};

const COPY_COMMAND_SIGN: u8 = b'#';

#[derive(Debug, PartialEq, Default)]
pub struct CopyCommand {
    chunk_length: u8,
}

impl CopyCommand {
    pub fn new(chunk_length: u8) -> Self {
        Self { chunk_length }
    }

    pub fn chunk_length(&self) -> u8 {
        self.chunk_length
    }

    pub fn increment(&mut self) -> Result<(), ChunkError> {
        if self.chunk_length == u8::MAX {
            return Err(ChunkError::ChunkLengthOverFlow);
        } 
        self.chunk_length += 1;
        Ok(())
    }

    pub fn increment_by(&mut self, amount: u8) -> Result<(), ChunkError> {
        self.chunk_length = self.chunk_length.checked_add(amount).ok_or(ChunkError::ChunkLengthOverFlow)?;
        Ok(())
    }
}

impl ToBytes for CopyCommand {
    
    fn to_bytes(&self) -> Vec<u8> {
        vec![COPY_COMMAND_SIGN, self.chunk_length]
    }
}

impl TryFrom<&mut Iter<'_, u8>> for CopyCommand {
    type Error = FromBytesError;

    fn try_from(value: &mut Iter<'_, u8>) -> Result<Self, Self::Error> {
        let chunk_length = value.next().ok_or(FromBytesError::ExpectedChunkLength)?;
        Ok(Self::new(*chunk_length))
    }
}

impl From<CopyCommand> for Command {
    fn from(value: CopyCommand) -> Self {
        Command::Copy(value)
    }
}

impl From<&mut Peekable<Zip<&mut Iter<'_, u8>, &mut Iter<'_, u8>>>> for CopyCommand {
    fn from(value: &mut Peekable<Zip<&mut Iter<'_, u8>, &mut Iter<'_, u8>>>) -> Self {
        let mut copy = CopyCommand::default();
        while let Some((_, _)) = value.next_if(|(source_byte, target_byte)| source_byte == target_byte){
            if copy.increment().is_err(){
                break;
            }
        }
        copy
    }
} 


#[cfg(test)]
mod copy_command_tests {
    use crate::commands::{command_util::{ToBytes, Command}, copy_command::COPY_COMMAND_SIGN};
    use super::CopyCommand;

    #[test]
    fn increment() {
        let mut copy = CopyCommand::new(u8::MAX-1);
        assert!(copy.increment().is_ok());
        assert!(copy.increment().is_err());
        assert_eq!(copy.chunk_length(), u8::MAX);
    }

    #[test]
    fn increment_by() {
        let mut copy = CopyCommand::new(u8::MAX-1);
        assert!(copy.increment_by(1).is_ok());
        assert!(copy.increment_by(1).is_err());
        assert_eq!(copy.chunk_length(), u8::MAX);
    }

    #[test]
    fn try_from_iter_bytes() {
        let bytes = CopyCommand::default().to_bytes();
        let copy = CopyCommand::try_from(&mut bytes[1..].iter());
        assert!(copy.is_ok());
        assert_eq!(copy.unwrap().to_bytes(), bytes);
    }

    #[test]
    fn to_bytes() {
        let copy = CopyCommand::default();
        assert_eq!(copy.to_bytes(), vec![COPY_COMMAND_SIGN, 0]);
    }

    #[test]
    fn from() {
        let _: Command = CopyCommand::default().into();
    }
}