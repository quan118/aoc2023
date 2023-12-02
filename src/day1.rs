use std::fs;
use std::io;

pub fn solve() {
    println!("Part 1: {}", part1("inputs/day1/part1").unwrap());
    //println!("Part 2: {}", part2("inputs/day1/part2"))
}

fn part1(file: &str) -> io::Result<u32> {
    let input = fs::read_to_string(file)?;
    let mut total: u32 = 0;

    for line in input.lines() {
        let mut first_number: u32 = 0;
        let mut last_number: u32 = 0;

        for b in line.bytes() {
            if b.is_ascii_digit() {
                first_number = (b - b'0') as u32;
                break;
            }
        }

        for b in line.bytes().rev() {
            if b.is_ascii_digit() {
                last_number = (b - b'0') as u32;
                break;
            }
        }
        total += first_number * 10 + last_number;
    }

    Ok(total)
}



