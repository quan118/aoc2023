use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::io;

#[derive(Debug)]
struct Node {
  left: String,
  right: String,
}

pub fn solve() {
  println!("Part 1: {}", part1("inputs/day8/part1").unwrap());
  println!("Part 2: {}", part2("inputs/day8/part1").unwrap());
}

fn part1(file: &str) -> io::Result<u64> {
  let (instructions, nodes) = build_instructions_and_nodes(file);

  let steps = count_steps(&instructions, &nodes);

  Ok(steps)
}

fn part2(file: &str) -> io::Result<u64> {
  let (instructions, nodes) = build_instructions_and_nodes(file);
  let mut steps_count_set: HashSet<u64> = HashSet::new();

  for (current_node, node) in nodes.iter() {
    if current_node.ends_with("A") {
      let steps = count_steps_part_2(&instructions, current_node, &nodes);
      steps_count_set.insert(steps);
    }
  }

  let steps = steps_count_set
    .iter()
    .fold(1, |acc, steps| lcm(acc, *steps));
  // println!("steps_count_set: {:?}", steps_count_set);

  Ok(steps)
}

fn build_instructions_and_nodes(file: &str) -> (Vec<char>, HashMap<String, Node>) {
  let input = fs::read_to_string(file).unwrap();
  let mut nodes: HashMap<String, Node> = HashMap::new();
  let mut line_iter = input.lines();

  // first line is instructions
  let instructions = line_iter.next().unwrap().chars().collect::<Vec<char>>();
  line_iter.next(); // skip the empty line
  while let Some(line) = line_iter.next() {
    let mut split = line.split("=");
    let label = split.next().expect("Missing label").trim().to_string();
    let neighbours_str = split.next().expect("Missing neighbours");
    let neighbours_str = neighbours_str.trim();
    let neighbours_str = neighbours_str.trim_matches('(').trim_matches(')');
    let mut neighbours = neighbours_str.split(',');

    let left = neighbours
      .next()
      .expect("Missing left neighbour")
      .trim()
      .to_string();
    let right = neighbours
      .next()
      .expect("Missing right neighbour")
      .trim()
      .to_string();
    let node = Node { left, right };
    nodes.insert(label, node);
  }

  (instructions, nodes)
}

fn count_steps(instructions: &Vec<char>, nodes: &HashMap<String, Node>) -> u64 {
  let mut steps: u64 = 0;
  let mut current_node = "AAA";
  let mut ins_idx = 0;
  while current_node != "ZZZ" {
    if instructions[ins_idx] == 'L' {
      current_node = &nodes
        .get(current_node)
        .expect("Missing {current_node} node")
        .left;
    } else {
      current_node = &nodes
        .get(current_node)
        .expect("Missing {current_node} node")
        .right;
    }
    steps += 1;
    ins_idx += 1;
    if ins_idx >= instructions.len() {
      ins_idx = 0;
    }
  }

  steps
}

fn count_steps_part_2(
  instructions: &Vec<char>,
  current_node: &str,
  nodes: &HashMap<String, Node>,
) -> u64 {
  let mut steps: u64 = 0;
  let mut current_node = current_node;
  let mut ins_idx = 0;
  while !current_node.ends_with("Z") {
    if instructions[ins_idx] == 'L' {
      current_node = &nodes
        .get(current_node)
        .expect("Missing {current_node} node")
        .left;
    } else {
      current_node = &nodes
        .get(current_node)
        .expect("Missing {current_node} node")
        .right;
    }
    steps += 1;
    ins_idx += 1;
    if ins_idx >= instructions.len() {
      ins_idx = 0;
    }
  }

  steps
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
