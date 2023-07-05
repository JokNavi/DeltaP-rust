use std::slice::Iter;
use super::util::{ChunkLengthError, FromBytesError, ToBytes, Command};

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

    pub fn increment(&mut self) -> Result<(), ChunkLengthError> {
        if self.chunk_length == u8::MAX {
            return Err(ChunkLengthError::ChunkLengthOverFlow);
        } 
        self.chunk_length += 1;
        Ok(())
    }

    pub fn increment_by(&mut self, amount: u8) -> Result<(), ChunkLengthError> {
        self.chunk_length = self.chunk_length.checked_add(amount).ok_or(ChunkLengthError::ChunkLengthOverFlow)?;
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


#[cfg(test)]
mod copy_command_tests {
    use crate::commands::util::ToBytes;
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
}
