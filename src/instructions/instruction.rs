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
    const BYTE_SIGN: u8;
    fn to_bytes(&self) -> Vec<u8>;
}

pub enum Instruction {
    Add(Add),
    Remove(Remove),
    Copy(Copy),
    Reference(Reference),
}

impl Instruction {
    pub fn is_add(&self) -> bool {
        matches!(self, Instruction::Add(_))
    }

    pub fn is_remove(&self) -> bool {
        matches!(self, Instruction::Remove(_))
    }

    pub fn is_copy(&self) -> bool {
        matches!(self, Instruction::Copy(_))
    }

    pub fn is_reference(&self) -> bool {
        matches!(self, Instruction::Reference(_))
    }
}

impl PartialEq for Instruction {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Add(l0), Self::Add(r0)) => l0 == r0,
            (Self::Remove(l0), Self::Remove(r0)) => l0 == r0,
            _ => false,
        }
    }
}


