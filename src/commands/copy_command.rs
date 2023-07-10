use super::commands_enum::{PushToCommand, CommandError};

pub type CopyAmount = u8;
const COPY_COMMAND_SIGN: u8 = b'#';

#[derive(Debug, PartialEq, Default)]
pub struct Copy {
    amount: CopyAmount,
}

impl Copy {
    pub fn new(amount: CopyAmount) -> Self {
        Self { amount } 
    }
}

impl PushToCommand for Copy {
    fn push(&mut self, _: u8) -> Result<(), CommandError>{
        self.amount = self.amount.checked_add(1).ok_or(CommandError::ByteLimitReached(CopyAmount::MAX.into()))?;
        Ok(())
    }

    fn push_chunk(&mut self, bytes: &[u8]) -> Result<(), CommandError> where Self: Sized{
        self.amount = self.amount.checked_add(bytes.len() as u8).ok_or(CommandError::ByteLimitReached(CopyAmount::MAX.into()))?;
        Ok(())
    }
}
