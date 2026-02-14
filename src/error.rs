use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum CryptoError{
    Hex(HexError),
    Base64(Base64Error)
}

impl fmt::Display for CryptoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Hex(err) => write!(f, "Hex Error: {}", err),
            Self::Base64(err) => write!(f, "Base64 Error: {}", err)
        }
    }
}

// Hex-String <-> Bytes encoding and decoding

#[derive(Debug)]
pub enum HexError{
    InvalidLength(usize),
    InvalidCharacter(char)
}

impl Error for HexError {}

impl fmt::Display for HexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLength(len) => write!(f, "Length of hex string to decode must be divisible by two, length is {}", len),
            Self::InvalidCharacter(c) => write!(f, "Invalid hex character: {}", c),

        }
    }
}

impl From<HexError> for CryptoError {
    fn from(err: HexError) -> Self {
        Self::Hex(err)
    }
}

// Base64 encoding and decoding

#[derive(Debug)]
pub enum Base64Error {
    OutOfBounds(u8),
    InvalidCharacter(char),
    InvalidLength(usize),
    InvalidPadding
}

impl Error for Base64Error {}

impl fmt::Display for Base64Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OutOfBounds(byte) => write!(f, "Base64 characters are only defined for values in range [0,63], received {}", byte),
            Self::InvalidCharacter(c) => write!(f, "'{}' is not a valid Base64 character", c),
            Self::InvalidLength(len) => write!(f, "Length of string to decode must be divisible by four, length is {}", len),
            Self::InvalidPadding => write!(f, "Non-zero trailing bit(s) detected. Non-canonical paddings are not accepted by the decoder.")
        }
    }
}

impl From<Base64Error> for CryptoError {
    fn from(err: Base64Error) -> Self {
        Self::Base64(err)
    }
}

impl Error for CryptoError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Hex(err) => Some(err),
            Self::Base64(err) => Some(err)
        }
    }
}

