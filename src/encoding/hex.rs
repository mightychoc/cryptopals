
use crate::error::{HexError};

fn hex_value(b: u8) -> Result<u8, HexError>{
    match b {
        b'0' ..= b'9' => Ok(b - b'0'),
        b'a' ..= b'f' => Ok(b - b'a' + 10),
        b'A' ..= b'F' => Ok(b - b'A' + 10),
        _ => Err(HexError::InvalidCharacter(b as char))
    }
}

fn hex_char(b: u8) -> char {
    match b {
        0..=9 => (b'0' + b) as char,
        10..=15 => (b'a' + (b - 10)) as char,
        _ => panic!("Byte {b} out of range [0,15]!")
    }
}

// Decodes a hexstring to a byte vector
pub fn decode(hexstr: &str) -> Result<Vec<u8>, HexError>{
    let bytes = hexstr.as_bytes();
    let len = bytes.len();

    if len % 2 != 0 {
        return Err(HexError::InvalidLength(len));
    }

    let mut out = Vec::with_capacity(len/2);

    for i in (0..len).step_by(2){
        let high = hex_value(bytes[i])?;
        let low = hex_value(bytes[i+1])?;
        out.push((high << 4) | low);
    }

    Ok(out)
}

// Encodes a byte vector to a hexstring
pub fn encode(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        out.push(hex_char(b >> 4));
        out.push(hex_char(b & 0x0f));
    }
    out
}