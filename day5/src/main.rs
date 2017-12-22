/*
 * Day 5 of Advent of Code
 * http://adventofcode.com/2017/day/5
 */

use std::io;

fn count_jumps_to_exit(offsets: &mut Vec<i32>) -> u32 {
    let mut jumps: u32 = 0;
    let mut instr_ptr: i32 = 0;
    let mut next_jump: i32;
    let offset_count = offsets.iter().count() as i32;

    while instr_ptr >= 0 && instr_ptr < offset_count {
        let i = instr_ptr as usize;
        next_jump = offsets[i];
        offsets[i] += 1;
        instr_ptr += next_jump;
        jumps += 1;
    }

    jumps
}

fn main() {
    let mut cur_line = String::new();
    let mut done = false;
    let mut offsets: Vec<i32> = Vec::new();
    let mut line_no = 0;

    while !done {
        line_no += 1;
        match io::stdin().read_line(&mut cur_line) {
            Ok(_n) => {
                let line = cur_line.trim();
                if _n == 0 || line.len() == 0 {
                    done = true;
                } else {
                    match line.parse::<i32>() {
                        Ok(offset) => offsets.push(offset),
                        Err(_e) => panic!("Could not parse value on line {}: {}", line_no, line)
                    }
                }
            },
            Err(_e) => done = true
        }
        cur_line.clear();
    }

    let jumps = count_jumps_to_exit(&mut offsets);
    println!("Jumps to exit: {}", jumps);
}
