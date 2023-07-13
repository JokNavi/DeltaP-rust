use std::{iter::Peekable, slice::Iter};

use super::{add_command::AddCommand, copy_command::{CopyCommand}, remove_command::RemoveCommand};


#[derive(Debug, PartialEq)]
pub enum Command {
    Copy(CopyCommand),
    Add(AddCommand),
    Remove(RemoveCommand),
}

#[derive(Debug, PartialEq)]
pub enum CommandError {
    ByteLimitReached(u32),
    ExpectedCommandSign(u8),
    ExpectedCommandLength,
    ExpectedChangeBytes,
}

pub trait PushToCommand {
    fn push(&mut self, byte: u8) -> Result<(), CommandError>;

    fn push_chunk(&mut self, bytes: &[u8]) -> Result<(), CommandError>
    where
        Self: Sized;
}

pub trait CommandBytes {
    const COMMAND_SIGN: u8;

    fn as_bytes(&self) -> Vec<u8>;

    fn from_bytes(bytes: &mut Peekable<Iter<'_, u8>>) -> Result<Self, CommandError>
    where
        Self: Sized;
}

impl Command {
    pub fn copy(&self) -> Option<&CopyCommand> {
        return match self {
            Self::Copy(command) => Some(command),
            _ => None,
        }
    }

    pub fn add(&self) -> Option<&AddCommand> {
        return match self {
            Self::Add(command) => Some(command),
            _ => None,
        }
    }

    pub fn remove(&self) -> Option<&RemoveCommand> {
        return match self {
            Self::Remove(command) => Some(command),
            _ => None,
        }
    }
}

impl PushToCommand for Command {
    fn push(&mut self, byte: u8) -> Result<(), CommandError> {
        match self {
            Self::Copy(command) => command.push(byte),
            Self::Add(command) => command.push(byte),
            Self::Remove(command) => command.push(byte),
        }
    }

    fn push_chunk(&mut self, bytes: &[u8]) -> Result<(), CommandError>
    where
        Self: Sized {
        match self {
            Self::Copy(command) => command.push_chunk(bytes),
            Self::Add(command) => command.push_chunk(bytes),
            Self::Remove(command) => command.push_chunk(bytes),
        }
    }
}

impl CommandBytes for Command {
    const COMMAND_SIGN: u8 = b'?';

    fn as_bytes(&self) -> Vec<u8> {
        match self {
            Self::Copy(command) => command.as_bytes(),
            Self::Add(command) => command.as_bytes(),
            Self::Remove(command) => command.as_bytes(),
        }
    }

    fn from_bytes(bytes: &mut Peekable<Iter<'_, u8>>) -> Result<Self, CommandError>
    where
        Self: Sized {
        match bytes.peek() {
            Some(&&CopyCommand::COMMAND_SIGN) => Ok(Self::Copy(CopyCommand::from_bytes(bytes)?)),
            Some(&&AddCommand::COMMAND_SIGN) => Ok(Self::Add(AddCommand::from_bytes(bytes)?)),
            Some(&&RemoveCommand::COMMAND_SIGN) => Ok(Self::Remove(RemoveCommand::from_bytes(bytes)?)),
            _ => Err(CommandError::ExpectedCommandSign(Self::COMMAND_SIGN)),
        }
    }
    
}


#[cfg(test)]
mod command_enum_tests {
    use super::*;

    #[test]
    fn test_push_to_command() {
        let mut copy_command = Command::Copy(CopyCommand::default());
        assert_eq!(copy_command.push(b'A'), Ok(()));
    }

    #[test]
    fn test_command_bytes() {
        let command = Command::Copy(CopyCommand::default());
        assert_eq!(command.as_bytes(), vec![CopyCommand::COMMAND_SIGN, 0]);
    }

    #[test]
    fn test_command_methods() {
        let command = Command::Copy(CopyCommand::default());
        assert_eq!(command.copy(), Some(&CopyCommand::default()));
        assert_eq!(command.add(), None);
        assert_eq!(command.remove(), None);
    }
}
