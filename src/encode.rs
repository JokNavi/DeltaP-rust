use std::collections::HashMap;
use crate::instructions::Instruction;

struct Encode {
    source: Vec<u8>,
    target: Vec<u8>,
    instruction_map: HashMap<u64, Instruction>,
}

impl Encode {
    pub fn new(source: &[u8], target: &[u8]) -> Self {
        Self { source: source.to_vec(), target: target.to_vec(), instruction_map: HashMap::new() }

    }
}