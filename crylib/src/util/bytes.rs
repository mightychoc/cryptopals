use crate::error::{CryptoError, CryptoResult};

pub fn xor(input: &[u8], stream: &[u8]) -> CryptoResult<Vec<u8>> {
    if stream.is_empty() {
        return Err(CryptoError::EmptyKeyStream);
    }

    Ok(input
        .iter()
        .zip(stream.iter().cycle())
        .map(|(b_in, b_st)| b_in ^ b_st)
        .collect())
}

// Decodes a hexstring to a byte vector
pub fn from_hex(hexstr: &str) -> CryptoResult<Vec<u8>> {
    let bytes = hexstr.as_bytes();
    let len = bytes.len();

    if len % 2 != 0 {
        return Err(CryptoError::InvalidLength {
            len: len,
            encoding: "hex",
        });
    }

    let mut out = Vec::with_capacity(len / 2);

    for i in (0..len).step_by(2) {
        let high = hex_char_to_byte(bytes[i])?;
        let low = hex_char_to_byte(bytes[i + 1])?;
        out.push((high << 4) | low);
    }

    Ok(out)
}

// Encodes a byte vector to a hexstring
pub fn to_hex(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        out.push(hex_byte_to_char(b >> 4));
        out.push(hex_byte_to_char(b & 0x0f));
    }
    out
}

fn hex_char_to_byte(b: u8) -> CryptoResult<u8> {
    match b {
        b'0'..=b'9' => Ok(b - b'0'),
        b'a'..=b'f' => Ok(b - b'a' + 10),
        b'A'..=b'F' => Ok(b - b'A' + 10),
        _ => Err(CryptoError::InvalidByte {
            byte: b,
            encoding: "hex",
        }),
    }
}

fn hex_byte_to_char(b: u8) -> char {
    match b {
        0..=9 => (b'0' + b) as char,
        10..=15 => (b'a' + (b - 10)) as char,
        _ => panic!("Byte {b} out of range [0,15]!"),
    }
}

pub fn to_base64(input: &[u8]) -> CryptoResult<String> {
    let mut out = String::with_capacity((input.len() + 2) / 3 * 4);
    let mut i = 0;

    while i + 3 <= input.len() {
        let bytes = &input[i..i + 3];

        let first = bytes[0] >> 2;
        let second = ((bytes[0] & 0x03) << 4) | (bytes[1] >> 4);
        let third = ((bytes[1] & 0x0f) << 2) | (bytes[2] >> 6);
        let fourth = bytes[2] & 0x3f;

        out.push(byte_to_base64(first)?);
        out.push(byte_to_base64(second)?);
        out.push(byte_to_base64(third)?);
        out.push(byte_to_base64(fourth)?);
        i += 3
    }

    let rem = input.len() - i;
    if rem == 1 {
        let b = input[i];
        let first = b >> 2;
        let second = (b & 0x03) << 4;

        out.push(byte_to_base64(first)?);
        out.push(byte_to_base64(second)?);
        out.push('=');
        out.push('=');
    } else if rem == 2 {
        let b1 = input[i];
        let b2 = input[i + 1];

        let first = b1 >> 2;
        let second = ((b1 & 0x03) << 4) | (b2 >> 4);
        let third = (b2 & 0x0f) << 2;

        out.push(byte_to_base64(first)?);
        out.push(byte_to_base64(second)?);
        out.push(byte_to_base64(third)?);
        out.push('=');
    }

    Ok(out)
}

pub fn from_base64(input: &str) -> CryptoResult<Vec<u8>> {
    if input.is_empty() {
        return Ok(Vec::new());
    }

    let in_len = input.len();

    if in_len % 4 != 0 {
        return Err(CryptoError::InvalidLength {
            len: in_len,
            encoding: "Base64",
        });
    }

    let trimmed = input.trim_end_matches('=');
    let padding_len = in_len - trimmed.len();

    if padding_len >= 3 {
        // There can be a maximum of two '=' at the end
        // for valid Base64 strings
        return Err(CryptoError::InvalidPadding { encoding: "Base64" });
    }

    let bytes = trimmed.as_bytes();

    let mut out = Vec::with_capacity((in_len / 4) * 3);

    let mut i = 0;

    while i + 4 <= bytes.len() {
        let first = base64_to_byte(bytes[i])?;
        let second = base64_to_byte(bytes[i + 1])?;
        let third = base64_to_byte(bytes[i + 2])?;
        let fourth = base64_to_byte(bytes[i + 3])?;

        out.push((first << 2) | (second >> 4));
        out.push(((second & 0x0f) << 4) | (third >> 2));
        out.push(((third & 0x03) << 6) | fourth);
        i += 4;
    }

    let rem = bytes.len() - i;
    if rem == 3 {
        let first = base64_to_byte(bytes[i])?;
        let second = base64_to_byte(bytes[i + 1])?;
        let third = base64_to_byte(bytes[i + 2])?;

        // The last two bits should be zero as they are part of the padding.
        // We thus enforce canonical padding by throwing an error otherwise.
        if (third & 0x03) != 0 {
            return Err(CryptoError::InvalidPadding { encoding: "Base64" });
        }

        out.push((first << 2) | (second >> 4));
        out.push(((second & 0x0f) << 4) | (third >> 2));
    } else if rem == 2 {
        let first = base64_to_byte(bytes[i])?;
        let second = base64_to_byte(bytes[i + 1])?;

        // The last four bits should be zero as they are part of the padding.
        if (second & 0x0f) != 0 {
            return Err(CryptoError::InvalidPadding { encoding: "Base64" });
        }

        out.push((first << 2) | (second >> 4));
    } else if rem == 1 {
        // This can never happen for valid Base64 as the minimum length for forming
        // a valid character is 8 bit (rem == 1 means we have 6 bit left at the end).
        // Hence, when this case is triggered, this means we have an invalid Base64
        // string with e.g. three '=' at the end, which was cut, corrupted...
        return Err(CryptoError::InvalidPadding { encoding: "Base64" });
    }

    Ok(out)
}

fn byte_to_base64(b: u8) -> CryptoResult<char> {
    match b {
        0..=25 => Ok((b'A' + b) as char),
        26..=51 => Ok((b'a' + (b - 26)) as char),
        52..=61 => Ok((b'0' + (b - 52)) as char),
        62 => Ok('+'),
        63 => Ok('/'),
        _ => Err(CryptoError::InvalidByte {
            byte: b,
            encoding: "Base64",
        }),
    }
}

fn base64_to_byte(b: u8) -> CryptoResult<u8> {
    match b {
        b'A'..=b'Z' => Ok(b - b'A'),
        b'a'..=b'z' => Ok(b - b'a' + 26),
        b'0'..=b'9' => Ok(b - b'0' + 52),
        b'+' => Ok(62),
        b'/' => Ok(63),
        _ => Err(CryptoError::InvalidByte {
            byte: b,
            encoding: "Base64",
        }),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[cfg(test)]
    mod xor {
        use super::*;

        // --- Error cases ---

        #[test]
        fn xor_empty_stream_is_error() {
            let err = xor(&[0x01, 0x02], &[]).unwrap_err();
            assert!(
                matches!(err, CryptoError::EmptyKeyStream),
                "Expected EmptyKeyStream, got: {err}"
            );
        }

        #[test]
        fn xor_empty_stream_empty_input_is_error() {
            // Empty stream is always invalid, even with empty input,
            // because we can't know the caller's intent
            let err = xor(&[], &[]).unwrap_err();
            assert!(matches!(err, CryptoError::EmptyKeyStream));
        }

        // --- Basic correctness ---

        #[test]
        fn xor_empty_input_returns_empty() {
            let result = xor(&[], &[0xff]).unwrap();
            assert!(result.is_empty());
        }

        #[test]
        fn xor_with_zero_is_identity() {
            let input = b"hello world";
            let stream = vec![0x00];
            let result = xor(input, &stream).unwrap();
            assert_eq!(result, input);
        }

        #[test]
        fn xor_with_all_ones_is_bitwise_not() {
            let input = vec![0b10101010, 0b11001100, 0b11110000];
            let result = xor(&input, &[0xff]).unwrap();
            assert_eq!(result, vec![0b01010101, 0b00110011, 0b00001111]);
        }

        #[test]
        fn xor_is_own_inverse() {
            let input = b"secret message";
            let key = b"key";
            let encrypted = xor(input, key).unwrap();
            let decrypted = xor(&encrypted, key).unwrap();
            assert_eq!(decrypted, input);
        }

        #[test]
        fn xor_single_byte_key() {
            let input = vec![0x1b, 0x37, 0x37, 0x33, 0x31];
            let result = xor(&input, &[0x58]).unwrap();
            assert_eq!(result, vec![0x43, 0x6f, 0x6f, 0x6b, 0x69]);
        }

        #[test]
        fn xor_equal_length_key_no_cycling() {
            // When key length == input length, cycling has no effect
            let a = vec![0b11001100, 0b10101010, 0b11110000];
            let b = vec![0b01010101, 0b01100110, 0b00001111];
            let result = xor(&a, &b).unwrap();
            assert_eq!(result, vec![0b10011001, 0b11001100, 0b11111111]);
        }

        // --- Cycling behaviour ---

        #[test]
        fn xor_key_cycles_correctly() {
            // Manually verify that the key wraps at the right positions
            let input = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
            let key = vec![0x01, 0x02, 0x03];
            let result = xor(&input, &key).unwrap();
            // XOR with 0x00 is identity on the key, so we just see the key repeated
            assert_eq!(result, vec![0x01, 0x02, 0x03, 0x01, 0x02, 0x03]);
        }

        #[test]
        fn xor_key_longer_than_input_truncates() {
            // Only the first input.len() bytes of the key are used
            let input = vec![0x00, 0x00];
            let key = vec![0xaa, 0xbb, 0xcc, 0xdd];
            let result = xor(&input, &key).unwrap();
            assert_eq!(result, vec![0xaa, 0xbb]);
        }

        #[test]
        fn xor_key_length_one_is_single_byte_xor() {
            let input = vec![0x00, 0x01, 0x02, 0x03];
            let result = xor(&input, &[0x0f]).unwrap();
            assert_eq!(result, vec![0x0f, 0x0e, 0x0d, 0x0c]);
        }

        // --- Properties ---

        #[test]
        fn xor_commutativity() {
            // a ^ b == b ^ a
            let a = vec![0xde, 0xad];
            let b = vec![0xbe, 0xef];
            assert_eq!(xor(&a, &b).unwrap(), xor(&b, &a).unwrap());
        }

        #[test]
        fn xor_output_length_equals_input_length() {
            for input_len in [0, 1, 2, 15, 16, 17, 100] {
                let input = vec![0xaa; input_len];
                let key = vec![0x01, 0x02, 0x03];
                let result = xor(&input, &key).unwrap();
                assert_eq!(
                    result.len(),
                    input_len,
                    "Output length should match input length for input_len={input_len}"
                );
            }
        }
    }

    mod hex {
        use super::*;

        // --- Helpers ---

        fn test_hex_encode(input: &[u8], expected: &str) {
            let result = to_hex(input);
            assert_eq!(result, expected, "Failed encoding {:?}", input);
        }

        fn test_hex_decode(input: &str, expected: &[u8]) {
            let result = from_hex(input).expect("Decoding should not fail");
            assert_eq!(result, expected, "Failed decoding '{}'", input);
        }

        // --- Encoding ---

        #[test]
        fn hex_encode_empty() {
            test_hex_encode(&[], "");
        }

        #[test]
        fn hex_encode_single_bytes() {
            test_hex_encode(&[0x00], "00");
            test_hex_encode(&[0x0f], "0f");
            test_hex_encode(&[0x10], "10");
            test_hex_encode(&[0xff], "ff");
        }

        #[test]
        fn hex_encode_boundary_values() {
            test_hex_encode(&[0x00, 0x09, 0x0a, 0x0f], "00090a0f");
            test_hex_encode(&[0x90, 0xa0, 0xf0], "90a0f0");
        }

        #[test]
        fn hex_encode_all_byte_values() {
            let all_bytes: Vec<u8> = (0u8..=255).collect();
            let encoded = to_hex(&all_bytes);
            assert_eq!(encoded.len(), 512);
            assert!(encoded.starts_with("00"));
            assert!(encoded.ends_with("ff"));
        }

        #[test]
        fn hex_encode_known_string() {
            test_hex_encode(b"foobar", "666f6f626172");
        }

        #[test]
        fn hex_encode_is_lowercase() {
            let result = to_hex(&[0xab, 0xcd, 0xef]);
            assert_eq!(result, "abcdef");
            assert!(result.chars().all(|c| !c.is_uppercase()));
        }

        #[test]
        fn hex_encode_all_nibble_values() {
            let expected = "0123456789abcdef";
            for (i, expected_char) in expected.chars().enumerate() {
                // Encode a byte where both nibbles are i, e.g. i=10 -> 0xaa
                let byte = ((i as u8) << 4) | (i as u8);
                let encoded = to_hex(&[byte]);
                let actual_char = encoded.chars().next().unwrap();
                assert_eq!(
                    actual_char, expected_char,
                    "Nibble {i} should encode to '{expected_char}', got '{actual_char}'"
                );
            }
        }

        // --- Decoding ---

        #[test]
        fn hex_decode_empty() {
            test_hex_decode("", &[]);
        }

        #[test]
        fn hex_decode_single_bytes() {
            test_hex_decode("00", &[0x00]);
            test_hex_decode("0f", &[0x0f]);
            test_hex_decode("10", &[0x10]);
            test_hex_decode("ff", &[0xff]);
        }

        #[test]
        fn hex_decode_uppercase() {
            test_hex_decode("0F", &[0x0f]);
            test_hex_decode("DEADBEEF", &[0xde, 0xad, 0xbe, 0xef]);
            test_hex_decode("AbCdEf", &[0xab, 0xcd, 0xef]);
        }

        #[test]
        fn hex_decode_known_string() {
            test_hex_decode("666f6f626172", b"foobar");
        }

        // --- Error cases ---

        #[test]
        fn hex_decode_odd_length_is_error() {
            let err = from_hex("abc").unwrap_err();
            assert!(
                matches!(
                    err,
                    CryptoError::InvalidLength {
                        len: 3,
                        encoding: "hex"
                    }
                ),
                "Expected InvalidLength, got: {err}"
            );
        }

        #[test]
        fn hex_decode_invalid_character_is_error() {
            for bad in ["gg", "zz", "0x", "0 ", "--"] {
                let err = from_hex(bad).unwrap_err();
                assert!(
                    matches!(
                        err,
                        CryptoError::InvalidByte {
                            encoding: "hex",
                            ..
                        }
                    ),
                    "Expected InvalidCharacter for input '{bad}', got: {err}"
                );
            }
        }

        #[test]
        fn hex_decode_invalid_character_reported_correctly() {
            // Check the *specific* bad character is captured in the error
            let err = from_hex("0g").unwrap_err();
            assert!(
                matches!(
                    err,
                    CryptoError::InvalidByte {
                        byte: b'g',
                        encoding: "hex"
                    }
                ),
                "Expected character 'g' in error, got: {err}"
            );
        }

        // --- Back-and-Forth ---

        #[test]
        fn hex_roundtrip_all_bytes() {
            let original: Vec<u8> = (0u8..=255).collect();
            let encoded = to_hex(&original);
            let decoded = from_hex(&encoded).unwrap();
            assert_eq!(decoded, original);
        }

        #[test]
        fn hex_roundtrip_random_ish() {
            // Deterministic "random-ish" data without pulling in a rand crate
            let data: Vec<u8> = (0u8..=255)
                .map(|i| i.wrapping_mul(179).wrapping_add(13))
                .collect();
            assert_eq!(from_hex(&to_hex(&data)).unwrap(), data);
        }
    }

    mod base64 {
        use super::*;

        // --- Helpers ---

        fn test_base64_encode(input: &str, expected: &str) {
            let result = to_base64(input.as_bytes()).expect("Encoding should not fail");
            assert_eq!(result, expected, "Failed encoding '{}'", input);
        }

        fn test_base64_decode(input: &str, expected: &str) {
            let result = from_base64(input).expect("Decoding should not fail");
            assert_eq!(result, expected.as_bytes(), "Failed decoding '{}'", input);
        }

        // --- Encoding ---

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
        fn base64_test_binary_data_encode() {
            // Test with non-ASCII/null bytes
            let data = [0, 0, 0, 255, 255, 255];
            let out = to_base64(&data).unwrap();
            assert_eq!(out, "AAAA////");
        }

        #[test]
        fn base64_test_long_string_encode() {
            let input = "The quick brown fox jumps over the lazy dog";
            let expected = "VGhlIHF1aWNrIGJyb3duIGZveCBqdW1wcyBvdmVyIHRoZSBsYXp5IGRvZw==";
            test_base64_encode(input, expected);
        }

        #[test]
        fn base64_test_all_6_bit_values_encode() {
            // Test a sequence that triggers every single Base64 character
            // This ensures the byte_to_b64 mapping table is correct
            for i in 0..64 {
                // Check if our mapping function itself is correct
                let c = byte_to_base64(i as u8).unwrap();
                let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
                let test_char = alphabet.chars().nth(i).unwrap();
                assert_eq!(
                    c, test_char,
                    "Missmatch for encoding character '{}'",
                    test_char
                );
            }
        }

        // --- Decoding ---

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
        fn base64_test_binary_data_decode() {
            let input = "AAAA////";
            let out = from_base64(&input).unwrap();
            assert_eq!(out, [0, 0, 0, 255, 255, 255]);
        }

        #[test]
        fn base64_test_long_string_decode() {
            let input = "VGhlIHF1aWNrIGJyb3duIGZveCBqdW1wcyBvdmVyIHRoZSBsYXp5IGRvZw==";
            let expected = "The quick brown fox jumps over the lazy dog";
            test_base64_decode(input, expected);
        }

        #[test]
        fn base64_test_all_6_bit_values_decode() {
            let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
            for (i, c) in alphabet.bytes().enumerate() {
                let b = base64_to_byte(c).expect("Character '{}' should be in alphabet");
                assert_eq!(b, i as u8, "Missmatch for decoding character '{}'", c);
            }
        }

        // --- Error cases ---

        #[test]
        fn base64_decode_odd_length_is_error() {
            for bad_len in ["a", "ab", "abc", "abcde"] {
                let err = from_base64(bad_len).unwrap_err();
                assert!(
                    matches!(
                        err,
                        CryptoError::InvalidLength {
                            encoding: "Base64",
                            ..
                        }
                    ),
                    "Expected InvalidLength for '{bad_len}', got: {err}"
                );
            }
        }

        #[test]
        fn base64_decode_invalid_character_is_error() {
            for bad in ["Zg=!", "A@AA", "AAA\x00", "AA-="] {
                let err = from_base64(bad).unwrap_err();
                assert!(
                    matches!(
                        err,
                        CryptoError::InvalidByte {
                            encoding: "Base64",
                            ..
                        }
                    ),
                    "Expected InvalidCharacter for '{bad}', got: {err}"
                );
            }
        }

        #[test]
        fn base64_decode_too_many_padding_chars_is_error() {
            // === is never valid
            for bad in ["a===", "===="] {
                let err = from_base64(bad).unwrap_err();
                assert!(
                    matches!(err, CryptoError::InvalidPadding { .. }),
                    "Expected InvalidPadding for '{bad}', got: {err}"
                );
            }
        }

        #[test]
        fn base64_decode_noncanonical_padding_is_error() {
            // 1-byte padding (==):
            // 'Z' = 25 = 0b011001, low 4 bits = 0b1001 != 0 -> invalid
            let err = from_base64("Zh==").unwrap_err();
            assert!(
                matches!(err, CryptoError::InvalidPadding { .. }),
                "Expected InvalidPadding for 'Zh==', got: {err}"
            );

            // 2-byte padding (=):
            // 'n' = 39 = 0b100111, low 2 bits = 11 != 0 -> invalid
            let err = from_base64("Zmn=").unwrap_err();
            assert!(
                matches!(err, CryptoError::InvalidPadding { .. }),
                "Expected InvalidPadding for 'Zmn=', got: {err}"
            );

            // Sanity check
            assert!(from_base64("Zg==").is_ok()); // 'g'=32=0b100000
            assert!(from_base64("Zm8=").is_ok()); // '8'=60=0b111100
        }

        #[test]
        fn base64_decode_padding_in_wrong_position_is_error() {
            // Padding must only appear at the end
            for bad in ["=AAA", "A=AA", "AA=A"] {
                let err = from_base64(bad).unwrap_err();
                assert!(
                    matches!(err, CryptoError::InvalidByte { .. }),
                    "Expected error for mid-string padding '{bad}', got: {err}"
                );
            }
        }

        // --- Back-and-Forth ---

        #[test]
        fn base64_roundtrip_all_byte_values() {
            let original: Vec<u8> = (0u8..=255).collect();
            let encoded = to_base64(&original).unwrap();
            let decoded = from_base64(&encoded).unwrap();
            assert_eq!(decoded, original);
        }

        #[test]
        fn base64_roundtrip_all_remainder_lengths() {
            // Explicitly hit rem==0, rem==1, rem==2 in the encoder
            // and the corresponding 4-char, 3-char, 2-char tails in the decoder
            for len in 0..=12 {
                let data: Vec<u8> = (0..len).map(|i| i as u8 * 17).collect();
                let encoded = to_base64(&data).unwrap();
                let decoded = from_base64(&encoded).unwrap();
                assert_eq!(decoded, data, "Back-and-forth failed for length {len}");
            }
        }
    }
}
