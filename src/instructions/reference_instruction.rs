struct ReferenceInstruction {
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

#[cfg(test)]
mod add_tests {
    use super::ReferenceInstruction;

    #[test]
    fn new() {
        let add = ReferenceInstruction::new(5);
        assert_eq!(add.index(), &5);
    }
}
