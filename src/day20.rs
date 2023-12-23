use core::panic;
use std::collections::{HashMap, VecDeque};
use std::fs;
use std::io;

#[derive(Debug, Clone)]
enum Module {
  FlipFlop(String, bool),
  Conjunction(String, HashMap<String, bool>),
  Broadcast(String),
}

pub fn solve() {
  println!("Part 1: {}", part1("inputs/day20/part1").unwrap());
  println!("Part 2: {}", part2("inputs/day20/part1").unwrap());
}

fn part1(file: &str) -> io::Result<u64> {
  let (flows, mut modules) = build_module_configuration(file);

  let mut total_low: u64 = 0;
  let mut total_high: u64 = 0;

  for _ in 0..1000 {
    let (low, high) = process(&flows, &mut modules, &mut HashMap::new(), 0);
    total_low += low;
    total_high += high;
  }

  Ok(total_low * total_high)
}

fn part2(file: &str) -> io::Result<u64> {
  let (flows, mut modules) = build_module_configuration(file);

  let rx_module: String = "rx".to_string();
  let mut before_rx = None;
  for (from, parts) in flows.iter() {
    if parts.contains(&rx_module) {
      before_rx = Some(from.clone());
      break;
    }
  }

  let before_rx = before_rx.unwrap();
  let before_rx_module = modules.get_mut(&before_rx).unwrap();
  let mut cycles: HashMap<String, u64> = HashMap::new();
  match before_rx_module {
    Module::Conjunction(_, inputs) => {
      for (name, _) in inputs.iter() {
        cycles.insert(name.clone(), 0);
      }
    }
    _ => {
      panic!("before_rx_module is not a conjunction");
    }
  }

  let mut cnt: u64 = 0;
  loop {
    cnt += 1;
    process(&flows, &mut modules, &mut cycles, cnt);

    if cycles.values().all(|v| *v > 0) {
      break;
    }
  }

  let result = lcm_array(
    cycles
      .iter()
      .map(|(_, v)| *v)
      .collect::<Vec<u64>>()
      .as_slice(),
  );
  Ok(result)
}

fn process(
  flows: &HashMap<String, Vec<String>>,
  modules: &mut HashMap<String, Module>,
  cycles: &mut HashMap<String, u64>,
  loop_cnt: u64,
) -> (u64, u64) {
  let mut low_cnt: u64 = 0;
  let mut hight_cnt: u64 = 0;
  let mut queue: VecDeque<(String, bool, String)> = VecDeque::new();

  queue.push_back(("button".to_string(), false, "broadcaster".to_string()));

  while let Some((from, pulse, label)) = queue.pop_front() {
    if cycles.contains_key(&from) && pulse && cycles.get(&from) == Some(&0) {
      cycles.insert(from.clone(), loop_cnt);
    }
    if pulse {
      hight_cnt += 1;
    } else {
      low_cnt += 1;
    }
    let mut out_pulse: Option<bool> = None;
    // update module state & out pulse
    if let Some(module) = modules.get_mut(&label) {
      match module {
        Module::FlipFlop(_, state) => {
          if !pulse {
            *state = !*state;
            out_pulse = Some(*state);
          }
        }
        Module::Conjunction(_, inputs) => {
          inputs.insert(from.clone(), pulse);

          if inputs.iter().all(|(_, b)| *b) {
            out_pulse = Some(false);
          } else {
            out_pulse = Some(true);
          }
        }
        _ => {
          if label != "broadcaster" {
            println!("untyped: label: {} pulse: {}", label, pulse);
          }
          out_pulse = Some(pulse);
        }
      }
    }

    if out_pulse == None {
      continue;
    }

    let out_pulse = out_pulse.unwrap();

    // update queue
    if let Some(parts) = flows.get(&label) {
      for part in parts.iter() {
        queue.push_back((label.clone(), out_pulse, part.to_string()));
      }
    }
  }

  (low_cnt, hight_cnt)
}

fn build_module_configuration(
  file: &str,
) -> (HashMap<String, Vec<String>>, HashMap<String, Module>) {
  let input: String = fs::read_to_string(file).unwrap();
  let mut lines: std::str::Lines = input.lines();
  let mut flows: HashMap<String, Vec<String>> = HashMap::new();
  let mut modules: HashMap<String, Module> = HashMap::new();
  while let Some(line) = lines.next() {
    let mut parts = line.split(" -> ");
    let from = parts.next().unwrap().trim();
    let to = parts.next().unwrap().trim();
    let mut label = from;
    if from == "broadcaster" {
      modules.insert(from.to_string(), Module::Broadcast(from.to_string()));
    } else {
      match &from[0..1] {
        "%" => {
          let name = from[1..].to_string();
          modules.insert(name.clone(), Module::FlipFlop(name, false));
        }
        "&" => {
          let name = from[1..].to_string();
          modules.insert(name.clone(), Module::Conjunction(name, HashMap::new()));
        }
        _ => {
          panic!("Unknown module type: {}", from);
        }
      }
      label = &from[1..];
    }

    let to_parts: Vec<String> = to.split(",").map(|s| s.trim().to_string()).collect();
    flows.insert(label.to_string(), to_parts);

    // update Conjunction parts
    for (from, parts) in flows.iter() {
      for part in parts.iter() {
        if let Some(module) = modules.get_mut(part) {
          match module {
            Module::Conjunction(_, inputs) => {
              inputs.insert(from.clone(), false);
            }
            _ => {}
          }
        }
      }
    }
  }

  (flows, modules)
}

fn gcd(a: u64, b: u64) -> u64 {
  if b == 0 {
    return a;
  }
  gcd(b, a % b)
}

fn lcm(a: u64, b: u64) -> u64 {
  a * b / gcd(a, b)
}

fn lcm_array(numbers: &[u64]) -> u64 {
  numbers.iter().fold(1, |acc, &num| lcm(acc, num))
}
