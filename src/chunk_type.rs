use std::{fmt, str::FromStr, convert::TryFrom, str};

use crate::{Error, Result, chunk::Chunk};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkType {
    bytes: [u8; 4]
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;

    fn try_from(value: [u8; 4]) -> Result<Self> {
        for byte in value {
            if !Self::is_ascii(byte) {
                return Err(ChunkTypeError::AsciiError.into());
            }
        }

        Ok(ChunkType { bytes: value })
    }
}

impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s.len() != 4 { return Err(ChunkTypeError::InvalidLength.into()) }

        let bytes: [u8; 4] = match s.as_bytes().try_into() {
            Ok(x) => {
                for byte in x {
                    if !Self::is_ascii(byte) {
                        return Err(ChunkTypeError::AsciiError.into());
                    }
                }

                x
            },
            Err(_) => return Err(ChunkTypeError::AsciiError.into())
        };

        Ok( Self { bytes })
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes = self.bytes();
        let chars = str::from_utf8(&bytes).unwrap();
        write!(f, "{}", chars)
    }
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.bytes
    }

    pub fn is_valid(&self) -> bool {
        for byte in self.bytes {
            if !Self::is_ascii(byte) {return false;}
        }

        Self::is_reserved_bit_valid(&self)
    }

    pub fn is_critical(&self) -> bool {
        (1 << 5) & self.bytes[0] == 0
    }

    pub fn is_public(&self) -> bool {
        (1 << 5) & self.bytes[1] == 0
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        (1 << 5) & self.bytes[2] == 0
    }

    pub fn is_safe_to_copy(&self) -> bool {
        (1 << 5) & self.bytes[3] != 0
    }

    fn is_ascii(byte: u8) -> bool {
        byte.is_ascii_uppercase() || byte.is_ascii_lowercase()
    }


}

#[derive(Clone)]
pub struct ChunkTypeWrapper(ChunkType);

impl FromStr for ChunkTypeWrapper {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Self(ChunkType::from_str(s).map_err(|e| e.to_string())?))
    }
}

#[derive(Debug, Clone)]
pub enum ChunkTypeError {
    InvalidLength,
    AsciiError,
}

impl fmt::Display for ChunkTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLength => write!(f, "length of string must be exactly 4 bytes"),
            Self::AsciiError => write!(f, "all bytes must be ascii a-z or A-Z")
        }
    }
}

impl std::error::Error for ChunkTypeError {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}