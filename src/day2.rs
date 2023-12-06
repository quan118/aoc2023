use regex::Regex;
use std::fs;
use std::io;

pub fn solve() {
  println!("Part 1: {}", part1("inputs/day2/part1").unwrap());
  println!("Part 2: {}", part2("inputs/day2/part1").unwrap());
}

fn part1(file: &str) -> io::Result<u32> {
  let input = fs::read_to_string(file)?;
  let mut total: u32 = 0;

  for (idx, line) in input.lines().enumerate() {
    let (red, green, blue) = get_max_colors(line);
    if red <= 12 && green <= 13 && blue <= 14 {
      total += (idx + 1) as u32;
    }
  }

  Ok(total)
}

fn part2(file: &str) -> io::Result<u32> {
  let input = fs::read_to_string(file)?;
  let mut total: u32 = 0;

  for line in input.lines() {
    let (red, green, blue) = get_max_colors(line);
    total += red * green * blue;
  }

  Ok(total)
}

fn get_max_colors(line: &str) -> (u32, u32, u32) {
  let mut red: u32 = 0;
  let mut green: u32 = 0;
  let mut blue: u32 = 0;
  let re = Regex::new(r"(\d+ red|\d+ green|\d+ blue)").unwrap();
  for mat in re.find_iter(line) {
    let mut split = mat.as_str().split_whitespace();
    let number = split.next().unwrap().parse::<u32>().unwrap();
    let color = split.next().unwrap().trim();
    match color {
      "red" => red = if red < number { number } else { red },
      "green" => green = if green < number { number } else { green },
      "blue" => blue = if blue < number { number } else { blue },
      _ => panic!("Unknown color: {}", color),
    }
  }
  (red, green, blue)
}
