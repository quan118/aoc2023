use std::collections::HashSet;
use std::fs;
use std::io;

pub fn solve() {
  println!("Part 1: {}", part1("inputs/day4/part1").unwrap())
}

fn part1(file: &str) -> io::Result<u32> {
  let input = fs::read_to_string(file)?;
  let mut total: u32 = 0;
  for line in input.lines() {
    let (winning_numbers, numbers_i_have) = get_numbers(line);
    // println!("winning numbers: {:?}", winning_numbers);
    // println!("numbers i have: {:?}", numbers_i_have);
    let my_winning_numbers = winning_numbers.intersection(&numbers_i_have).count() as u32;
    if my_winning_numbers > 0 {
      total += (2 as u32).pow(my_winning_numbers-1);
      
    }
    // println!("my winning numbers: {:?}", my_winning_numbers);
  }

  Ok(total)
}

fn get_numbers(line: &str) -> (HashSet<u32>, HashSet<u32>) {
  let mut split = line.split(":");
  split.next(); // skip the `Card #` part
  let mut split = split.next().unwrap().split("|");
  let winning_numbers_line = split.next().unwrap();
  let numbers_i_have_line = split.next().unwrap();

  // get winning numbers
  let mut winning_numbers = HashSet::new();
  let mut split = winning_numbers_line.split_whitespace();
  while let Some(number) = split.next() {
    winning_numbers.insert(number.parse::<u32>().unwrap());
  }

  // get numbers i have
  let mut numbers_i_have = HashSet::new();
  let mut split = numbers_i_have_line.split_whitespace();
  while let Some(number) = split.next() {
    numbers_i_have.insert(number.parse::<u32>().unwrap());
  }

  (winning_numbers, numbers_i_have)
}