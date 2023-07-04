use crate::instructions::instruction::ToBytes;

pub type ReferenceSize = u16;

pub struct Reference {
    index: ReferenceSize,
}

impl Reference {
    pub fn index(&self) -> ReferenceSize {
        self.index
    }
}

impl ToBytes for Reference {
    const BYTE_SIGN: u8 = '&' as u8;

    fn to_bytes(&self) -> Vec<u8> { 
        let mut bytes = vec![Self::BYTE_SIGN];
        bytes.extend_from_slice(&self.index.to_be_bytes());
        bytes
    }
}