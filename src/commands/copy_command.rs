use std::iter::Peekable;
use std::slice::Iter;

use super::commands_enum::{Command, CommandBytes, CommandError, PushToCommand};
pub type CopyLength = u8;

#[derive(Debug, PartialEq, Default)]
pub struct CopyCommand {
    length: CopyLength,
}

impl CopyCommand {
    pub fn new(length: CopyLength) -> Self {
        Self { length }
    }

    pub fn length(&self) -> CopyLength {
        self.length
    }
}

impl PushToCommand for CopyCommand {
    fn push(&mut self, _: u8) -> Result<(), CommandError> {
        self.length = self
            .length
            .checked_add(1)
            .ok_or(CommandError::ByteLimitReached(CopyLength::MAX.into()))?;
        Ok(())
    }

    fn push_chunk(&mut self, bytes: &[u8]) -> Result<(), CommandError>
    where
        Self: Sized,
    {
        self.length = self
            .length
            .checked_add(bytes.len() as u8)
            .ok_or(CommandError::ByteLimitReached(CopyLength::MAX.into()))?;
        Ok(())
    }
}

impl CommandBytes for CopyCommand {
    const COMMAND_SIGN: u8 = b'#';
    fn as_bytes(&self) -> Vec<u8> {
        vec![Self::COMMAND_SIGN, self.length]
    }

    fn from_bytes(bytes: &mut Peekable<Iter<'_, u8>>) -> Result<Self, CommandError>
    where
        Self: Sized,
    {
        if !matches!(bytes.next(), Some(&Self::COMMAND_SIGN)) {
            return Err(CommandError::ExpectedCommandSign(Self::COMMAND_SIGN));
        };
        Ok(Self::new(
            *bytes.next().ok_or(CommandError::ExpectedCommandLength)?,
        ))
    }
}

impl From<CopyCommand> for Vec<u8> {
    fn from(value: CopyCommand) -> Self {
        value.as_bytes()
    }
}

impl TryFrom<&mut Peekable<Iter<'_, u8>>> for CopyCommand {
    type Error = CommandError;

    fn try_from(value: &mut Peekable<Iter<'_, u8>>) -> Result<Self, Self::Error> {
        CopyCommand::from_bytes(value)
    }
}

impl From<CopyCommand> for Command {
    fn from(value: CopyCommand) -> Self {
        Command::Copy(value)
    }
}

#[cfg(test)]
mod copy_command_tests {
    use super::*;

    #[test]
    fn new() {
        assert_eq!(CopyCommand::new(0).length(), 0);
        assert_eq!(CopyCommand::default().length(), 0);
        assert_eq!(CopyCommand::new(CopyLength::MAX).length(), CopyLength::MAX);
    }

    #[test]
    fn push() {
        let mut command = CopyCommand::new(CopyLength::MAX-1);
        assert!(command.push(b'A').is_ok());
        assert_eq!(command.length(), CopyLength::MAX);
        assert!(command.push(b'A').is_err());
    }

    #[test]
    fn push_chunk() {
        let mut command = CopyCommand::new(CopyLength::MAX-2);
        assert!(command.push_chunk(b"AA").is_ok());
        assert_eq!(command.length(), CopyLength::MAX);
        assert!(command.push_chunk(b"AA").is_err());
    }

    #[test]
    fn as_bytes() {
        assert_eq!(CopyCommand::default().as_bytes(), vec![CopyCommand::COMMAND_SIGN, 0]);
        assert_eq!(CopyCommand::new(CopyLength::MAX).as_bytes(), vec![CopyCommand::COMMAND_SIGN, CopyLength::MAX]);
    }

    #[test]
    fn from_bytes() {
        let bytes = vec![CopyCommand::COMMAND_SIGN, 0 as u8];
        let copy = CopyCommand::try_from(&mut bytes.iter().peekable());
        assert!(copy.is_ok());
        assert_eq!(copy.unwrap(), CopyCommand::default());

        let bytes = b"lol";
        let copy = CopyCommand::try_from(&mut bytes.iter().peekable());
        assert!(copy.is_err());
        assert_eq!(copy.unwrap_err(), CommandError::ExpectedCommandSign(CopyCommand::COMMAND_SIGN));
    }
}
