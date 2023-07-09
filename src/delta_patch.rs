use crate::commands::{
    add_command::AddCommand,
    command_util::{AppendCommand, Command, ToBytes},
    copy_command::CopyCommand,
    remove_command::RemoveCommand,
};

pub struct Patch {
    commands: Vec<Command>,
}

impl Patch {
    pub fn encode(source_bytes: &[u8], target_bytes: &[u8]) -> Self {
        let mut source_bytes_iter = source_bytes.iter();
        let mut target_bytes_iter = target_bytes.iter();
        let mut bytes_iterator = source_bytes_iter
            .by_ref()
            .zip(target_bytes_iter.by_ref())
            .peekable();

        let mut commands: Vec<Command> = vec![];
        while let Some((source_byte_peek, target_byte_peek)) = bytes_iterator.peek() {
            if source_byte_peek == target_byte_peek {
                commands.push_command(CopyCommand::from(&mut bytes_iterator).into());
            } else {
                let mut remove = RemoveCommand::default();
                let mut add = AddCommand::default();
                while let Some((source_byte, target_byte)) =
                    bytes_iterator.next_if(|(source_byte, target_byte)| source_byte != target_byte)
                {
                    if remove.push(*source_byte).is_err() {
                        commands.push_command(remove.into());
                        remove = RemoveCommand::default();
                    }
                    if add.push(*target_byte).is_err() {
                        commands.push_command(add.into());
                        add = AddCommand::default();
                    }
                }
                commands.push_command(remove.into());
                commands.push_command(add.into());
            }
        }

        if source_bytes_iter.len() > 0 {
            while source_bytes_iter.len() > u8::MAX as usize {
                commands.push_command(
                    RemoveCommand::new(
                        source_bytes_iter
                            .by_ref()
                            .take(u8::MAX as usize)
                            .copied()
                            .collect::<Vec<u8>>(),
                    )
                    .unwrap()
                    .into(),
                );
            }
            commands.push_command(
                RemoveCommand::new(
                    source_bytes_iter
                        .take(u8::MAX as usize)
                        .copied()
                        .collect::<Vec<u8>>(),
                )
                .unwrap()
                .into(),
            );
        }
        if target_bytes_iter.len() > 0 {
            while target_bytes_iter.len() > u8::MAX as usize {
                commands.push_command(
                    AddCommand::new(
                        target_bytes_iter
                            .by_ref()
                            .take(u8::MAX as usize)
                            .copied()
                            .collect::<Vec<u8>>(),
                    )
                    .unwrap()
                    .into(),
                );
            }
            commands.push_command(
                AddCommand::new(
                    target_bytes_iter
                        .take(u8::MAX as usize)
                        .copied()
                        .collect::<Vec<u8>>(),
                )
                .unwrap()
                .into(),
            );
        }
        Patch::new(commands)
    }

    fn new(commands: Vec<Command>) -> Self {
        Self { commands }
    }
}

impl ToBytes for Patch {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];
        for command in &self.commands {
            bytes.append(&mut command.to_bytes());
        }
        bytes
    }
}

#[derive(Debug, PartialEq)]
pub enum ApplyPatchError {}

pub trait ApplyPatch {
    fn to_target(&self, patch: &Patch) -> Result<Self, ApplyPatchError>
    where
        Self: Sized;
        
    fn to_source(&self, patch: &Patch) -> Result<Self, ApplyPatchError>
    where
        Self: Sized;
}

#[cfg(test)]
mod delta_patch_tests {
    use crate::{
        commands::{
            add_command::AddCommand,
            command_util::{Command, REFERENCE_COPY_COMMANDS},
            copy_command::CopyCommand,
            reference_command::ReferenceCommand,
            remove_command::RemoveCommand,
        },
        delta_patch::Patch,
    };

    #[test]
    fn encode() {
        let source_bytes = b"AAABBBCCCDDDEEE";
        let target_bytes = b"AAAFFFCCCFFFCCCFFF";

        if REFERENCE_COPY_COMMANDS {
            let expected_commands: Vec<Command> = vec![
                CopyCommand::new(3).into(),
                RemoveCommand::new(b"BBB".to_vec()).unwrap().into(),
                AddCommand::new(b"FFF".to_vec()).unwrap().into(),
                ReferenceCommand::new(0).into(),
                RemoveCommand::new(b"DDDEEE".to_vec()).unwrap().into(),
                AddCommand::new(b"FFFCCC".to_vec()).unwrap().into(),
                ReferenceCommand::new(2).into(),
            ];
            assert_eq!(
                expected_commands,
                Patch::encode(source_bytes, target_bytes).commands
            );
        } else {
            let expected_commands: Vec<Command> = vec![
                CopyCommand::new(3).into(),
                RemoveCommand::new(b"BBB".to_vec()).unwrap().into(),
                AddCommand::new(b"FFF".to_vec()).unwrap().into(),
                CopyCommand::new(3).into(),
                RemoveCommand::new(b"DDDEEE".to_vec()).unwrap().into(),
                AddCommand::new(b"FFFCCC".to_vec()).unwrap().into(),
                ReferenceCommand::new(2).into(),
            ];
            assert_eq!(
                expected_commands,
                Patch::encode(source_bytes, target_bytes).commands
            );
        }
    }
}
