
use crate::instructions::{add::*, copy::*, instruction::*, reference::*, remove::*};

pub struct DeltaP {
    instructions: Vec<Instruction>,
}

impl DeltaP {
    fn new(source_bytes: &[u8], target_bytes: &[u8]) -> Self {
        let mut instructions: Vec<Instruction> = vec![];
        let mut target_bytes_iter =
            target_bytes.iter().zip(source_bytes);

        while let Some((target_byte, source_byte)) = target_bytes_iter.next() {
            match instructions.last_mut() {
                Some(Instruction::Add(_)) | Some(Instruction::Remove(_)) | Some(Instruction::Reference(_)) | None if target_byte == source_byte => {
                    let mut copy_instruction = Copy::default();
                    copy_instruction.push(*target_byte);
                    instructions.push(Instruction::Copy(Copy::default()));
                },
                Some(Instruction::Add(add_instruction)) => {
                    add_instruction.push(*target_byte);
                    let remove_instruction_index = instructions.len() - 2;
                    let remove_instruction = match instructions.get_mut(remove_instruction_index) {
                        Some(Instruction::Remove(remove_instruction)) => Some(remove_instruction),
                        _ => None,
                    }.unwrap();
                    remove_instruction.push(*source_byte);
                },
                Some(Instruction::Remove(_)) => unreachable!(),
                Some(Instruction::Copy(copy_instruction)) if target_byte == source_byte => {
                    copy_instruction.push(*target_byte);
                },
                Some(Instruction::Copy(_)) | Some(Instruction::Reference(_)) | None => {
                    let mut remove_instruction = Remove::default();
                    remove_instruction.push(*target_byte);
                    instructions.push(Instruction::Remove(remove_instruction));
                    let mut add_instruction = Add::default();
                    add_instruction.push(*source_byte); 
                    instructions.push(Instruction::Add(add_instruction));
                },
            };
        }
        if target_bytes.len() > source_bytes.len() {
            let mut add_instruction = Add::default();
            add_instruction.push_slice(target_bytes.get(source_bytes.len()..).unwrap());
            instructions.push(Instruction::Add(add_instruction));
        }
        if target_bytes.len() < source_bytes.len() {
            let mut remove_instruction = Remove::default();
            remove_instruction.push_slice(source_bytes.get(target_bytes.len()..).unwrap());
            instructions.push(Instruction::Remove(remove_instruction));
        }
        Self { instructions}
    }
}


#[cfg(test)]
mod delta_p_tests {
    
}