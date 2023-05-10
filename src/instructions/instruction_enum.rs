use super::{
    add_instruction::AddInstruction, copy_instruction::CopyInstruction,
    reference_instruction::ReferenceInstruction,
};

pub enum Instruction {
    Add {
        instruction: AddInstruction,
        hash: Option<u64>,
        index: u16,
    },
    Copy {
        instruction: CopyInstruction,
        hash: Option<u64>,
        index: u16,
    },
    Reference {
        instruction: ReferenceInstruction,
        hash: Option<u64>,
        index: u16,
    },
}

