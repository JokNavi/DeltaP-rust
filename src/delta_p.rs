use crate::instructions::{
    add::{Add, AddSize, self},
    copy::Copy,
    instruction::{self, Instruction, PushByte, ChunkLength},
    reference::{Reference, ReferenceSize},
    remove::{Remove, RemoveSize},
};

#[derive(Debug)]
pub struct DeltaP {
    instructions: Vec<Instruction>,
}

impl DeltaP {
    fn new(source_bytes: &[u8], target_bytes: &[u8]) -> Self {
        let mut instructions: Vec<Instruction> = vec![];
        let mut target_bytes_iter = target_bytes.iter().zip(source_bytes);

        if target_bytes.len() > ReferenceSize::MAX.into() {
            unimplemented!()
        }

        

        while let Some((target_byte, source_byte)) = target_bytes_iter.next() {
            match instructions.last_mut() {
                Some(Instruction::Add(_))
                | Some(Instruction::Remove(_))
                | Some(Instruction::Reference(_))
                | None
                    if target_byte == source_byte =>
                {
                    let mut copy_instruction = Copy::default();
                    copy_instruction.push(*target_byte);
                    instructions.push(Instruction::Copy(copy_instruction));
                }
                Some(Instruction::Add(add_instruction)) => {
                    add_instruction.push(*target_byte);
                    let remove_instruction_index = instructions.len() - 2;
                    let remove_instruction = instructions.get_mut(remove_instruction_index).unwrap().remove_mut().unwrap();
                    remove_instruction.push(*source_byte);
                }
                Some(Instruction::Remove(_)) => unreachable!(),
                Some(Instruction::Copy(copy_instruction)) if target_byte == source_byte => {
                    copy_instruction.push(*target_byte);
                }
                Some(Instruction::Copy(_)) | Some(Instruction::Reference(_)) | None => {
                    let mut remove_instruction = Remove::default();
                    remove_instruction.push(*source_byte);
                    instructions.push(Instruction::Remove(remove_instruction));

                    let mut add_instruction = Add::default();
                    add_instruction.push(*source_byte);
                    instructions.push(Instruction::Add(add_instruction));
                }
            };
        }

        if target_bytes.len() > source_bytes.len() {
            let mut missing_bytes = target_bytes.get(source_bytes.len()..).unwrap();
            if let Some(add_instruction) = instructions.last_mut().and_then(|instruction| instruction.add_mut()) {
                let slice = missing_bytes.get(..(AddSize::MAX - add_instruction.length()) as usize).unwrap_or(missing_bytes);
                add_instruction.push_slice(slice);
                missing_bytes = missing_bytes.get((AddSize::MAX - add_instruction.length()-1) as usize..).unwrap_or(missing_bytes);
            }    
            for chunk in missing_bytes.chunks(AddSize::MAX.into()) {
                let mut add_instruction = Add::default();
                add_instruction.push_slice(chunk);
                instructions.push(Instruction::Add(add_instruction));
            }
        }
        if target_bytes.len() < source_bytes.len() {
            let mut missing_bytes = source_bytes.get(source_bytes.len()..).unwrap();
            if let Some(remove_instruction) = instructions.last_mut().and_then(|instruction| instruction.remove_mut()) {
                let slice = missing_bytes.get(..(RemoveSize::MAX - remove_instruction.length()) as usize).unwrap_or(missing_bytes);
                remove_instruction.push_slice(slice);
                missing_bytes = missing_bytes.get((RemoveSize::MAX - remove_instruction.length()-1) as usize..).unwrap_or(missing_bytes);
            }                     
            for chunk in missing_bytes.chunks(AddSize::MAX.into()) {
                let mut remove_instruction = Remove::default();
                remove_instruction.push_slice(chunk);
                instructions.push(Instruction::Remove(remove_instruction));
            }
        }

        let try_create_refence = |instruction: Instruction| -> Option<Reference> {
            if !matches!(instruction, Instruction::Add(_) | Instruction::Remove(_)) {panic!();}
            if instructions.contains(&instruction) {
                return Some(Reference::new(instructions
                    .iter()
                    .position(|other_instruction| other_instruction == &instruction)
                    .unwrap() as u16));
            }
            None
        };


        for instruction in &mut instructions {
            match instruction {
                Instruction::Add(add_instruction) => todo!(),
                Instruction::Remove(remove_instruction) => todo!(),
                Instruction::Copy(_) => todo!(),
                Instruction::Reference(_) => todo!(),
            }
        }
        Self { instructions }
    }
}

#[cfg(test)]
mod delta_p_tests {
    use crate::{
        delta_p::{DeltaP, Instruction, Reference},
        instructions::{add::Add, copy::Copy, instruction::PushByte, remove::Remove},
    };

    #[test]
    fn new() {
        let delta = DeltaP::new(b"AAABBBAAABBB", b"AAAXXXAAADDDFFF");
        let mut remove_instruction = Remove::with_capacity(3);
        remove_instruction.push_slice(b"BBB");
        let mut add_instruction_one = Add::with_capacity(3);
        add_instruction_one.push_slice(b"XXX");
        let mut add_instruction_two = Add::with_capacity(6);
        add_instruction_two.push_slice(b"DDDFFF");
        let outcome_vector = vec![
            Instruction::Copy(Copy::new(3)),
            Instruction::Remove(remove_instruction),
            Instruction::Add(add_instruction_one),
            Instruction::Copy(Copy::new(3)),
            Instruction::Reference(Reference::new(2)),
            Instruction::Add(add_instruction_two),
        ];
        assert_eq!(delta.instructions, outcome_vector);
    }

    #[test]
    fn new_2() {
        let delta = DeltaP::new(b"AAABBBAAABBB", b"AAAXXXAAADDDFFF");
        let mut remove_instruction = Remove::with_capacity(3);
        remove_instruction.push_slice(b"BBB");
        let mut add_instruction_one = Add::with_capacity(3);
        add_instruction_one.push_slice(b"XXX");
        let mut add_instruction_two = Add::with_capacity(6);
        add_instruction_two.push_slice(b"DDDFFF");
        let outcome_vector = vec![
            Instruction::Copy(Copy::new(3)),
            Instruction::Remove(remove_instruction),
            Instruction::Add(add_instruction_one),
            Instruction::Reference(Reference::new(2)),
            Instruction::Add(add_instruction_two),
            ];
        assert_eq!(delta.instructions, outcome_vector);
    }

    #[test]
    fn new_3() {
        let delta = DeltaP::new(b"AAABBBAAABBB", b"AAAXXXAAADDDFFF");
        let mut remove_instruction = Remove::with_capacity(3);
        remove_instruction.push_slice(b"BBB");
        let mut add_instruction_one = Add::with_capacity(3);
        add_instruction_one.push_slice(b"XXX");
        let mut add_instruction_two = Add::with_capacity(6);
        add_instruction_two.push_slice(b"DDDFFF");
        let outcome_vector = vec![
            Instruction::Copy(Copy::new(3)),
            Instruction::Remove(remove_instruction),
            Instruction::Add(add_instruction_one),
            Instruction::Reference(Reference::new(2)),
            Instruction::Add(add_instruction_two),
            ];

        let mut remove_instruction = Remove::with_capacity(3);
        remove_instruction.push_slice(b"BBB");
        dbg!(outcome_vector.contains(&Instruction::Remove(remove_instruction)));
        assert_eq!(delta.instructions, outcome_vector);
    }
}
