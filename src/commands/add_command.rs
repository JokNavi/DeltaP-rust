use std::num::TryFromIntError;

struct Add {
    value: Vec<u8>,
    length: u8,
}

impl Add {

    pub fn new(value: &[u8], length: u8) -> Result<Self, TryFromIntError> {
        Ok( Self {value: value.to_vec(), length,})
    }

    pub fn value(&self) -> &[u8] {
        &self.value
    }

    pub fn length(&self) -> &u8 {
        &self.length
    }
}

#[cfg(test)]
mod add_tests{
    use super::Add;


    #[test]
    fn new() {
        let value = "Test".as_bytes();
        let add = Add::new(value, 4).unwrap();
        assert_eq!(add.length(), &4);
        assert_eq!(add.value(), value);
    }
}