pub struct AddInstruction {
    value: Vec<u8>,
    length: u8,
}

impl AddInstruction {
    pub fn new(value: &[u8], length: u8) -> Self {
        Self {
            value: value.to_vec(),
            length,
        }
    }

    pub fn value(&self) -> &[u8] {
        &self.value
    }

    pub fn length(&self) -> &u8 {
        &self.length
    }
}

pub struct CopyInstruction {
    index: u32,
    length: u8,
}

impl CopyInstruction {
    pub fn new(index: u32, length: u8) -> Self {
        Self { index, length }
    }

    pub fn index(&self) -> &u32 {
        &self.index
    }

    pub fn length(&self) -> &u8 {
        &self.length
    }
}

pub struct ReferenceInstruction {
    index: u16,
}

impl ReferenceInstruction {
    pub fn new(index: u16) -> Self {
        Self { index }
    }

    pub fn index(&self) -> &u16 {
        &self.index
    }
}

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

#[cfg(test)]
mod instructions_tests {
    use super::{AddInstruction, CopyInstruction, ReferenceInstruction};

    #[test]
    fn add_new() {
        let value = "Test".as_bytes();
        let add = AddInstruction::new(value, 4);
        assert_eq!(add.length(), &4);
        assert_eq!(add.value(), value);
    }

    #[test]
    fn copy_new() {
        let add = CopyInstruction::new(5, 4);
        assert_eq!(add.index(), &5);
        assert_eq!(add.length(), &4);
    }

    #[test]
    fn reference_new() {
        let add = ReferenceInstruction::new(5);
        assert_eq!(add.index(), &5);
    }
}