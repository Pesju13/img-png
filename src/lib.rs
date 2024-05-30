#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
pub mod chunk;
pub mod crc;
pub mod error;
pub static PNG: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];

pub fn png_size(buf: &[u8]) -> Result<(u32, u32), error::Error> {
    if !buf.starts_with(&PNG) || buf.len() < 24 {
        return Err(error::Error::InvalidData("Invalid PNG!".to_owned()));
    }
    let bytes = &buf[16..24];
    let width = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
    let height = u32::from_be_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
    Ok((width, height))
}

