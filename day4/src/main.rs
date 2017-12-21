/*
 * Day 4 of Advent of Code
 * http://adventofcode.com/2017/day/4
 */

use std::io;
use std::collections::HashSet;
use std::iter::FromIterator;

fn is_valid_passphrase(s: &str) -> bool {
    let all_words: Vec<&str> = s.split(' ').collect();
    let unique_words: HashSet<&str> = HashSet::from_iter(all_words.iter().cloned());
    //println!("all_words = {:?}, unique_words = {:?}", all_words, unique_words);
    unique_words.iter().count() == all_words.iter().count()
}

fn main() {
    let mut cur_line = String::new();
    let mut done = false;
    let mut valid_count: u32 = 0;

    while !done {
        match io::stdin().read_line(&mut cur_line) {
            Ok(_n) => {
                let line = cur_line.trim();
                if _n == 0 || line.len() == 0 {
                    done = true;
                } else {
                    valid_count += if is_valid_passphrase(&line) { 1 } else { 0 };
                }
            },
            Err(_e) => done = true
        }
        cur_line.clear();
    }

    println!("Total valid passphrases: {}", valid_count);

}
