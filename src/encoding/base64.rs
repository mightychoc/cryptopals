use crate::error::{Base64Error, CryptoError};

fn byte_to_b64(b: u8) -> Result<char, Base64Error> {
    match b {
        0..=25 => Ok((b'A' + b) as char),
        26..=51 => Ok((b'a' + (b - 26)) as char),
        52..=61 => Ok((b'0' + (b - 52)) as char),
        62 => Ok('+'),
        63 => Ok('/'),
        _ => Err(Base64Error::OutOfBounds(b)),
    }
}

pub fn encode(input: &[u8]) -> Result<String, Base64Error> {
    let mut out = String::with_capacity((input.len() + 2) / 3 * 4);
    let mut i = 0;

    while i + 3 <= input.len() {
        let bytes = &input[i..i + 3];

        let first = bytes[0] >> 2;
        let second = ((bytes[0] & 0x03) << 4) | (bytes[1] >> 4);
        let third = ((bytes[1] & 0x0f) << 2) | (bytes[2] >> 6);
        let fourth = bytes[2] & 0x3f;

        out.push(byte_to_b64(first)?);
        out.push(byte_to_b64(second)?);
        out.push(byte_to_b64(third)?);
        out.push(byte_to_b64(fourth)?);
        i += 3
    }

    let rem = input.len() - i;
    if rem == 1 {
        let b = input[i];
        let first = b >> 2;
        let second = (b & 0x03) << 4;

        out.push(byte_to_b64(first)?);
        out.push(byte_to_b64(second)?);
        out.push('=');
        out.push('=');
    } else if rem == 2 {
        let b1 = input[i];
        let b2 = input[i + 1];

        let first = b1 >> 2;
        let second = ((b1 & 0x03) << 4) | (b2 >> 4);
        let third = (b2 & 0x0f) << 2;

        out.push(byte_to_b64(first)?);
        out.push(byte_to_b64(second)?);
        out.push(byte_to_b64(third)?);
        out.push('=');
    }

    Ok(out)
}

fn b64_to_byte(b: u8) -> Result<u8, Base64Error> {
    match b {
        b'A'..=b'Z' => Ok(b - b'A'),
        b'a'..=b'z' => Ok(b - b'a' + 26),
        b'0'..=b'9' => Ok(b - b'0' + 52),
        b'+' => Ok(62),
        b'/' => Ok(63),
        _ => Err(Base64Error::InvalidCharacter(b as char)),
    }
}

pub fn decode(input: &str) -> Result<Vec<u8>, CryptoError> {

    if input.is_empty() {
        return Ok(Vec::new());
    }

    let in_len = input.len();

    if in_len % 4 != 0 {
        return Err(Base64Error::InvalidLength(in_len).into());
    }

    let trimmed = input.trim_end_matches('=');
    let padding_len = in_len - trimmed.len();

    if padding_len >= 3 {
        // There can be a maximum of two '=' at the end
        // for valid Base64 strings
        return Err(Base64Error::InvalidPadding.into());
    }

    let bytes = trimmed.as_bytes();

    let mut out = Vec::with_capacity((in_len / 4) * 3);

    let mut i = 0;

    while i + 4 <= bytes.len() {
        let first = b64_to_byte(bytes[i])?;
        let second = b64_to_byte(bytes[i+1])?;
        let third = b64_to_byte(bytes[i+2])?;
        let fourth = b64_to_byte(bytes[i+3])?;

        out.push((first << 2) | (second >> 4));
        out.push(((second & 0x0f) << 4) | (third >> 2));
        out.push(((third & 0x03) << 6) | fourth);
        i += 4;
    }

    let rem = bytes.len() - i;
    if rem == 3 {
        let first = b64_to_byte(bytes[i])?;
        let second = b64_to_byte(bytes[i+1])?;
        let third = b64_to_byte(bytes[i+2])?;

        // The last two bits should be zero as they are part of the padding.
        // We thus enforce canonical padding by throwing an error otherwise.
        if (third & 0x03) != 0 {
            return Err(Base64Error::InvalidPadding.into());
        }

        out.push((first << 2) | (second >> 4));
        out.push(((second & 0x0f) << 4) | (third >> 2));

    } else if rem == 2 {
        let first = b64_to_byte(bytes[i])?;
        let second = b64_to_byte(bytes[i+1])?;

        // The last four bits should be zero as they are part of the padding.
        if (second & 0x0f) != 0 {
            return Err(Base64Error::InvalidPadding.into())
        }

        out.push((first << 2) | (second >> 4));

    } else if rem == 1 {
        // This can never happen for valid Base64 as the minimum length for forming 
        // a valid character is 8 bit (rem == 1 means we have 6 bit left at the end).
        // Hence, when this case is triggered, this means we have an invalid Base64
        // string with e.g. three '=' at the end, which was cut, corrupted...
        return Err(Base64Error::InvalidPadding.into())
    }

    Ok(out)
}

#[cfg(test)]
mod tests {

    use super::*;

    fn test_base64_encode(input: &str, expected: &str) {
        let result = encode(input.as_bytes()).expect("Encoding should not fail");
        assert_eq!(result, expected, "Failed encoding '{}'", input);
    }

    fn test_base64_decode(input: &str, expected: &str) {
        let result = decode(input).expect("Decoding should not fail");
        assert_eq!(result, expected.as_bytes(), "Failed decoding '{}'", input);
    }

    #[test]
    fn base64_rfc_test_vectors_encode() {
        // RFC 4648 standard test vectors
        test_base64_encode("", "");
        test_base64_encode("f", "Zg==");
        test_base64_encode("fo", "Zm8=");
        test_base64_encode("foo", "Zm9v");
        test_base64_encode("foob", "Zm9vYg==");
        test_base64_encode("fooba", "Zm9vYmE=");
        test_base64_encode("foobar", "Zm9vYmFy");
    }

    #[test]
    fn base64_rfc_test_vectors_decode() {
        // RFC 4648 standard test vectors
        test_base64_decode("", "");
        test_base64_decode("Zg==", "f");
        test_base64_decode("Zm8=", "fo");
        test_base64_decode("Zm9v", "foo");
        test_base64_decode("Zm9vYg==", "foob");
        test_base64_decode("Zm9vYmE=", "fooba");
        test_base64_decode("Zm9vYmFy", "foobar");
    }

    #[test]
    fn base64_test_binary_data_encode() {
        // Test with non-ASCII/null bytes
        let data = [0, 0, 0, 255, 255, 255];
        let out = encode(&data).unwrap();
        assert_eq!(out, "AAAA////");
    }

    #[test]
    fn base64_test_binary_data_decode() {
        let input = "AAAA////";
        let out = decode(&input).unwrap();
        assert_eq!(out, [0, 0, 0, 255, 255, 255]);
    }

    #[test]
    fn base64_test_long_string_encode() {
        let input = "The quick brown fox jumps over the lazy dog";
        let expected = "VGhlIHF1aWNrIGJyb3duIGZveCBqdW1wcyBvdmVyIHRoZSBsYXp5IGRvZw==";
        test_base64_encode(input, expected);
    }

    #[test]
    fn base64_test_long_string_decode() {
        let input = "VGhlIHF1aWNrIGJyb3duIGZveCBqdW1wcyBvdmVyIHRoZSBsYXp5IGRvZw==";
        let expected = "The quick brown fox jumps over the lazy dog";
        test_base64_decode(input, expected);
    }

    #[test]
    fn base64_test_all_6_bit_values_encode() {
        // Test a sequence that triggers every single Base64 character
        // This ensures the byte_to_b64 mapping table is correct
        for i in 0..64 {
            // Check if our mapping function itself is correct
            let c = byte_to_b64(i as u8).unwrap();
            let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
            let test_char = alphabet.chars().nth(i).unwrap();
            assert_eq!(
                c, test_char,
                "Missmatch for encoding character '{}'",
                test_char
            );
        }
    }

    #[test]
    fn base64_test_all_6_bit_values_decode() {
        let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        for (i, c) in alphabet.bytes().enumerate() {
            let b = b64_to_byte(c).expect("Character '{}' should be in alphabet");
            assert_eq!(b, i as u8, "Missmatch for decoding character '{}'", c);
        }
    }
}
