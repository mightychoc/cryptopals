mod outcome;

mod set1;

use std::fmt;
use crylib::error::CryptoResult;


fn run<T, F>(name: &str, f:F)
where 
    T : fmt::Display + PartialEq,
    F: FnOnce() -> CryptoResult<outcome::Outcome<T>>
{
    print!("  {name}: ");
    match f() {
        Ok(outcome) => println!("{outcome}"),
        Err(e) => println!("Error: {e}")
    }
}

fn main() {
    println!("=== Set 1 ===");
    run("C1: Hex to Base64", set1::challenge1::solve);
    run( "C2: Fixed XOR", set1::challenge2::solve);
}
