use std::fmt::Display;
use crate::errors::Result;
use crate::patch::errors::UPatchError;
use memmap2::Mmap;
use memmap2::MmapMut;

#[derive(Debug, Clone)]
pub struct Bytes(Vec<u8>);

impl Bytes {
    pub fn new<U: Into<Vec<u8>>>(data: U) -> Self {
        Self(data.into())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }

    pub fn to_hex(&self) -> String {
        let mut s = String::new();
        for b in &self.0 {
            s.push_str(&format!("{:02X}", b));
        }
        s
    }

    pub fn try_from_hex(hex: String) -> Result<Self> {
        Ok(Self::try_from(Hex(hex))?)
    }

    pub fn to_utf8(&self) -> Result<String> {
        Ok(String::from_utf8(self.0.clone()).map_err(|_| UPatchError::InvalidUtf8Data)?)
    }
}

impl TryFrom<Hex> for Bytes {
    type Error = UPatchError;

    fn try_from(value: Hex) -> core::result::Result<Bytes, UPatchError> {
        if value.len() % 2 != 0 {
            return Err(UPatchError::InvalidHexData);
        }
        let mut bytes = Vec::new();
        for i in (0..value.len()).step_by(2) {
            let byte = u8::from_str_radix(&value.0[i..i + 2], 16)
                .map_err(|_| UPatchError::InvalidHexData)?;
            bytes.push(byte);
        }
        Ok(Bytes(bytes))
    }
}

#[derive(Debug, Clone)]
pub struct Hex(String);

impl Hex {
    pub fn new(data: String) -> Self {
        Self(data)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn as_str_mut(&mut self) -> &mut str {
        &mut self.0
    }

    pub fn try_to_bytes(self) -> Result<Bytes> {
        Ok(self.try_into()?)
    }

    pub fn from_bytes(bytes: Bytes) -> Self {
        Self(bytes.to_hex())
    }
}

impl From<Bytes> for Hex {
    fn from(value: Bytes) -> Self {
        Self(value.to_hex())
    }
}

impl Display for Bytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

#[derive(Debug, Clone)]
pub enum PatchType {
    String(String),
    Data(Vec<u8>),
}

impl From<String> for PatchType {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for PatchType {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<Vec<u8>> for PatchType {
    fn from(value: Vec<u8>) -> Self {
        Self::Data(value)
    }
}

#[derive(Debug)]
pub enum PatchDataType {
    Mmap(Mmap),
    MmapMut(MmapMut),
    Data(Vec<u8>),
}
