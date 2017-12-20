/*
 * Day 1 of Advent of Code
 * http://adventofcode.com/2017/day/1
 */

use std::io::Read;
use std::string::String;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)
        .expect("Failed to read from stdin");
    let trimmed_input = input.trim();
    let char_count = trimmed_input.chars().count();
    if char_count == 0 {
        panic!("No data from stdin");
    }
    println!("Read {} characters from stdin", char_count);

    let mut sum: i32 = 0;
    let mut last_digit: i32 = -1;
    let mut first_digit: i32 = -1;

    for c in trimmed_input.chars() {
        match c.to_digit(10) {
            Some(v) => {
                let cur_digit: i32 = v as i32;

                if last_digit >= 0 {
                    if last_digit == cur_digit {
                        sum += cur_digit;
                    }
                } else {
                    first_digit = cur_digit;
                }
                last_digit = cur_digit;
            },
            None => {}
        }
    }

    if first_digit == last_digit {
        sum += last_digit;
    }

    println!("CAPTCHA solution: {}", sum);
}
