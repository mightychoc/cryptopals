use crylib::{CryptoResult, util::{from_hex, to_base64}};
use crate::Outcome;

const INPUT: &str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
const EXPECTED: &str = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

pub fn solve() -> CryptoResult<Outcome<String>> {
    let bytes = from_hex(INPUT)?;
    let encoded = to_base64(&bytes)?;
    Ok(Outcome::verified(encoded, EXPECTED.to_string()))
}