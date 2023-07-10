use std::iter::Peekable;
use std::slice::Iter;

use super::commands_enum::{Command, CommandBytes, CommandError, PushToCommand};
pub type AddLength = u8;

#[derive(Debug, PartialEq, Default)]
pub struct RemoveCommand {
    new_bytes: Vec<u8>,
}

impl RemoveCommand {
    pub fn new(new_bytes: Vec<u8>) ->  Result<Self, CommandError> {
        if new_bytes.len() > AddLength::MAX.into() {
            return Err(CommandError::ByteLimitReached(AddLength::MAX.into()));
        }
        Ok(Self { new_bytes})
    }

    pub fn length(&self) -> AddLength {
        self.new_bytes.len() as AddLength
    }

    pub fn bytes(&self) -> &[u8] {
        &self.new_bytes
        
    }
}

impl PushToCommand for RemoveCommand {
    fn push(&mut self, byte: u8) -> Result<(), CommandError> {
        if self.new_bytes.len() + 1 > AddLength::MAX.into() {
            return Err(CommandError::ByteLimitReached(AddLength::MAX.into()));
        }
        self.new_bytes.push(byte);
        Ok(())
    }

    fn push_chunk(&mut self, bytes: &[u8]) -> Result<(), CommandError>
    where
        Self: Sized,
    {
        if self.new_bytes.len() + bytes.len() > AddLength::MAX.into() {
            return Err(CommandError::ByteLimitReached(AddLength::MAX.into()));
        }
        self.new_bytes.extend_from_slice(bytes);
        Ok(())
    }
}

impl CommandBytes for RemoveCommand {
    const COMMAND_SIGN: u8 = b'-';

    fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![Self::COMMAND_SIGN];
        let command_bytes = self.bytes();
        bytes.extend_from_slice(&(command_bytes.len() as AddLength).to_be_bytes());
        bytes.extend_from_slice(command_bytes);
        bytes
    }

    fn from_bytes(bytes: &mut Peekable<Iter<'_, u8>>) -> Result<Self, CommandError>
    where
        Self: Sized,
    {
        if bytes.next().ok_or(CommandError::ExpectedCommandSign(Self::COMMAND_SIGN))? != &Self::COMMAND_SIGN {
            return Err(CommandError::ExpectedCommandSign(Self::COMMAND_SIGN));
        };
        let length = *bytes.next().ok_or(CommandError::ExpectedCommandLength)?;
        let mut add = RemoveCommand::default();
        for _ in 0..length {
            add.push( *bytes.next().ok_or(CommandError::ExpectedChangeBytes)?)?;
        }
        Ok(add)
    }
}

impl From<RemoveCommand> for Vec<u8> {
    fn from(value: RemoveCommand) -> Self {
        value.as_bytes()
    }
}

impl TryFrom<&mut Peekable<Iter<'_, u8>>> for RemoveCommand {
    type Error = CommandError;

    fn try_from(value: &mut Peekable<Iter<'_, u8>>) -> Result<Self, Self::Error> {
        RemoveCommand::from_bytes(value)
    }
}

impl From<RemoveCommand> for Command {
    fn from(value: RemoveCommand) -> Self {
        Command::Remove(value)
    }
}

#[cfg(test)]
mod copy_command_tests {
    use super::*;

    #[test]
    fn new() {
        let remove = RemoveCommand::default();
        assert_eq!(remove.length(), 0);
        assert_eq!(remove.bytes(), b"");

        let remove = RemoveCommand::new(b"AAA".to_vec()).unwrap();
        assert_eq!(remove.length(), 3);
        assert_eq!(remove.bytes(), b"AAA");

        let add = RemoveCommand::new(vec![0; AddLength::MAX as usize]).unwrap();
        assert_eq!(add.length(), AddLength::MAX);
        assert_eq!(add.bytes(), vec![0; AddLength::MAX as usize]);

        let remove = RemoveCommand::new(vec![0; AddLength::MAX as usize + 1]);
        assert!(remove.is_err());
        assert_eq!(remove.unwrap_err(), CommandError::ByteLimitReached(AddLength::MAX.into()));
    }

    #[test]
    fn push() {
        let mut remove = RemoveCommand::new(vec![0; AddLength::MAX as usize -1]).unwrap();
        assert_eq!(remove.length(), AddLength::MAX -1);
        assert!(remove.push(b'A').is_ok());
        assert_eq!(remove.length(), AddLength::MAX);
        assert!(remove.push(b'A').is_err());
    }

    #[test]
    fn push_chunk() {
        let mut remove = RemoveCommand::new(vec![0; AddLength::MAX as usize -2]).unwrap();
        assert_eq!(remove.length(), AddLength::MAX -2);
        assert!(remove.push_chunk(b"AA").is_ok());
        assert_eq!(remove.length(), AddLength::MAX);
        assert!(remove.push_chunk(b"AA").is_err());
    }

    #[test]
    fn as_bytes() {
        assert_eq!(RemoveCommand::default().as_bytes(), vec![RemoveCommand::COMMAND_SIGN, 0]);
        assert_eq!(RemoveCommand::new(b"AAA".to_vec()).unwrap().as_bytes(), vec![RemoveCommand::COMMAND_SIGN, 3, b'A', b'A', b'A']);
    }

    #[test]
    fn from_bytes() {
        let bytes = vec![RemoveCommand::COMMAND_SIGN, 0 as u8];
        let remove = RemoveCommand::try_from(&mut bytes.iter().peekable());
        assert!(remove.is_ok());
        assert_eq!(remove.unwrap(), RemoveCommand::default());

        let bytes = b"lol";
        let remove = RemoveCommand::try_from(&mut bytes.iter().peekable());
        assert!(remove.is_err());
        assert_eq!(remove.unwrap_err(), CommandError::ExpectedCommandSign(RemoveCommand::COMMAND_SIGN));
    }
}
