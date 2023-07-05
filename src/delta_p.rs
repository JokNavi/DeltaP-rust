use crate::instructions::{
    add::{Add, AddSize},
    copy::{Copy, CopySize},
    instruction::{Instruction, PushByte, ChunkLength, ToBytes, self},
    remove::{Remove, RemoveSize},
};

#[derive(Debug)]
pub struct DeltaPatch {
    instructions: Vec<Instruction>,
}

impl DeltaPatch {
    pub fn encode(source_bytes: &[u8], target_bytes: &[u8]) -> Self {
        let mut instructions: Vec<Instruction> = vec![];
        let mut zipped_iterator = source_bytes.iter().zip(target_bytes).peekable();
        while let Some((source_byte, target_byte)) = zipped_iterator.peek() {
            if source_byte == target_byte {
                let mut copy_instruction: Copy = Copy::default();
                while let Some((source_byte, _)) = zipped_iterator.next_if(|(source_byte, target_byte)| source_byte == target_byte){
                    copy_instruction.push(*source_byte);
                    if copy_instruction.length() == CopySize::MAX {
                        break
                    }
                }
                instructions.push(Instruction::Copy(copy_instruction));
            }
            else {
                let mut remove_instruction = Remove::default();
                let mut add_instruction = Add::default();
                while let Some((source_byte, target_byte)) = zipped_iterator.next_if(|(source_byte, target_byte)| source_byte != target_byte){
                    remove_instruction.push(*source_byte);
                    add_instruction.push(*target_byte);
                    if remove_instruction.length() == RemoveSize::MAX{
                        instructions.push(Instruction::Remove(remove_instruction).try_into_reference(&instructions));
                        remove_instruction = Remove::default();
                    }
                    if add_instruction.length() == AddSize::MAX{
                        instructions.push(Instruction::Add(add_instruction).try_into_reference(&instructions));
                        add_instruction = Add::default();
                    } 
                }
                instructions.push(Instruction::Remove(remove_instruction).try_into_reference(&instructions));
                instructions.push(Instruction::Add(add_instruction).try_into_reference(&instructions));
            }
        }
        if source_bytes.len() > target_bytes.len() {
            let mut remove_instruction = Remove::default();
            for slice in source_bytes.get(target_bytes.len()..).unwrap().chunks(AddSize::MAX as usize) {
                remove_instruction.push_slice(slice);
            }
            instructions.push(Instruction::Remove(remove_instruction).try_into_reference(&instructions));
        };
        if target_bytes.len() > source_bytes.len() {
            let mut add_instruction = Add::default();
            for slice in target_bytes.get(source_bytes.len()..).unwrap().chunks(AddSize::MAX as usize) {
                add_instruction.push_slice(slice);
            }
            instructions.push(Instruction::Add(add_instruction).try_into_reference(&instructions));
        };
        Self { instructions }
    }
}

impl ToBytes for DeltaPatch {
    fn to_bytes(&self) -> Vec<u8> {
        self.instructions.iter().map(|instruction| instruction.to_bytes()).flatten().collect()
    }
}

#[cfg(test)]
mod delta_p_tests {
    use crate::{
        delta_p::{DeltaPatch, Instruction},
        instructions::{add::Add, copy::Copy, instruction::{PushByte, ToBytes}, remove::Remove, reference::Reference},
    };

    #[test]
    fn encode() {
        let delta = DeltaPatch::encode(b"AAABBBAAABBB", b"AAAXXXAAADDDFFF");
       let mut remove_instruction = Remove::with_capacity(3);
        remove_instruction.push_slice(b"BBB");
        let mut add_instruction_one = Add::with_capacity(3);
        add_instruction_one.push_slice(b"XXX");
        let mut add_instruction_two = Add::with_capacity(3);
        add_instruction_two.push_slice(b"DDD");
        let mut add_instruction_three = Add::with_capacity(3);
        add_instruction_three.push_slice(b"FFF");
        let outcome_vector = vec![
            Instruction::Copy(Copy::new(3)),
            Instruction::Remove(remove_instruction),
            Instruction::Add(add_instruction_one),
            Instruction::Copy(Copy::new(3)),
            Instruction::Reference(Reference::new(1)),
            Instruction::Add(add_instruction_two),
            Instruction::Add(add_instruction_three),
            ];
        assert_eq!(delta.instructions, outcome_vector);
    }

    #[test]
    fn bench() {
        let delta = DeltaPatch::encode(b"AAABBBAAABBB", b"AAAXXXAAADDDFFF");
        println!("{}", delta.to_bytes().iter().map(|char| *char as char).collect::<String>());
    }

}
