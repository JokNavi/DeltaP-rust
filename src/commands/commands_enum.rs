
pub enum Commands {
    Copy(u8),
    Add(u8),
}

pub enum CommandError {
    ByteLimitReached(u32),
}

pub trait PushToCommand {
    fn push(&mut self, byte: u8) -> Result<(), CommandError>;
    fn push_chunk(&mut self, bytes: &[u8]) -> Result<(), CommandError> where Self: Sized;
}