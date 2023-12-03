use std::fs::File;
use std::io::prelude::*;

fn main () {
  let mut file = File::open("puzzle_input.txt").expect("Can't open file!");
  let mut contents = String::new();
  file.read_to_string(&mut contents)
    .expect("Oops! Can not read file...");

  let mut sum: u32 = 0;

  for line in contents.lines() {
    let mut first_digit: String = "".to_string();
    let mut last_digit: String = "".to_string();
    let mut has_first: bool = false;
    for character in line.chars() {
      if character.is_numeric() {
        last_digit = character.to_string();
        if !has_first {
          first_digit = character.to_string();
          has_first = true;
        }
      }
    }
    let num_string: String = first_digit + &last_digit;
    let num: u32 = num_string.parse().unwrap();
    sum = sum + num;
  }
  println!("{}", sum);
}