use std::fs;
use std::io;

pub fn solve() {
  println!("Part 1: {}", part1("inputs/day1/part1").unwrap());
  println!("Part 2: {}", part2("inputs/day1/part1").unwrap());
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

fn part2(file: &str) -> io::Result<u32> {
  let input = fs::read_to_string(file)?;
  const NUMBERS_IN_TEXT: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
  ];
  let mut total: u32 = 0;

  for line in input.lines() {
    let mut first_number: u32 = 0;
    'find_first_number: for (idx, b) in line.bytes().enumerate() {
      if b.is_ascii_digit() {
        first_number = (b - b'0') as u32;
        break;
      } else {
        for (idx2, &text) in NUMBERS_IN_TEXT.iter().enumerate() {
          if idx + text.len() <= line.len() && text == &line[idx..(idx + text.len())] {
            first_number = (idx2 + 1) as u32;
            break 'find_first_number;
          }
        }
      }
    }

    let mut last_number: u32 = 0;
    'find_last_number: for (idx, b) in line.bytes().rev().enumerate() {
      let rev_idx = line.len() - idx - 1;
      if b.is_ascii_digit() {
        last_number = (b - b'0') as u32;
        break;
      } else {
        for (idx2, &text) in NUMBERS_IN_TEXT.iter().enumerate() {
          if rev_idx + text.len() <= line.len() && text == &line[rev_idx..(rev_idx + text.len())] {
            last_number = (idx2 + 1) as u32;
            break 'find_last_number;
          }
        }
      }
    }

    total += first_number * 10 + last_number;
  }

  Ok(total)
}
