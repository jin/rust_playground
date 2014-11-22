// Extracted from Yehuda Katz's GoGaRuCo 2014 Rust talk

// Load the macro_rules! macro for creating macros
// This is a feature gate.
#![feature(macro_rules)]

extern crate regex;
use regex::Regex;

macro_rules! regex(
    ($inp:expr) => (Regex::new($inp));
)

trait IsBlank {
    fn is_blank(&self) -> bool;
}

// Similar to open-classing String in Ruby, we can do implement a
// trait for the String type.
impl IsBlank for String {
    fn is_blank(&self) -> bool {
        let re = match regex!(r"\A[[:space:]]*\z") {
            Ok(re) => re,
            Err(err) => panic!("{}", err),
        };

        // fn as_slice(&'a self) -> &'a str
        re.is_match(self.as_slice())
    }
}

fn main() {
    let blank_str = "";
    let not_blank_str = "foo";
    println!("{}", blank_str.is_blank());
    println!("{}", not_blank_str.is_blank());
}
