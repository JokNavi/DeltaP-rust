use std::{iter::Peekable, slice::Iter};

use super::{add_command::AddCommand, copy_command::CopyCommand, remove_command::RemoveCommand};

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
    pub fn copy(self) -> Option<CopyCommand> {
        return match self {
            Self::Copy(command) => Some(command),
            _ => None,
        }
    }

    pub fn add(self) -> Option<AddCommand> {
        return match self {
            Self::Add(command) => Some(command),
            _ => None,
        }
    }

    pub fn remove(self) -> Option<RemoveCommand> {
        return match self {
            Self::Remove(command) => Some(command),
            _ => None,
        }
    }
    
}
