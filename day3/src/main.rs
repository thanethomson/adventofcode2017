/*
 * Day 3 of Advent of Code
 * http://adventofcode.com/2017/day/3
 */

use std::io;

fn find_dist_from_mid_point(a: &u32, b: &u32, v: &u32) -> u32 {
    let mid_point = (((*b as i32) - (*a as i32)) / 2) + (*a as i32);
    ((*v as i32) - mid_point).abs() as u32
}

fn find_manhattan_dist(v: &u32) -> u32 {
    if *v < 1 {
        panic!("Invalid value for v: {}", v);
    }
    // the centre of the map
    if *v == 1 {
        return 0;
    }
    let mut r: u32 = 1;
    let mut corners: Vec<u32> = vec![1, 1, 1, 1];

    loop {
        for i in 0..4 {
            corners[i] += (2*((i as u32)+1)) + (8 * (r-1));
        }
        //println!("corners = {:?}", corners);

        let max_val = *corners.iter().max().unwrap();

        // if we've found a region for v
        if *v <= max_val {
            for i in 1..4 {
                if corners[i-1] <= *v && *v <= corners[i] {
                    //println!("Found it between {} and {}", i-1, i);
                    return r + find_dist_from_mid_point(&corners[i-1], &corners[i], v);
                }
            }
            let side_len: u32 = corners[1] - corners[0];
            let virtual_bot_right: u32 = corners[0] - side_len;
            return r + find_dist_from_mid_point(&virtual_bot_right, &corners[0], v);
        }

        r += 1;
    }
}

fn main() {
    let mut line = String::new();
    match io::stdin().read_line(&mut line) {
        Ok(_n) => {
            let trimmed = line.trim();
            match trimmed.parse::<u32>() {
                Ok(v) => println!("Manhattan distance: {}", find_manhattan_dist(&v)),
                _ => panic!("Unable to parse input line: {}", trimmed)
            }
        },
        _ => panic!("Unable to read from stdin")
    }
}
