use thiserror::Error;
use std::string::FromUtf8Error;

#[derive(Debug, Error)]
pub enum CryptoError {

    #[error("{}", format_invalid_byte(*byte, encoding))]
    InvalidByte {byte: u8, encoding: &'static str},

    #[error("{encoding} input length {len} is invalid")]
    InvalidLength { len: usize, encoding: &'static str },

    #[error("Invalid padding in {encoding} input")]
    InvalidPadding { encoding: &'static str },

    #[error("Key length {got} is invalid, expected {expected}")]
    InvalidKeyLength { expected: usize, got: usize },

    #[error("Empty key stream")]
    EmptyKeyStream,

    #[error("Invalid block padding")]
    InvalidBlockPadding,

    // Conversions
    #[error("UTF-8 decoding failed: {0}")]
    Utf8(#[from] FromUtf8Error),

}

pub type CryptoResult<T> = std::result::Result<T, CryptoError>;

fn format_invalid_byte(byte: u8, encoding: &'static str) -> String {
    if byte.is_ascii_graphic() {
        format!("Invalid character '{}' (0x{byte:02x}) in {encoding} input", byte as char)
    } else {
        format!("Invalid byte 0x{byte:02x} in {encoding} input")
    }
}