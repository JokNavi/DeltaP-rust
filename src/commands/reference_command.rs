use std::{slice::Iter, iter::{Peekable, Zip}};

use super::command_util::{AppendCommand, Command, FindReference, FromBytesError, ToBytes};

const REFERENCE_COMMAND_SIGN: u8 = b'&';

pub type ReferenceCommandIndex = u8;

#[derive(Debug, PartialEq)]
pub struct ReferenceCommand {
    index: ReferenceCommandIndex,
}

impl ReferenceCommand {
    pub fn new(index: ReferenceCommandIndex) -> Self {
        Self { index }
    }

    pub fn index(&self) -> ReferenceCommandIndex {
        self.index
    }
}

impl ToBytes for ReferenceCommand {
    fn to_bytes(&self) -> Vec<u8> {
        vec![REFERENCE_COMMAND_SIGN, self.index()]
    }
}

impl TryFrom<&mut Iter<'_, u8>> for ReferenceCommand {
    type Error = FromBytesError;

    fn try_from(value: &mut Iter<'_, u8>) -> Result<Self, Self::Error> {
        Ok(ReferenceCommand::new(
            *value.next().ok_or(FromBytesError::ExpectedIndex)?,
        ))
    }
}


impl From<ReferenceCommand> for Command {
    fn from(value: ReferenceCommand) -> Self {
        Command::Reference(value)
    }
}


#[cfg(test)]
mod command_tests {
    use crate::commands::{command_util::{Command, ToBytes}};
    use super::{ReferenceCommand, REFERENCE_COMMAND_SIGN};

    #[test]
    fn to_bytes() {
        let reference = ReferenceCommand::new(0);
        assert_eq!(reference.to_bytes(), vec![REFERENCE_COMMAND_SIGN, 0]);
        let reference = ReferenceCommand::new(10);
        assert_eq!(reference.to_bytes(), vec![REFERENCE_COMMAND_SIGN, 10]);
    }

    #[test]
    fn try_from_iter_bytes() {
        let bytes = ReferenceCommand::new(0).to_bytes();
        let reference = ReferenceCommand::try_from(&mut bytes[1..].iter());
        assert!(reference.is_ok());
        assert_eq!(bytes, reference.unwrap().to_bytes());
    }

    #[test]
    fn from() {
        let _: Command = ReferenceCommand::new(0).into();
    }
}
