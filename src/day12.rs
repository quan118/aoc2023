use std::collections::HashMap;
use std::fs;
use std::io;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Record {
  condition: Vec<char>,
  numbers: Vec<u64>,
}

pub fn solve() {
  println!("Part 1: {}", part1("inputs/day12/part1").unwrap());
  println!("Part 2: {}", part2("inputs/day12/part1").unwrap());
}

fn part1(file: &str) -> io::Result<u64> {
  let input = fs::read_to_string(file)?;

  let mut total: u64 = 0;
  for line in input.lines() {
    let parts = line.split_whitespace().collect::<Vec<&str>>();
    let condition = parts[0].chars().collect::<Vec<char>>();
    let numbers = parts[1]
      .split(",")
      .map(|s| s.parse::<u64>().unwrap())
      .collect::<Vec<u64>>();
    let mut memory: HashMap<Record, u64> = HashMap::new();
    let condition = normalize_condition(&condition);
    total += count_arrangement(condition, &numbers, &mut memory, 0, "");
  }

  Ok(total)
}

fn part2(file: &str) -> io::Result<u64> {
  let input = fs::read_to_string(file)?;

  let mut total: u64 = 0;
  for line in input.lines() {
    let parts = line.split_whitespace().collect::<Vec<&str>>();
    let condition = parts[0].chars().collect::<Vec<char>>();
    let condition = unfold_condition(&condition);
    let numbers = parts[1]
      .split(",")
      .map(|s| s.parse::<u64>().unwrap())
      .collect::<Vec<u64>>();
    let numbers = unfold_numbers(&numbers);
    let mut memory: HashMap<Record, u64> = HashMap::new();
    let condition = normalize_condition(&condition);
    total += count_arrangement(condition, &numbers, &mut memory, 0, "");
  }

  Ok(total)
}

fn unfold_condition(condition: &Vec<char>) -> Vec<char> {
  let mut new_condition = condition.clone();
  new_condition.push('?');
  for _ in 0..3 {
    new_condition.extend(condition.clone());
    new_condition.push('?');
  }
  new_condition.extend(condition.clone());
  new_condition
}

fn unfold_numbers(numbers: &Vec<u64>) -> Vec<u64> {
  let mut new_numbers = numbers.clone();
  for _ in 0..4 {
    new_numbers.extend(numbers.clone());
  }
  new_numbers
}

fn count_arrangement<'a, 'b>(
  condition: &'a [char],
  numbers: &'b [u64],
  memory: &mut HashMap<Record, u64>,
  recursive_level: u8,
  tag: &str,
) -> u64 {
  // base cases
  if condition.len() < numbers.len() {
    return 0;
  }

  if condition.len() == 0 && numbers.len() == 0 {
    return 1;
  }

  if numbers.len() == 0 {
    for &c in condition {
      if c == '#' {
        return 0;
      }
    }
    return 1;
  }

  // check in memory
  let record = Record {
    condition: condition.to_vec(),
    numbers: numbers.to_vec(),
  };
  if memory.contains_key(&record) {
    return *memory.get(&record).unwrap();
  }

  let number = numbers[0];
  let mut total = 0;
  let mut cont_cnt = 0;

  let mut i = 0;
  let mut is_break = false;
  while i < condition.len() {
    if cont_cnt > 0 {
      // counting
      if condition[i] == '#' {
        cont_cnt += 1;
        if cont_cnt > number {
          is_break = true;
          break;
        }
        i += 1
      } else if condition[i] == '?' {
        if cont_cnt == number {
          let new_condition = normalize_condition(&condition[i + 1..]);
          total += count_arrangement(
            &new_condition,
            &numbers[1..],
            memory,
            recursive_level + 1,
            "1",
          );
          is_break = true;
          break;
        } else {
          // cont_cnt < number -> continue to count
          cont_cnt += 1;
          i += 1;
        }
      } else {
        if cont_cnt == number {
          let new_condition = normalize_condition(&condition[i + 1..]);
          total += count_arrangement(
            &new_condition,
            &numbers[1..],
            memory,
            recursive_level + 1,
            "2",
          );
        }
        is_break = true;
        break;
      }
    } else {
      if condition[i] == '#' {
        cont_cnt += 1;
        i += 1;
      } else if condition[i] == '?' {
        // case 1: replace with '.'
        let new_condition = normalize_condition(&condition[i + 1..]);
        total += count_arrangement(&new_condition, &numbers, memory, recursive_level + 1, "3");

        // case 2: replace with '#'
        let mut new_condition = condition[i..].to_vec();
        new_condition[0] = '#';
        total += count_arrangement(&new_condition, &numbers, memory, recursive_level + 1, "4");
        is_break = true;
        break;
      } else {
        i += 1;
      }
    }
  }

  if !is_break && cont_cnt == number && numbers.len() == 1 {
    total += 1;
  }

  memory.insert(record, total);
  total
}

fn normalize_condition(condition: &[char]) -> &[char] {
  if condition.len() == 0 {
    return condition;
  }

  let mut head = 0;
  while head < condition.len() && condition[head] == '.' {
    head += 1;
  }

  let mut tail: i32 = condition.len() as i32 - 1;
  while tail > head as i32 && condition[tail as usize] == '.' {
    tail -= 1;
  }
  tail = tail.max(head as i32);
  &condition[head..tail as usize + 1]
}
