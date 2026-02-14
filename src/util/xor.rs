use crate::error::{StreamCipherError, CryptoError};

pub fn encrypt(input: Vec<u8>, stream: Vec<u8>) -> Result<Vec<u8>, CryptoError> {

    if input.len() != stream.len() {
        return Err(StreamCipherError::LengthMismatch(input.len(), stream.len()).into());
    }

    let mut out = Vec::with_capacity(input.len());

    for (b_in, b_st) in input.iter().zip(stream.iter()) {
        out.push(b_in ^ b_st);
    }

    Ok(out)
}