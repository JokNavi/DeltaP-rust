struct AddInstruction {
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

#[cfg(test)]
mod add_tests {
    use super::AddInstruction;

    #[test]
    fn new() {
        let value = "Test".as_bytes();
        let add = AddInstruction::new(value, 4);
        assert_eq!(add.length(), &4);
        assert_eq!(add.value(), value);
    }
}
