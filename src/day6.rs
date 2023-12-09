use std::fs;
use std::io;

pub fn solve() {
  println!("Part 1: {}", part1("inputs/day6/part1").unwrap());
  println!("Part 1: {}", part2("inputs/day6/part1").unwrap());
}

fn part1(file: &str) -> io::Result<u64> {
  let input = fs::read_to_string(file)?;
  let mut line_iter = input.lines();
  let times = line_iter
    .next()
    .unwrap()
    .split(":")
    .nth(1)
    .unwrap()
    .split_whitespace()
    .map(|s| s.parse::<u64>().unwrap())
    .collect::<Vec<u64>>();
  let distances = line_iter
    .next()
    .unwrap()
    .split(":")
    .nth(1)
    .unwrap()
    .split_whitespace()
    .map(|s| s.parse::<u64>().unwrap())
    .collect::<Vec<u64>>();

  let mut total: u64 = 1;
  for (idx, time) in times.iter().enumerate() {
    let distance = distances[idx];
    let possible_options = count_possible_options(*time, distance);
    total *= possible_options;
  }
  Ok(total)
}

fn part2(file: &str) -> io::Result<u64> {
  let input = fs::read_to_string(file)?;
  let mut line_iter = input.lines();
  let time = line_iter
    .next()
    .unwrap()
    .split(":")
    .nth(1)
    .unwrap()
    .replace(" ", "")
    .parse::<u64>()
    .unwrap();
  let distance = line_iter
    .next()
    .unwrap()
    .split(":")
    .nth(1)
    .unwrap()
    .replace(" ", "")
    .parse::<u64>()
    .unwrap();

  Ok(count_possible_options(time, distance))
}

fn count_possible_options(time: u64, distance: u64) -> u64 {
  let delta: f64 = (time * time - 4 * distance) as f64;
  let sqrt_delta = delta.sqrt();
  let x1 = (time as f64 - sqrt_delta) / 2.0;
  let x2 = (time as f64 + sqrt_delta) / 2.0;
  let x1 = if x1.ceil() == x1 { x1 + 1.0 } else { x1.ceil() };
  let x2 = if x2.floor() == x2 {
    x2 - 1.0
  } else {
    x2.floor()
  };
  x2 as u64 - x1 as u64 + 1
}
