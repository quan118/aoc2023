use std::collections::HashSet;
use std::fs;
use std::io;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Pos {
  row: i64,
  col: i64,
}

pub fn solve() {
  println!("Part 1: {}", part1("inputs/day21/part1").unwrap());
  println!("Part 2: {}", part2("inputs/day21/part1").unwrap());
}

fn part1(file: &str) -> io::Result<usize> {
  let (map, nrow, ncol, start_idx) = read_map(file)?;

  let mut steps: usize = 64;
  let mut visited: HashSet<usize> = HashSet::new();
  visited.insert(start_idx);

  while steps > 0 {
    let visited_arr = visited.iter().cloned().collect::<Vec<usize>>();
    visited.clear();
    for idx in visited_arr.iter() {
      let row = idx / ncol;
      let col = idx % ncol;
      let next_pos = generate_next_pos(&map, nrow, ncol, row, col);
      for next_idx in next_pos.iter() {
        visited.insert(*next_idx);
      }
    }
    steps -= 1;
  }

  Ok(visited.len())
}

fn part2(file: &str) -> io::Result<usize> {
  let (map, nrow, ncol, start_idx) = read_map(file)?;

  let mut f: Vec<f64> = Vec::new();
  let mut a: Vec<f64> = Vec::new();
  for i in 1..4 {
    let max_steps = (nrow * (i * 2 + 1) - 1) / 2;
    let total = count_plots(&map, nrow, ncol, start_idx, max_steps);
    f.push(total as f64);
    a.push(max_steps as f64);
  }

  let coefficients = solve_matrix_equation(a[0], a[1], a[2], f[0], f[1], f[2]).unwrap();

  const MAX_STEPS: usize = 26501365;

  let result = coefficients.0 * MAX_STEPS as f64 * MAX_STEPS as f64
    + coefficients.1 * MAX_STEPS as f64
    + coefficients.2 as f64;

  Ok(result as usize)
}

fn read_map(file: &str) -> io::Result<(Vec<char>, usize, usize, usize)> {
  let input = fs::read_to_string(file)?;
  let nrow = input.lines().count();
  let ncol = input.lines().next().unwrap().len();
  let map = input
    .lines()
    .flat_map(|line| line.chars())
    .collect::<Vec<char>>();

  let start_idx = map.iter().position(|&c| c == 'S').unwrap();

  Ok((map, nrow, ncol, start_idx))
}

fn generate_next_pos(
  map: &Vec<char>,
  nrow: usize,
  ncol: usize,
  row: usize,
  col: usize,
) -> Vec<usize> {
  let dirs = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
  let mut next_pos: Vec<usize> = Vec::new();

  for d in dirs {
    let next_row = row as i64 + d.0;
    let next_col = col as i64 + d.1;
    let next_idx: usize = next_row as usize * ncol + next_col as usize;
    if 0 <= next_row
      && next_row < nrow as i64
      && 0 <= next_col
      && next_col < ncol as i64
      && map[next_idx] != '#'
    {
      next_pos.push(next_idx);
    }
  }

  next_pos
}

fn generate_next_pos_no_boundary(map: &Vec<char>, nrow: i64, ncol: i64, pos: &Pos) -> Vec<Pos> {
  let dirs = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
  let mut next_pos: Vec<Pos> = Vec::new();

  for d in dirs {
    let next_row = pos.row + d.0;
    let next_col = pos.col + d.1;
    let co_next_row = mod_positive(next_row, nrow);
    let co_next_col: i64 = mod_positive(next_col, ncol);
    let co_next_idx: i64 = co_next_row * ncol + co_next_col;
    // println!("co_next_idx: {co_next_idx}");
    if map[co_next_idx as usize] != '#' {
      next_pos.push(Pos {
        col: next_col,
        row: next_row,
      });
    }
  }

  next_pos
}

fn mod_positive(a: i64, b: i64) -> i64 {
  let r = a % b;
  if r < 0 {
    r + b
  } else {
    r
  }
}

fn count_plots(
  map: &Vec<char>,
  nrow: usize,
  ncol: usize,
  start_idx: usize,
  max_steps: usize,
) -> usize {
  let mut visited_at_odd_steps: HashSet<Pos> = HashSet::new();
  let mut visited_at_even_steps: HashSet<Pos> = HashSet::new();
  let mut new_visited: HashSet<Pos> = HashSet::new();
  let mut steps = 0;
  new_visited.insert(Pos {
    row: start_idx as i64 / ncol as i64,
    col: start_idx as i64 % ncol as i64,
  });

  while steps <= max_steps {
    let visited_arr = new_visited.iter().cloned().collect::<Vec<Pos>>();
    new_visited.clear();

    let visited = if steps % 2 == 0 {
      &mut visited_at_even_steps
    } else {
      &mut visited_at_odd_steps
    };
    for pos in visited_arr.iter() {
      visited.insert(*pos);
    }

    for pos in visited_arr {
      let next_pos = generate_next_pos_no_boundary(&map, nrow as i64, ncol as i64, &pos);
      for new_pos in next_pos.iter() {
        if steps % 2 == 0 && visited_at_odd_steps.contains(new_pos) {
          continue;
        } else if steps % 2 == 1 && visited_at_even_steps.contains(new_pos) {
          continue;
        }
        new_visited.insert(new_pos.clone());
      }
    }
    steps += 1;
  }

  let total = if max_steps % 2 == 0 {
    visited_at_even_steps.len()
  } else {
    visited_at_odd_steps.len()
  };

  total
}

fn solve_matrix_equation(
  a: f64,
  b: f64,
  c: f64,
  u: f64,
  v: f64,
  w: f64,
) -> Option<(f64, f64, f64)> {
  // Calculate the denominators
  let denom1 = a * a - a * b - a * c + b * c;
  let denom2 = a * b - a * c - b * b + b * c;
  let denom3 = a * b - a * c - b * c + c * c;

  // Check for invertibility of matrix A
  if denom1 == 0.0 || denom2 == 0.0 || denom3 == 0.0 {
    return None;
  }

  // Calculate x, y, and z
  let x = u / denom1 - v / denom2 + w / denom3;
  let y = u * (-b - c) / denom1 + v * (a + c) / denom2 + w * (-a - b) / denom3;
  let z = a * b * w / denom3 - a * c * v / denom2 + b * c * u / denom1;

  Some((x, y, z))
}
