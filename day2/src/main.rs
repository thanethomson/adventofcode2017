/*
 * Day 2 of Advent of Code
 * http://adventofcode.com/2017/day/2
 */

use std::io;

fn row_to_values(row: &String) -> Vec<i32> {
    row.split('\t')
        .filter_map(|s| s.trim().parse::<i32>().ok())
        .collect()
}

fn row_checksum(row: &Vec<i32>) -> i32 {
    let max = row.iter().max();
    let min = row.iter().min();
    match (max, min) {
        (Some(max_val), Some(min_val)) => return max_val - min_val,
        _ => panic!("Missing minimum and/or maximum value in row")
    }
}

fn main() {
    let mut cur_line = String::new();
    let mut done = false;
    let mut checksum: i32 = 0;

    while !done {
        match io::stdin().read_line(&mut cur_line) {
            Ok(_n) => {
                if _n == 0 {
                    done = true;
                } else {
                    let row_values = row_to_values(&cur_line);
                    checksum += row_checksum(&row_values);
                }
            },
            Err(_e) => done = true
        }
        cur_line.clear();
    }

    println!("Calculated checksum: {}", checksum);

}
