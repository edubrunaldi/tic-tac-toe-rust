
extern crate regex;

use regex::Regex;
use std::io::{stdin,stdout,Write};

pub fn get_input(msg: String) -> String {
  let mut s=String::new();
  println!("{}",msg);
  print!("> ");
  let _= stdout().flush();
  stdin().read_line(&mut s).expect("Did not enter a correct string");
  s
}

///
/// @r: regex pattern
/// @s: string to match with r
pub fn validate_regex(r: &str, s: &str) -> bool {
  Regex::new(r).unwrap().is_match(s)
}
