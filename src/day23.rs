use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::io;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Pos {
  row: usize,
  col: usize,
}

pub fn solve() {
  println!("Part 1: {}", part1("inputs/day23/part1").unwrap());
  println!("Part 2: {}", part2("inputs/day23/part1").unwrap());
}

fn part1(file: &str) -> io::Result<usize> {
  let (map, nrow, ncol) = read_map(file)?;
  let mut visited: HashSet<usize> = HashSet::new();
  visited.insert(1);
  let mut max_distance = 0;

  dfs(&map, &mut visited, nrow, ncol, 0, 1, 0, &mut max_distance);

  Ok(max_distance)
}

fn part2(file: &str) -> io::Result<usize> {
  let (map, nrow, ncol) = read_map(file)?;
  let intersections = get_all_intersections(&map, nrow, ncol);
  let mut graph: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();
  let mut visited: HashSet<usize> = HashSet::new();

  visited.insert(1);
  graph.insert(1, vec![]);

  build_graph(
    &map,
    &intersections,
    nrow,
    ncol,
    0,
    1,
    1,
    &mut visited,
    &mut graph,
    (1, 0),
  );

  visited.clear();
  let mut max_distance = 0;
  dfs_graph_brute_force(&graph, 1, map.len() - 2, &mut visited, 0, &mut max_distance);

  Ok(max_distance - 1)
}

fn dfs(
  map: &Vec<char>,
  visited: &mut HashSet<usize>,
  nrow: usize,
  ncol: usize,
  row: usize,
  col: usize,
  distance: usize,
  max_distance: &mut usize,
) {
  let idx = row * ncol + col;
  if idx == map.len() - 2 {
    // destination
    if *max_distance < distance {
      *max_distance = distance;
    }
    return;
  }

  // calculate dirs
  let dirs: Vec<(i32, i32)>;
  match map[idx] {
    '>' => {
      dirs = vec![(0, 1)];
    }
    '<' => {
      dirs = vec![(0, -1)];
    }
    '^' => {
      dirs = vec![(-1, 0)];
    }
    'v' => {
      dirs = vec![(1, 0)];
    }
    '.' => {
      dirs = vec![(1, 0), (-1, 0), (0, -1), (0, 1)];
    }
    _ => {
      panic!("Invalid tile");
    }
  }

  //
  for d in dirs {
    let next_row = row as i32 + d.0;
    let next_col = col as i32 + d.1;

    if next_row < 0 || next_row >= nrow as i32 || next_col < 0 || next_col >= ncol as i32 {
      continue;
    }

    let next_idx = next_row as usize * ncol + next_col as usize;
    let next_tile = map[next_idx];

    if next_tile == '#'
      || (next_tile == '>' && d == (0, -1))
      || (next_tile == '<' && d == (0, 1))
      || (next_tile == '^' && d == (1, 0))
      || (next_tile == 'v' && d == (-1, 0))
    {
      continue;
    }

    if visited.contains(&next_idx) {
      continue;
    }

    visited.insert(next_idx);
    dfs(
      map,
      visited,
      nrow,
      ncol,
      next_row as usize,
      next_col as usize,
      distance + 1,
      max_distance,
    );
    visited.remove(&next_idx);
  }
}

fn dfs_graph_brute_force(
  graph: &HashMap<usize, Vec<(usize, usize)>>,
  current_idx: usize,
  destination_idx: usize,
  visited: &mut HashSet<usize>,
  distance: usize,
  max_distance: &mut usize,
) -> bool {
  if current_idx == destination_idx {
    return true;
  }
  if graph[&current_idx].len() == 0 {
    return false;
  }
  let mut reached_destination = false;
  for (next_idx, dist) in &graph[&current_idx] {
    if visited.contains(next_idx) {
      continue;
    }
    visited.insert(*next_idx);
    let visited_destination = dfs_graph_brute_force(
      graph,
      *next_idx,
      destination_idx,
      visited,
      distance + dist,
      max_distance,
    );
    visited.remove(next_idx);
    if visited_destination {
      reached_destination = true;
      if *max_distance < distance + dist {
        *max_distance = distance + dist;
      }
    }
  }
  reached_destination
}

fn read_map(file: &str) -> io::Result<(Vec<char>, usize, usize)> {
  let input = fs::read_to_string(file)?;
  let nrow = input.lines().count();
  let ncol = input.lines().next().unwrap().len();
  let map = input
    .lines()
    .flat_map(|line| line.chars())
    .collect::<Vec<char>>();

  Ok((map, nrow, ncol))
}

fn get_all_intersections(map: &Vec<char>, nrow: usize, ncol: usize) -> HashSet<usize> {
  let mut intersections: HashSet<usize> = HashSet::new();

  for r in 0..nrow {
    for c in 0..ncol {
      let idx = r * ncol + c;
      if map[idx] == '#' {
        continue;
      }
      let dirs = vec![(1, 0), (-1, 0), (0, -1), (0, 1)];
      let mut neighbor_cnt = 0;
      for d in dirs {
        let next_row = r as i32 + d.0;
        let next_col = c as i32 + d.1;

        if next_row < 0 || next_row >= nrow as i32 || next_col < 0 || next_col >= ncol as i32 {
          continue;
        }

        let next_idx = next_row as usize * ncol + next_col as usize;
        let next_tile = map[next_idx];

        if next_tile == '#' {
          continue;
        }
        neighbor_cnt += 1;
      }
      if neighbor_cnt > 2 {
        intersections.insert(idx);
      }
    }
  }
  intersections
}

// return the mapping of idx -> [(idx, distance)]
fn build_graph(
  map: &Vec<char>,
  intersections: &HashSet<usize>,
  nrow: usize,
  ncol: usize,
  start_row: usize,
  start_col: usize,
  current_intersection: usize,
  visited: &mut HashSet<usize>,
  graph: &mut HashMap<usize, Vec<(usize, usize)>>,
  dir: (i32, i32),
) {
  let mut distance = 1;
  let mut cur_row = start_row;
  let mut cur_col = start_col;
  let mut cur_idx = cur_row * ncol + cur_col;
  let mut cur_dir = dir;
  let mut is_dead_end = false;

  while !intersections.contains(&cur_idx) {
    let mut next_row = cur_row as i32 + cur_dir.0;
    let mut next_col = cur_col as i32 + cur_dir.1;
    let mut next_idx = next_row as usize * ncol + next_col as usize;

    if visited.contains(&next_idx) {
      if intersections.contains(&next_idx) {
        // visit an old intersection
        update_graph(graph, current_intersection, next_idx, distance);
      } else {
        // is dead end
        is_dead_end = true;
        break;
      }
    }

    if next_row < 0
      || next_row >= nrow as i32
      || next_col < 0
      || next_col >= ncol as i32
      || map[next_idx] == '#'
    {
      // find new dir
      let dirs = gen_dirs(cur_dir);

      let mut found_new_dir = false;
      for d in dirs {
        next_row = cur_row as i32 + d.0;
        next_col = cur_col as i32 + d.1;
        next_idx = next_row as usize * ncol + next_col as usize;

        if next_row < 0
          || next_row >= nrow as i32
          || next_col < 0
          || next_col >= ncol as i32
          || map[next_idx] == '#'
        {
          continue;
        }
        found_new_dir = true;
        cur_dir = d;
        break;
      }
      if !found_new_dir {
        // cannot find a new dir
        is_dead_end = true;

        break;
      }
    }

    distance += 1;
    cur_row = next_row as usize;
    cur_col = next_col as usize;
    cur_idx = next_idx;
    visited.insert(cur_idx);
  }

  if cur_idx == map.len() - 2 {
    // reach the destination
    update_graph(graph, current_intersection, cur_idx, distance);
  }

  if is_dead_end {
    return;
  }

  update_graph(graph, current_intersection, cur_idx, distance);

  let mut dirs = gen_dirs(cur_dir);
  dirs.push(cur_dir);
  for d in dirs {
    let next_row = cur_row as i32 + d.0;
    let next_col = cur_col as i32 + d.1;

    if next_row < 0 || next_row >= nrow as i32 || next_col < 0 || next_col >= ncol as i32 {
      continue;
    }

    let next_idx = next_row as usize * ncol + next_col as usize;
    let next_tile = map[next_idx];

    if next_tile == '#' || visited.contains(&next_idx) {
      continue;
    }

    build_graph(
      map,
      intersections,
      nrow,
      ncol,
      next_row as usize,
      next_col as usize,
      cur_idx,
      visited,
      graph,
      d,
    );
  }
}

fn gen_dirs(dir: (i32, i32)) -> Vec<(i32, i32)> {
  match dir {
    (1, 0) | (-1, 0) => vec![(0, 1), (0, -1)],
    (0, 1) | (0, -1) => vec![(1, 0), (-1, 0)],
    _ => panic!("Invalid dir"),
  }
}

fn update_graph(
  graph: &mut HashMap<usize, Vec<(usize, usize)>>,
  intersection1: usize,
  intersection2: usize,
  distance: usize,
) {
  // update intersection1
  if graph.contains_key(&intersection1) {
    let mut has_intersection2 = false;
    for (idx, dist) in graph.get_mut(&intersection1).unwrap().iter_mut() {
      if *idx == intersection2 {
        has_intersection2 = true;
        if *dist < distance {
          *dist = distance;
        }
      }
    }
    if !has_intersection2 {
      graph
        .get_mut(&intersection1)
        .unwrap()
        .push((intersection2, distance));
    }
  } else {
    graph.insert(intersection1, vec![(intersection2, distance)]);
  }

  // update intersection2
  if graph.contains_key(&intersection2) {
    let mut has_intersection1 = false;
    for (idx, dist) in graph.get_mut(&intersection2).unwrap().iter_mut() {
      if *idx == intersection1 {
        has_intersection1 = true;
        if *dist < distance {
          *dist = distance;
        }
      }
    }
    if !has_intersection1 {
      graph
        .get_mut(&intersection2)
        .unwrap()
        .push((intersection1, distance));
    }
  } else {
    graph.insert(intersection2, vec![(intersection1, distance)]);
  }
}
