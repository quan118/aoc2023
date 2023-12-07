use std::cmp;
use std::fs;
use std::io;

#[derive(Clone, Debug)]
struct MappingRule {
  start: u64,
  end: u64,
  range: u64,
}

#[derive(Clone, Debug)]
struct MappingGuide {
  rules: Vec<MappingRule>,
}

impl MappingGuide {
  fn get_output(&self, input: u64) -> u64 {
    let mut output: u64 = input;
    for rule in &self.rules {
      if rule.start <= input && input <= rule.start + rule.range {
        output = rule.end + (input - rule.start);
        break;
      }
    }
    output
  }
}

pub fn solve() {
  println!("Part 1: {}", part1("inputs/day5/part1").unwrap());
}

fn part1(file: &str) -> io::Result<u64> {
  let input = fs::read_to_string(file)?;
  let mut guides: Vec<MappingGuide> = Vec::new();
  let mut seeds: Vec<u64> = Vec::new();
  let mut guide = MappingGuide { rules: Vec::new() };

  for line in input.lines() {
    if line.starts_with("seeds:") {
      let mut split = line.split(":");
      split.next(); // skip the `seeds` part
      let seeds_str = split.next().unwrap();
      seeds = seeds_str
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    } else if line.ends_with("map:") {
      if guide.rules.len() > 0 {
        guides.push(guide.clone());
      }
      guide = MappingGuide { rules: Vec::new() };
    } else if line.len() > 0 {
      let mut split = line.split_whitespace();
      let end = split.next().unwrap().parse::<u64>().unwrap();
      let start = split.next().unwrap().parse::<u64>().unwrap();
      let range = split.next().unwrap().parse::<u64>().unwrap();
      guide.rules.push(MappingRule { start, end, range });
    }
  }
  guides.push(guide.clone());

  // println!("seeds: {:?}", seeds);
  // println!("guides: {:?}", guides);

  let mut lowest_output = u64::MAX;
  for seed in seeds {
    let mut output = seed;
    for guide in &guides {
      output = guide.get_output(output);
    }
    lowest_output = cmp::min(lowest_output, output);
  }

  // let mut output = seeds[3];
  // for guide in &guides {
  //   output = guide.get_output(output);
  //   println!("output: {}", output);
  // }
  // lowest_output = cmp::min(lowest_output, output);

  Ok(lowest_output)
}
