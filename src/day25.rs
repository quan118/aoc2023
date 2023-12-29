use rand::Rng;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::io;

pub fn solve() {
  println!("Part 1: {}", part1("inputs/day25/part1").unwrap());
}

fn part1(file: &str) -> io::Result<usize> {
  let mut map = parse_file(file)?;
  let vertices: Vec<String> = map.keys().map(|k| k.clone()).collect::<Vec<String>>();

  println!("Using monte carlo method to find solution. This may take a few minutes...");

  let mut rng = rand::thread_rng();
  let mut edges_stats: HashMap<(String, String), usize> = HashMap::new();
  let keys_cnt = map.keys().len();

  for _ in 0..keys_cnt {
    // generate a random number from 0 to vertices.len()
    let v1 = rng.gen_range(0..vertices.len());
    let mut v2 = v1;
    while v2 == v1 {
      v2 = rng.gen_range(0..vertices.len());
    }
    let v1 = vertices[v1].clone();
    let v2 = vertices[v2].clone();

    dfs(&map, &v1, &v2, &mut edges_stats);
  }

  // find top 10 edges by value
  let top = 10;
  let mut edges: Vec<((String, String), usize)> = edges_stats
    .iter()
    .map(|(k, v)| (k.clone(), v.clone()))
    .collect::<Vec<((String, String), usize)>>();
  edges.sort_by(|a, b| b.1.cmp(&a.1));

  let mut total = 0;

  'a: for i in 0..top - 2 {
    let e1 = edges[i].0.clone();
    remove_edge(&mut map, &e1);

    for j in i + 1..top - 1 {
      let e2 = edges[j].0.clone();
      remove_edge(&mut map, &e2);

      for k in j + 1..top {
        let e3 = edges[k].0.clone();
        remove_edge(&mut map, &e3);

        // println!("e1: {:?} e2: {:?} e3: {:?}", e1, e2, e3);

        let result = count_sets(&map);
        // println!("{:?}", result);
        if result.len() == 2 {
          total = result[0] * result[1];

          break 'a;
        }

        add_edge(&mut map, &e3);
      }

      add_edge(&mut map, &e2);
    }

    add_edge(&mut map, &e1);
  }

  Ok(total)
}

fn count_sets(map: &HashMap<String, HashSet<String>>) -> Vec<usize> {
  let mut visited: HashSet<String> = HashSet::new();
  let mut queue: VecDeque<String> = VecDeque::new();
  let mut output: Vec<usize> = Vec::new();

  for (k, _) in map {
    let mut cnt: usize = 0;
    if visited.contains(k) {
      continue;
    }

    queue.push_back(k.clone());

    while queue.len() > 0 {
      let current = queue.pop_front().unwrap();
      if !visited.contains(&current) {
        visited.insert(current.clone());
        cnt += 1;
      }

      for neighbor in map.get(&current).unwrap() {
        if visited.contains(neighbor) {
          continue;
        }
        queue.push_back(neighbor.clone());
      }
    }
    output.push(cnt);
  }

  output
}

fn dfs(
  map: &HashMap<String, HashSet<String>>,
  v1: &str,
  v2: &str,
  edges_stats: &mut HashMap<(String, String), usize>,
) {
  let mut visited: HashSet<String> = HashSet::new();
  let mut stack: Vec<String> = Vec::new();
  let mut tracing: Vec<String> = Vec::new();

  stack.push(v1.to_string());
  visited.insert(v1.to_string());
  tracing.push(v1.to_string());

  while stack.len() > 0 {
    let current = stack.last().unwrap().to_string();
    tracing.push(current.clone());

    if current == v2 {
      break;
    }

    let neighbors = map.get(&current).unwrap();
    let mut neighbors = neighbors.iter().collect::<Vec<&String>>();
    for i in 0..neighbors.len() {
      let j = rand::thread_rng().gen_range(0..neighbors.len());
      neighbors.swap(i, j);
    }

    for neighbor in neighbors {
      if visited.contains(neighbor) {
        continue;
      }
      stack.push(neighbor.clone());
      visited.insert(neighbor.clone());
    }

    // no changes in stack
    if &current == stack.last().unwrap() {
      stack.pop();
      tracing.pop();
    }
  }

  // println!("{:?}", stack);

  // tracing
  for i in 0..tracing.len() - 2 {
    let mut e = [tracing[i].clone(), tracing[i + 1].clone()].to_vec();
    e.sort();
    let e = (e[0].clone(), e[1].clone());
    let cnt = edges_stats.entry(e).or_insert(0);
    *cnt += 1;
  }
}

fn remove_edge(map: &mut HashMap<String, HashSet<String>>, edge: &(String, String)) {
  let neighbors = map.get_mut(&edge.0).unwrap();
  neighbors.remove(&edge.1);

  let neighbors = map.get_mut(&edge.1).unwrap();
  neighbors.remove(&edge.0);
}

fn add_edge(map: &mut HashMap<String, HashSet<String>>, edge: &(String, String)) {
  let neighbors = map.entry(edge.0.clone()).or_insert(HashSet::new());
  neighbors.insert(edge.1.clone());

  let neighbors = map.entry(edge.1.clone()).or_insert(HashSet::new());
  neighbors.insert(edge.0.clone());
}

fn parse_file(file: &str) -> io::Result<HashMap<String, HashSet<String>>> {
  let input = fs::read_to_string(file)?;

  let mut map: HashMap<String, HashSet<String>> = HashMap::new();
  for line in input.lines() {
    let parts = line
      .split([':', ' '])
      .filter(|s| !s.is_empty())
      .collect::<Vec<&str>>();

    let neighbors = map.entry(parts[0].to_string()).or_insert(HashSet::new());

    for i in 1..parts.len() {
      neighbors.insert(parts[i].to_string());
    }

    for i in 1..parts.len() {
      let neighbors_of_neighbor = map.entry(parts[i].to_string()).or_insert(HashSet::new());

      neighbors_of_neighbor.insert(parts[0].to_string());
    }
  }
  Ok(map)
}
