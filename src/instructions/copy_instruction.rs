struct CopyInstruction {
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

#[cfg(test)]
mod add_tests {
    use super::CopyInstruction;

    #[test]
    fn new() {
        let add = CopyInstruction::new(5, 4);
        assert_eq!(add.index(), &5);
        assert_eq!(add.length(), &4);
    }
}
