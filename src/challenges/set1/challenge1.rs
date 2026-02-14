use crate::encoding::hex;
use crate::encoding::base64;
use crate::error::CryptoError;

#[allow(dead_code)]
fn solve(input: &str) -> Result<String, CryptoError> {
    let bytes = hex::decode(input)?;
    let encoded = base64::encode(&bytes)?;
    Ok(encoded)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_set1_challenge1() {
        let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        let result = solve(input).unwrap();
        assert_eq!(result, expected);
    }
}