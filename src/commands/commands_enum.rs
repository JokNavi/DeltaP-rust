use std::{slice::Iter, iter::Peekable};

use super::{copy_command::CopyCommand, add_command::AddCommand, remove_command::RemoveCommand};

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
    /// Push a byte to the command, modifying it.
    ///
    /// # Arguments
    ///
    /// * `byte` - The byte to push.
    ///
    /// # Returns
    ///
    /// * `Result<(), CommandError>` - Result indicating success or an error.
    fn push(&mut self, byte: u8) -> Result<(), CommandError>;

    /// Push a chunk of bytes to the command, modifying it.
    ///
    /// # Arguments
    ///
    /// * `bytes` - The chunk of bytes to push.
    ///
    /// # Returns
    ///
    /// * `Result<(), CommandError>` - Result indicating success or an error.
    fn push_chunk(&mut self, bytes: &[u8]) -> Result<(), CommandError>
    where
        Self: Sized;
}

/// Trait for converting a command to and from bytes.
pub trait CommandBytes {
    /// The command sign.
    const COMMAND_SIGN: u8;

    /// Convert the command to a vector of bytes.
    ///
    /// # Returns
    ///
    /// * `Vec<u8>` - The command as a vector of bytes, Prefixed with a command sign.
    fn as_bytes(&self) -> Vec<u8>;

    /// Convert a vector of bytes to a command instance.
    ///
    /// # Arguments
    ///
    /// * `bytes` - The vector of bytes.
    ///
    /// # Returns
    ///
    /// * `Result<Self, CommandError>` - Result containing the command instance or an error.
    ///
    /// # Errors
    ///
    /// The function returns an error if the expected command sign or length is not found in the vector of bytes.
    fn from_bytes(bytes: &mut Peekable<Iter<'_, u8>>) -> Result<Self, CommandError>
    where
        Self: Sized;
}