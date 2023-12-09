use std::fs;
use std::io;

pub fn solve() {
  println!("Part 1: {}", part1("inputs/day9/part1").unwrap());
  println!("Part 2: {}", part2("inputs/day9/part1").unwrap());
}

fn part1(file: &str) -> io::Result<i64> {
  let input = fs::read_to_string(file)?;
  let mut total: i64 = 0;

  for line in input.lines() {
    let numbers: Vec<i64> = line
      .split_whitespace()
      .map(|s| s.parse::<i64>().unwrap())
      .collect();
    let mut next_number = get_next_number(&numbers);
    total += next_number;
  }

  Ok(total)
}

fn part2(file: &str) -> io::Result<i64> {
  let input = fs::read_to_string(file)?;
  let mut total: i64 = 0;

  for line in input.lines() {
    let numbers: Vec<i64> = line
      .split_whitespace()
      .map(|s| s.parse::<i64>().unwrap())
      .collect();
    let mut prev_number = get_prev_number(&numbers);
    total += prev_number;
  }

  Ok(total)
}

fn get_next_number(numbers: &Vec<i64>) -> i64 {
  let mut len = numbers.len();
  let mut diffs = numbers.clone();
  let mut is_all_zero = false;
  while !is_all_zero {
    is_all_zero = true;
    for idx in 0..len - 1 {
      let diff = diffs[idx + 1] - diffs[idx];
      if diff != 0 {
        is_all_zero = false;
      }
      diffs[idx] = diff;
    }
    len -= 1;
  }

  diffs.iter().sum()
}

fn get_prev_number(numbers: &Vec<i64>) -> i64 {
  let mut len = numbers.len();
  let mut diffs = numbers.clone();
  let mut first_numbers: Vec<i64> = vec![numbers[0]];
  let mut is_all_zero = false;
  while !is_all_zero {
    is_all_zero = true;
    for idx in 0..len - 1 {
      let diff = diffs[idx + 1] - diffs[idx];
      if diff != 0 {
        is_all_zero = false;
      }
      diffs[idx] = diff;
      if idx == 0 {
        first_numbers.push(diffs[idx]);
      }
    }
    len -= 1;
  }

  let mut prev_number: i64 = 0;
  for num in first_numbers.iter().rev() {
    prev_number = num - prev_number;
  }
  prev_number
}
