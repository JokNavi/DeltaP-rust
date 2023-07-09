
use super::{
    add_command::AddCommand,
    copy_command::CopyCommand,
    reference_command::{ReferenceCommand, ReferenceCommandIndex},
    remove_command::RemoveCommand,
};
 
pub const REFERENCE_COPY_COMMANDS: bool = false;

#[derive(Debug, PartialEq)]
pub enum ChunkError {
    ChunkLengthOverFlow,
}

#[derive(Debug, PartialEq)]
pub enum FromBytesError {
    InvalidSign,
    ExpectedChunkLength,
    ExpectedChunk,
    ExpectedIndex,
}

pub trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
}

pub trait AppendCommand {
    fn push_command(&mut self, command: Command) -> ();
}

pub trait FindReference {
    fn find_reference(&self, command: &Command) -> Option<ReferenceCommand>;
}

#[derive(Debug, PartialEq)]
pub enum Command {
    Copy(CopyCommand),
    Add(AddCommand),
    Remove(RemoveCommand),
    Reference(ReferenceCommand),
}

impl Command {
    pub fn copy(&self) -> Option<&CopyCommand> {
        match self {
            Command::Copy(copy) => Some(copy),
            _ => None,
        }
    }

    pub fn add(&self) -> Option<&AddCommand> {
        match self {
            Command::Add(add) => Some(add),
            _ => None,
        }
    }

    pub fn remove(&self) -> Option<&RemoveCommand> {
        match self {
            Command::Remove(remove) => Some(remove),
            _ => None,
        }
    }

    pub fn reference(&self) -> Option<&ReferenceCommand> {
        match self {
            Command::Reference(reference) => Some(reference),
            _ => None,
        }
    }
}

impl AppendCommand for Vec<Command> {
    fn push_command(&mut self, command: Command) -> () {
        if let Some(reference) = self.find_reference(&command) {
            self.push(Command::Reference(reference));
        } else {
            self.push(command);
        }
    }
}


impl FindReference for [Command] {
    fn find_reference(&self, command: &Command) -> Option<ReferenceCommand> {
        if let Some(slice) = self.get(ReferenceCommandIndex::MAX as usize..) {
            return slice.find_reference(command);
        }
        self.iter()
            .position(|vector_command| vector_command == command && (!matches!(vector_command, Command::Copy(_)) || REFERENCE_COPY_COMMANDS))
            .map(|index| ReferenceCommand::new(index as u8))
    }
}


impl ToBytes for Command {
    fn to_bytes(&self) -> Vec<u8> {
        match self {
            Command::Copy(copy) => copy.to_bytes(),
            Command::Add(add) => add.to_bytes(),
            Command::Remove(remove) => remove.to_bytes(),
            Command::Reference(reference) => reference.to_bytes(),
        }
    }
}

#[cfg(test)]
mod command_tests {
    use crate::commands::{copy_command::CopyCommand, reference_command::ReferenceCommand};
    use super::{Command, AppendCommand, REFERENCE_COPY_COMMANDS, FindReference};

    #[test]
    fn push_command() {
        let mut commands: Vec<Command> = vec![];
        println!("REFERENCE_COPY_COMMANDS: {}", REFERENCE_COPY_COMMANDS);
        commands.push_command(CopyCommand::default().into());
        assert_eq!(commands, vec![CopyCommand::default().into()]);
        commands.push_command(CopyCommand::default().into());
        if REFERENCE_COPY_COMMANDS {
            assert_eq!(commands, vec![CopyCommand::default().into(), ReferenceCommand::new(0).into()]);
        }
        else {
            assert_eq!(commands, vec![CopyCommand::default().into(), CopyCommand::default().into()]);
        }
    }

    #[test]
    fn find_reference() {
        let commands: Vec<Command> = vec![CopyCommand::default().into()];
        if REFERENCE_COPY_COMMANDS {
            assert!(commands.find_reference(&CopyCommand::default().into()).is_some());
            assert!(commands.find_reference(&CopyCommand::new(1).into()).is_none());
        }
        else {
            assert!(commands.find_reference(&CopyCommand::default().into()).is_none());
            assert!(commands.find_reference(&CopyCommand::new(1).into()).is_none());
        }      
    }
}
