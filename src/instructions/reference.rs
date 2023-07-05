use crate::instructions::instruction::ToBytes;

pub type ReferenceSize = u16;


#[derive(Debug, Default, PartialEq)]
pub struct Reference {
    index: ReferenceSize,
}

impl Reference {
    pub fn new(index: ReferenceSize) -> Self {
        Self { index }
    }

    pub fn index(&self) -> ReferenceSize {
        self.index
    }
}

impl ToBytes for Reference {

    fn to_bytes(&self) -> Vec<u8> { 
        let mut bytes = vec![b'&'];
        bytes.extend_from_slice(self.index.to_be_bytes().as_slice());
        bytes
    }
}