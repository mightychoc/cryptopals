use crate::encoding::hex;
use crate::encoding::base64;
use crate::error::CryptoError;

#[allow(dead_code)]
fn solve(input: &str, stream: &str) -> Result<String, CryptoError> {
    let input_bytes = hex::decode(input)?;
    let stream_bytes = hex::decode(stream)?;
    let encrypted = xor::encrypt(input_bytes, stream_bytes)?;

    Ok(encrypted)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_set1_challenge1() {
        let input = "1c0111001f010100061a024b53535009181c";
        let stream = "686974207468652062756c6c277320657965";
        let expected = "746865206b696420646f6e277420706c6179";
        let result = solve(input, stream).unwrap();
        assert_eq!(result, expected);
    }
}