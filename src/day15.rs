use std::collections::HashMap;
use std::fs;
use std::io;

#[derive(Debug)]
struct Lens {
  label: String,
  focal_length: u32,
}

pub fn solve() {
  println!("Part 1: {}", part1("inputs/day15/part1").unwrap());
  println!("Part 2: {}", part2("inputs/day15/part1").unwrap());
}

fn part1(file: &str) -> io::Result<u64> {
  let input = fs::read_to_string(file)?;
  let line = input.lines().next().unwrap();
  let tokens = line.split(",").collect::<Vec<&str>>();
  let mut total: u64 = 0;

  for token in tokens {
    let hash = get_hash(token);
    total += get_hash(token);
  }

  Ok(total)
}

fn part2(file: &str) -> io::Result<u64> {
  let input = fs::read_to_string(file)?;
  let line = input.lines().next().unwrap();
  let tokens = line.split(",").collect::<Vec<&str>>();
  let mut total: u64 = 0;
  let mut boxes: HashMap<u32, Vec<Lens>> = HashMap::new();

  for token in tokens {
    perform_instruction(token, &mut boxes);
  }

  Ok(get_total_focusing_power(&boxes))
}

fn get_hash(s: &str) -> u64 {
  let mut current_value = 0;
  for c in s.bytes() {
    current_value += c as u64;
    current_value *= 17;
    current_value %= 256;
  }

  current_value
}

fn perform_instruction(instruction: &str, boxes: &mut HashMap<u32, Vec<Lens>>) {
  let mut op: char = '?';
  let mut op_idx = 0;
  for (idx, c) in instruction.chars().enumerate() {
    if c == '=' || c == '-' {
      op_idx = idx;
      op = c;
      break;
    }
  }

  let label = instruction[0..op_idx].to_string();
  let box_idx = get_hash(&label) as u32;
  let slots = boxes.entry(box_idx).or_insert(vec![]);

  match op {
    '=' => {
      let focal_length = instruction[op_idx + 1..].parse::<u32>().unwrap();
      let lens = Lens {
        label: label.clone(),
        focal_length,
      };
      let mut is_updated = false;
      for slot in slots.iter_mut() {
        if slot.label == label {
          slot.focal_length = focal_length;
          is_updated = true;
          break;
        }
      }

      if !is_updated {
        slots.push(lens);
      }
    }
    '-' => {
      if let Some(pos) = slots.iter().position(|slot| slot.label == label) {
        slots.remove(pos);
      }
    }
    _ => panic!("Unknown op: {}", op),
  }
}

fn get_total_focusing_power(boxes: &HashMap<u32, Vec<Lens>>) -> u64 {
  let mut total: u64 = 0;
  for (box_id, slots) in boxes {
    let mut power: u64 = 0;
    for (slot_idx, slot) in slots.iter().enumerate() {
      total += (*box_id as u64 + 1) * (slot_idx as u64 + 1) * slot.focal_length as u64;
    }
  }
  total
}
