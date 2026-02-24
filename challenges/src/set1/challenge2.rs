use crylib::{CryptoResult, util::{from_hex, to_hex, XorCipher}};
use crate::Outcome;

const INPUT: &str = "1c0111001f010100061a024b53535009181c";
const KEYSTREAM: &str = "686974207468652062756c6c277320657965";
const EXPECTED: &str = "746865206b696420646f6e277420706c6179";

pub fn solve() -> CryptoResult<Outcome<String>> {
    let input = from_hex(INPUT)?;
    let keystream = from_hex(KEYSTREAM)?;
    let xored = input.xor(&keystream)?;
    let result = to_hex(&xored);
    Ok(Outcome::verified(result, EXPECTED.to_string()))

}