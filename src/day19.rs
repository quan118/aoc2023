use std::collections::HashMap;
use std::fs;
use std::io;

#[derive(Debug, Copy, Clone)]
struct Part {
  x: u64,
  m: u64,
  a: u64,
  s: u64,
}

impl Part {
  fn sum(self: &Self) -> u64 {
    self.x + self.m + self.a + self.s
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum Category {
  X,
  M,
  A,
  S,
}

#[derive(Debug, Copy, Clone)]
enum Op {
  GreaterThan,
  LessThan,
}

#[derive(Debug, Copy, Clone)]
struct Condition {
  category: Category,
  op: Op,
  value: u64,
}

impl Condition {
  fn from_string(s: &str) -> Option<Condition> {
    let category = match &s[0..1] {
      "x" => Category::X,
      "m" => Category::M,
      "a" => Category::A,
      "s" => Category::S,
      _ => return None,
    };
    let op = match &s[1..2] {
      ">" => Op::GreaterThan,
      "<" => Op::LessThan,
      _ => return None,
    };
    if let Some(value) = s[2..].parse::<u64>().ok() {
      return Some(Condition {
        category,
        op,
        value,
      });
    }
    None
  }
}

#[derive(Debug)]
struct Rule {
  condition: Option<Condition>,
  label: String,
}

pub fn solve() {
  println!("Part 1: {}", part1("inputs/day19/part1").unwrap());
  println!("Part 2: {}", part2("inputs/day19/part1").unwrap());
}

fn part1(file: &str) -> io::Result<u64> {
  let (workflows, parts) = build_workflows_and_parts(file);

  let mut total: u64 = 0;
  for part in parts.iter() {
    let mut current_workflow = "in".to_string();
    while current_workflow != "A" && current_workflow != "R" {
      let rules = workflows.get(&current_workflow).unwrap();
      for rule in rules.iter() {
        if let Some(condition) = rule.condition {
          let value = match condition.category {
            Category::X => part.x,
            Category::M => part.m,
            Category::A => part.a,
            Category::S => part.s,
          };
          let is_match = match condition.op {
            Op::GreaterThan => value > condition.value,
            Op::LessThan => value < condition.value,
          };
          if is_match {
            current_workflow = rule.label.clone();
            break;
          }
        } else {
          current_workflow = rule.label.clone();
          break;
        }
      }
    }
    if current_workflow == "A" {
      total += part.sum();
    }
  }

  Ok(total)
}

fn part2(file: &str) -> io::Result<u64> {
  let (workflows, parts) = build_workflows_and_parts(file);

  let mut total: u64 = 0;

  let initial_workflow = "in".to_string();
  let initial_state: HashMap<Category, Vec<u64>> = [
    (Category::X, vec![1, 4000]),
    (Category::M, vec![1, 4000]),
    (Category::A, vec![1, 4000]),
    (Category::S, vec![1, 4000]),
  ]
  .iter()
  .cloned()
  .collect();

  Ok(traverse(&initial_workflow, &initial_state, &workflows))
}

fn build_workflows_and_parts(file: &str) -> (HashMap<String, Vec<Rule>>, Vec<Part>) {
  let input = fs::read_to_string(file).unwrap();

  let mut workflows: HashMap<String, Vec<Rule>> = HashMap::new();
  let mut is_workflow = true;
  let mut parts: Vec<Part> = Vec::new();

  for line in input.lines() {
    if line.trim() == "" {
      is_workflow = false;
      continue;
    }

    if is_workflow {
      let end_of_name_idx = line.find("{").unwrap();
      let name = line[0..end_of_name_idx].trim().to_string();
      let rules = line[end_of_name_idx + 1..line.len() - 1].split(",");
      let mut rules_vec: Vec<Rule> = Vec::new();

      for rule in rules {
        let mut condition: Option<Condition> = None;
        let mut label: String = "".to_string();
        if let Some(colon_index) = rule.find(":") {
          condition = Condition::from_string(&rule[0..colon_index]);
          label = rule[colon_index + 1..].trim().to_string();
        } else {
          label = rule.to_string();
        }
        rules_vec.push(Rule { condition, label });
      }

      workflows.insert(name, rules_vec);
    } else {
      let cats = line[1..line.len() - 1].split(",");
      let values = cats
        .map(|s| s.split('=').last().unwrap())
        .map(|num| num.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

      let part = Part {
        x: values[0],
        m: values[1],
        a: values[2],
        s: values[3],
      };

      parts.push(part);
    }
  }

  (workflows, parts)
}

fn traverse(
  initial_workflow: &str,
  initial_state: &HashMap<Category, Vec<u64>>,
  workflows: &HashMap<String, Vec<Rule>>,
) -> u64 {
  if initial_workflow == "A" {
    return count_combinations(initial_state);
  } else if initial_workflow == "R" {
    return 0;
  }

  let mut total: u64 = 0;
  let rules = workflows.get(initial_workflow).unwrap();

  let mut next_state = initial_state.clone();
  for rule in rules.iter() {
    if let Some(condition) = rule.condition {
      match condition.op {
        Op::GreaterThan => {
          let mut new_state = next_state.clone();
          new_state.get_mut(&condition.category).unwrap()[0] = condition.value + 1;
          total += traverse(&rule.label, &new_state, workflows);

          next_state.get_mut(&condition.category).unwrap()[1] = condition.value;
        }
        Op::LessThan => {
          let mut new_state = next_state.clone();
          new_state.get_mut(&condition.category).unwrap()[1] = condition.value - 1;
          total += traverse(&rule.label, &new_state, workflows);

          next_state.get_mut(&condition.category).unwrap()[0] = condition.value;
        }
      }
    } else {
      if rule.label == "A" {
        total += count_combinations(&next_state);
      } else if rule.label != "R" {
        total += traverse(&rule.label, &next_state, workflows);
      }
    }
  }

  total
}

fn count_combinations(initial_state: &HashMap<Category, Vec<u64>>) -> u64 {
  let mut total: u64 = 1;
  for (_, range) in initial_state.iter() {
    total *= range[1] - range[0] + 1;
  }
  total
}
