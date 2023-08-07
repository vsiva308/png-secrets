use std::fmt::{Display, self};

use crate::{Error, Result, chunk::Chunk};

#[derive(Debug, Clone)]
pub struct Png {
    chunks: Vec<Chunk>
}

impl TryFrom<&[u8]> for Png {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self> {
        todo!()
    }
}

impl Display for Png {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Png {
    pub const STANDARD_HEADER: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];
    
    pub fn from_chunks(chunks: Vec<Chunk>) -> Png {
        todo!()
    }

    pub fn append_chunk(&mut self, chunk: Chunk) {
        self.chunks.push(chunk)
    }
    
    pub fn remove_chunk(&mut self, chunk_type: &str) -> Result<Chunk> {
        let index = self.chunks.iter().position(|x| x.chunk_type().bytes() == chunk_type.as_bytes());
        if index.is_none() {
            return Err(PngError::ChunkNotPresent.into());
        }
        Ok(self.chunks.remove(index.unwrap()))
    }

    fn header(&self) -> &[u8; 8] {
        &Self::STANDARD_HEADER
    }

    fn chunks(&self) -> &[Chunk] {
        &self.chunks
    }

    fn chunk_by_type(&self, chunk_type: &str) -> Option<&Chunk> {
        self.chunks.iter().find(|&x| x.chunk_type().bytes() == chunk_type.as_bytes())
    }

    fn as_bytes(&self) -> Vec<u8> {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub enum PngError {
    ChunkNotPresent,
}

impl fmt::Display for PngError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ChunkNotPresent => write!(f, "chunk type does not exist in file"),
        }
    }
}

impl std::error::Error for PngError {}