use std::iter::Peekable;
use std::slice::Iter;

use super::commands_enum::{Command, CommandBytes, CommandError, PushToCommand};
pub type AddLength = u8;

#[derive(Debug, PartialEq, Default)]
pub struct AddCommand {
    new_bytes: Vec<u8>,
}

impl AddCommand {
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

impl PushToCommand for AddCommand {
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

impl CommandBytes for AddCommand {
    const COMMAND_SIGN: u8 = b'+';

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
        let mut add = AddCommand::default();
        for _ in 0..length {
            add.push( *bytes.next().ok_or(CommandError::ExpectedChangeBytes)?)?;
        }
        Ok(add)
    }
}

impl From<AddCommand> for Vec<u8> {
    fn from(value: AddCommand) -> Self {
        value.as_bytes()
    }
}

impl TryFrom<&mut Peekable<Iter<'_, u8>>> for AddCommand {
    type Error = CommandError;

    fn try_from(value: &mut Peekable<Iter<'_, u8>>) -> Result<Self, Self::Error> {
        AddCommand::from_bytes(value)
    }
}

impl From<AddCommand> for Command {
    fn from(value: AddCommand) -> Self {
        Command::Add(value)
    }
}

#[cfg(test)]
mod copy_command_tests {
    use super::*;

    #[test]
    fn new() {
        let add = AddCommand::default();
        assert_eq!(add.length(), 0);
        assert_eq!(add.bytes(), b"");

        let add = AddCommand::new(b"AAA".to_vec()).unwrap();
        assert_eq!(add.length(), 3);
        assert_eq!(add.bytes(), b"AAA");

        let add = AddCommand::new(vec![0; AddLength::MAX as usize]).unwrap();
        assert_eq!(add.length(), AddLength::MAX);
        assert_eq!(add.bytes(), vec![0; AddLength::MAX as usize]);

        let add = AddCommand::new(vec![0; AddLength::MAX as usize + 1]);
        assert!(add.is_err());
        assert_eq!(add.unwrap_err(), CommandError::ByteLimitReached(AddLength::MAX.into()));
    }

    #[test]
    fn push() {
        let mut add = AddCommand::new(vec![0; AddLength::MAX as usize -1]).unwrap();
        assert_eq!(add.length(), AddLength::MAX -1);
        assert!(add.push(b'A').is_ok());
        assert_eq!(add.length(), AddLength::MAX);
        assert!(add.push(b'A').is_err());
    }

    #[test]
    fn push_chunk() {
        let mut add = AddCommand::new(vec![0; AddLength::MAX as usize -2]).unwrap();
        assert_eq!(add.length(), AddLength::MAX -2);
        assert!(add.push_chunk(b"AA").is_ok());
        assert_eq!(add.length(), AddLength::MAX);
        assert!(add.push_chunk(b"AA").is_err());
    }

    #[test]
    fn as_bytes() {
        assert_eq!(AddCommand::default().as_bytes(), vec![AddCommand::COMMAND_SIGN, 0]);
        assert_eq!(AddCommand::new(b"AAA".to_vec()).unwrap().as_bytes(), vec![AddCommand::COMMAND_SIGN, 3, b'A', b'A', b'A']);
    }

    #[test]
    fn from_bytes() {
        let bytes = vec![AddCommand::COMMAND_SIGN, 0 as u8];
        let add = AddCommand::try_from(&mut bytes.iter().peekable());
        assert!(add.is_ok());
        assert_eq!(add.unwrap(), AddCommand::default());

        let bytes = b"lol";
        let add = AddCommand::try_from(&mut bytes.iter().peekable());
        assert!(add.is_err());
        assert_eq!(add.unwrap_err(), CommandError::ExpectedCommandSign(AddCommand::COMMAND_SIGN));
    }
}
