#[allow(non_camel_case_types)]
mod data;
pub use data::*;
use super::PNG;
use crate::error::{Error, Result};
use op::ternary;

pub static CHUNK_IHDR: [u8; 4] = [73, 72, 68, 82];
pub static CHUNK_cHRM: [u8; 4] = [99, 72, 82, 77];
// pub static CHUNK_gAMA: [u8; 4] = [103, 65, 77, 65];
// pub static CHUNK_sBIT: [u8; 4] = [115, 66, 73, 84];
pub static CHUNK_PLTE: [u8; 4] = [80, 76, 84, 69];
// pub static CHUNK_bKGD: [u8; 4] = [98, 75, 71, 68];
// pub static CHUNK_hIST: [u8; 4] = [104, 73, 83, 84];
// pub static CHUNK_tRNS: [u8; 4] = [116, 82, 78, 83];
// pub static CHUNK_oFFs: [u8; 4] = [111, 70, 70, 115];
// pub static CHUNK_pHYs: [u8; 4] = [112, 72, 89, 115];
// pub static CHUNK_sCAL: [u8; 4] = [115, 67, 65, 76];
pub static CHUNK_IDAT: [u8; 4] = [73, 68, 65, 84];
// pub static CHUNK_tIME: [u8; 4] = [116, 73, 77, 69];
// pub static CHUNK_tEXt: [u8; 4] = [116, 69, 88, 116];
// pub static CHUNK_zTXt: [u8; 4] = [122, 84, 88, 116];
// pub static CHUNK_fRAc: [u8; 4] = [102, 82, 65, 99];
// pub static CHUNK_gIFg: [u8; 4] = [103, 73, 70, 103];
// pub static CHUNK_gIFt: [u8; 4] = [103, 73, 70, 116];
// pub static CHUNK_gIFx: [u8; 4] = [103, 73, 70, 120];
pub static CHUNK_IEND: [u8; 4] = [73, 69, 78, 68];

macro_rules! check_length {
    ($len:expr, $arg:expr) => {
        if $len <= $arg {
            return Err(Error::InvalidLength);
        }
    };
}
macro_rules! check_crc {
    ($chunk:expr) => {
        let chunk_crc = u32::from_be_bytes($chunk.crc);
        if $crate::crc::crc32($chunk.type_code, &$chunk.data) != chunk_crc {
            return Err(Error::InvalidCRC);
        }
    };
}
pub trait FromBytes: Sized {
    /// # Safety
    unsafe fn from_bytes_unchecked(bytes: &[u8]) -> Self;
    fn from_bytes(bytes: &[u8]) -> Result<Self>;
}
pub type ChunkBytes = Chunk<Vec<u8>>;
#[derive(Debug)]
pub struct Chunk<T> {
    pub len: u32,
    pub type_code: [u8; 4],
    pub data: T,
    pub crc: [u8; 4],
}

impl<T> Chunk<T> {

}


impl ChunkBytes {
    

    /// # Safety
    pub unsafe fn from_bytes_unchecked(bytes: &[u8]) -> Vec<Self> {
        let mut chunks = Vec::new();
        let mut index = ternary!(bytes.starts_with(&PNG) => 8; 0);

        loop {
            let len = u32::from_be_bytes([
                bytes[index],
                bytes[index + 1],
                bytes[index + 2],
                bytes[index + 3],
            ]) as usize;
            let type_code = [
                bytes[index + 4],
                bytes[index + 5],
                bytes[index + 6],
                bytes[index + 7],
            ];
            let data = bytes[index + 8..index + len + 8].to_vec();
            let crc = [
                bytes[index + len + 8],
                bytes[index + len + 9],
                bytes[index + len + 10],
                bytes[index + len + 11],
            ];
            let chunk = Self {
                len: len as u32,
                type_code,
                data,
                crc,
            };

            chunks.push(chunk);
            index += len + 12;
            if type_code == CHUNK_IEND {
                break;
            }
        }
        chunks
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Vec<Self>> {
        let mut chunks = Vec::new();
        let mut index = ternary!(bytes.starts_with(&PNG) => 8; 0);
        loop {
            check_length!(bytes.len(), index + 3);
            let len = u32::from_be_bytes([
                bytes[index],
                bytes[index + 1],
                bytes[index + 2],
                bytes[index + 3],
            ]) as usize;
            check_length!(bytes.len(), index + 7 + len);
            let type_code = [
                bytes[index + 4],
                bytes[index + 5],
                bytes[index + 6],
                bytes[index + 7],
            ];
            let data = bytes[index + 8..index + len + 8].to_vec();
            let crc = [
                bytes[index + len + 8],
                bytes[index + len + 9],
                bytes[index + len + 10],
                bytes[index + len + 11],
            ];
            let chunk = Self {
                len: len as u32,
                type_code,
                data,
                crc,
            };
            check_crc!(&chunk);
            chunks.push(chunk);
            index += len + 12;
            if type_code == CHUNK_IEND {
                break;
            } else if index >= bytes.len() - 1 {
                return Err(Error::InvalidData("Missing IEND chunk".into()));
            }
        }
        if chunks[0].type_code != CHUNK_IHDR {
            return Err(Error::InvalidData(
                "IHDR chunk is not first or missing".into(),
            ));
        }
        Ok(chunks)
    }
}

impl Chunk<Vec<u8>> {
    pub fn try_convert<T: FromBytes>(&self) -> Result<Chunk<T>> {
        Ok(Chunk {
            data: T::from_bytes(&self.data)?,
            len: self.len,
            type_code: self.type_code,
            crc: self.crc,
        })
    }
    /// # Safety
    pub unsafe fn convert_unchecked<T: FromBytes>(&self) -> Chunk<T> {
        Chunk {
            data: T::from_bytes_unchecked(&self.data),
            len: self.len,
            type_code: self.type_code,
            crc: self.crc,
        }
    }
}
