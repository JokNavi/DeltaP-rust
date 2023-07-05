use super::{add::Add, reference::Reference, remove::Remove, copy::Copy};
use std::{iter::{Peekable, Enumerate, Zip}, slice::Iter};

pub trait ChunkLength {
    type Output;
    fn length(&self) -> Self::Output;
}

pub trait PushByte {
    fn push(&mut self, byte: u8) -> ();
    fn push_slice(&mut self, slice: &[u8]);
}

pub trait ByteChunk {
    fn bytes(&self) -> &[u8];
}

pub trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Add(Add),
    Remove(Remove),
    Copy(Copy),
    Reference(Reference),
}

impl Instruction {
    pub fn add(&self) -> Option<&Add> {
        match self {
            Instruction::Add(add_instruction) => Some(add_instruction),
            _ => None
        }
    }

    pub fn remove(&self) -> Option<&Remove> {
        match self {
            Instruction::Remove(remove_instruction) => Some(remove_instruction),
            _ => None
        }
    }

    pub fn copy(&self) -> Option<&Copy> {
        match self {
            Instruction::Copy(copy_instruction) => Some(copy_instruction),
            _ => None
        }
    }

    pub fn reference(&self) -> Option<&Reference> {
        match self {
            Instruction::Reference(reference_instruction) => Some(reference_instruction),
            _ => None
        }
    }

    pub fn add_mut(&mut self) -> Option<&mut Add> {
        match self {
            Instruction::Add(add_instruction) => Some(add_instruction),
            _ => None
        }
    }

    pub fn remove_mut(&mut self) -> Option<&mut Remove> {
        match self {
            Instruction::Remove(remove_instruction) => Some(remove_instruction),
            _ => None
        }
    }

    pub fn copy_mut(&mut self) -> Option<&mut Copy> {
        match self {
            Instruction::Copy(copy_instruction) => Some(copy_instruction),
            _ => None
        }
    }

    pub fn reference_mut(&mut self) -> Option<&mut Reference> {
        match self {
            Instruction::Reference(reference_instruction) => Some(reference_instruction),
            _ => None
        }
    }

    pub fn try_into_reference(self, collection: &Vec<Instruction>) -> Instruction {
        if collection.contains(&self) {
            let index = collection.iter().position(|instruction| instruction == &self).unwrap();
            return Instruction::Reference(Reference::new(index.try_into().unwrap()));
        }
        self
    }
}

impl ToBytes for Instruction {
    fn to_bytes(&self) -> Vec<u8> {
        match self {
            Instruction::Add(add) => add.to_bytes(),
            Instruction::Remove(remove) => remove.to_bytes(),
            Instruction::Copy(copy) => copy.to_bytes(),
            Instruction::Reference(reference) => reference.to_bytes(),
        }
    }
}


