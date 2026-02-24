mod outcome;

use outcome::Outcome;

mod set1;

use std::fmt;
use colored::Colorize;
use crylib::error::CryptoResult;

macro_rules! run_set {
    ($set_module: ident, $display_name:expr, [ $( $id: ident : $title:expr), * $(,)?]) => {
       println!("{}", format!("\n=== {} ===\n", $display_name).bold());
        $(
            run($title, $set_module::$id);
        )*
        println!()
    };
}

fn run<T, F>(name: &str, f:F)
where 
    T : fmt::Display + PartialEq,
    F: FnOnce() -> CryptoResult<outcome::Outcome<T>>
{
    print!("  {}: ", name.bold());
    match f() {
        Ok(outcome) => println!("{outcome}\n"),
        Err(e) => println!("{} {e}\n", "Error:".red())
    }
}

fn main() {
    run_set!(set1, "Set 1", [
        c1: "Hex to Base64",
        c2: "Fixed XOR"
    ]);
    // println!("\n{}\n", "=== Set 1 ===".bold());
    // run("C1: Hex to Base64", set1::c1);
    // run( "C2: Fixed XOR", set1::c2);
}
