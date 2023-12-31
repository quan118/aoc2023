use std::collections::HashSet;
use std::fs;
use std::io;

pub fn solve() {
  println!("Part 1: {}", part1("inputs/day10/part1").unwrap());
  println!("Part 2: {}", part2("inputs/day10/part1").unwrap());
}

fn part1(file: &str) -> io::Result<u64> {
  let input = fs::read_to_string(file)?;
  let nrow = input.lines().count();
  let ncol = input.lines().next().unwrap().len();
  let mut map = input.chars().filter(|c| *c != '\n').collect::<Vec<char>>();
  let (row, col) = get_starting_position(&map, nrow, ncol);
  map[row * ncol + col] = '-';

  Ok((count_steps(&map, nrow, ncol, row, col) as f64 / 2.0).ceil() as u64)
}

fn part2(file: &str) -> io::Result<u64> {
  let input = fs::read_to_string(file)?;
  let nrow = input.lines().count();
  let ncol = input.lines().next().unwrap().len();
  let mut map = input.chars().filter(|c| *c != '\n').collect::<Vec<char>>();
  let (row, col) = get_starting_position(&map, nrow, ncol);
  map[row * ncol + col] = '-';

  Ok(count_enclosed_tiles(&map, nrow, ncol, row, col))
}

fn count_steps(map: &Vec<char>, nrow: usize, ncol: usize, row: usize, col: usize) -> u64 {
  let mut visited: HashSet<usize> = HashSet::new();
  let mut count: u64 = 0;
  visited.insert(row * ncol + col);

  let mut current_row = row;
  let mut current_col = col;

  loop {
    let next_position = get_next_position(&map, nrow, ncol, current_row, current_col, &visited);
    if next_position.is_none() {
      break;
    }
    current_row = next_position.unwrap().0;
    current_col = next_position.unwrap().1;
    visited.insert(current_row * ncol + current_col);
    count += 1;
  }

  count
}

fn get_next_position(
  map: &Vec<char>,
  nrow: usize,
  ncol: usize,
  row: usize,
  col: usize,
  visited: &HashSet<usize>,
) -> Option<(usize, usize)> {
  let idx = row * ncol + col;
  let can_go_up = row > 0 && !visited.contains(&((row - 1) * ncol + col));
  let can_go_down = row < nrow - 1 && !visited.contains(&((row + 1) * ncol + col));
  let can_go_left = col > 0 && !visited.contains(&(row * ncol + col - 1));
  let can_go_right = col < ncol - 1 && !visited.contains(&(row * ncol + col + 1));

  match map[idx] {
    '|' => {
      if can_go_up {
        Some((row - 1, col))
      } else if can_go_down {
        Some((row + 1, col))
      } else {
        None
      }
    }
    '-' => {
      if can_go_left {
        Some((row, col - 1))
      } else if can_go_right {
        Some((row, col + 1))
      } else {
        None
      }
    }
    'F' => {
      if can_go_down {
        Some((row + 1, col))
      } else if can_go_right {
        Some((row, col + 1))
      } else {
        None
      }
    }
    'L' => {
      if can_go_up {
        Some((row - 1, col))
      } else if can_go_right {
        Some((row, col + 1))
      } else {
        None
      }
    }
    'J' => {
      if can_go_up {
        Some((row - 1, col))
      } else if can_go_left {
        Some((row, col - 1))
      } else {
        None
      }
    }
    '7' => {
      if can_go_down {
        Some((row + 1, col))
      } else if can_go_left {
        Some((row, col - 1))
      } else {
        None
      }
    }
    _ => None,
  }
}

fn get_starting_position(map: &Vec<char>, nrow: usize, ncol: usize) -> (usize, usize) {
  let mut row: usize = 0;
  let mut col: usize = 0;
  for idx in 0..map.len() {
    if map[idx] == 'S' {
      row = idx / ncol;
      col = idx % ncol;
      break;
    }
  }

  (row, col)
}

fn get_pipes_of_loop(
  map: &Vec<char>,
  nrow: usize,
  ncol: usize,
  start_row: usize,
  start_col: usize,
) -> HashSet<usize> {
  let mut visited: HashSet<usize> = HashSet::new();
  visited.insert(start_row * ncol + start_col);

  let mut current_row = start_row;
  let mut current_col = start_col;

  loop {
    let next_position = get_next_position(&map, nrow, ncol, current_row, current_col, &visited);
    if next_position.is_none() {
      break;
    }
    current_row = next_position.unwrap().0;
    current_col = next_position.unwrap().1;
    visited.insert(current_row * ncol + current_col);
  }

  visited
}

fn count_enclosed_tiles(
  map: &Vec<char>,
  nrow: usize,
  ncol: usize,
  start_row: usize,
  start_col: usize,
) -> u64 {
  let pipes_of_loop = get_pipes_of_loop(map, nrow, ncol, start_row, start_col);
  let mut is_inside = false;
  let mut total: u64 = 0;
  let mut entering_pipe: Option<char> = None;

  for r in 0..nrow {
    is_inside = false;
    entering_pipe = None;
    for c in 0..ncol {
      let idx = r * ncol + c;
      if pipes_of_loop.contains(&idx) {
        match map[idx] {
          '|' => {
            if is_inside {
              is_inside = false;
              entering_pipe = None;
            } else {
              is_inside = true;
              entering_pipe = Some('|');
            }
          }
          'F' | 'L' => {
            entering_pipe = Some(map[idx]);
          }
          '7' => {
            if let Some(pipe) = entering_pipe {
              if pipe == 'L' {
                is_inside = !is_inside;
              }
            }
          }
          'J' => {
            if let Some(pipe) = entering_pipe {
              if pipe == 'F' {
                is_inside = !is_inside;
              }
            }
          }
          _ => {}
        }
      } else if is_inside {
        total += 1;
      }
    }
  }

  total
}
