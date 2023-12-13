use std::fs;
use std::io;

pub fn solve() {
  println!("Part 1: {}", part1("inputs/day11/part1").unwrap());
  println!("Part 2: {}", part2("inputs/day11/part1").unwrap());
}

fn part1(file: &str) -> io::Result<u64> {
  solveImpl(file, 2)
}

fn part2(file: &str) -> io::Result<u64> {
  solveImpl(file, 1000000)
}

fn solveImpl(file: &str, expansion_times: u64) -> io::Result<u64> {
  let (map, nrow, ncol) = read_map(file)?;

  let (empty_rows, empty_cols) = get_empty_rows_and_cols(&map, nrow, ncol);

  // find all galaxies
  let galaxies = map
    .iter()
    .enumerate()
    .filter(|(_, &value)| value == '#')
    .map(|(idx, _)| idx)
    .collect::<Vec<usize>>();

  // find shortest paths between 2 galaxies
  let mut total: u64 = 0;
  for i in 0..galaxies.len() - 1 {
    for j in i + 1..galaxies.len() {
      let row1 = galaxies[i] / ncol;
      let col1 = galaxies[i] % nrow;
      let row2 = galaxies[j] / ncol;
      let col2 = galaxies[j] % nrow;
      let fromRow = row1.min(row2);
      let toRow = row1.max(row2);
      let fromCol = col1.min(col2);
      let toCol = col1.max(col2);

      for r in fromRow..toRow {
        if empty_rows.contains(&(r as u32)) {
          total += expansion_times;
        } else {
          total += 1;
        }
      }

      for c in fromCol..toCol {
        if empty_cols.contains(&(c as u32)) {
          total += expansion_times;
        } else {
          total += 1;
        }
      }
    }
  }

  Ok(total)
}

fn read_map(file: &str) -> io::Result<(Vec<char>, usize, usize)> {
  let input = fs::read_to_string(file)?;
  let nrow = input.lines().count();
  let ncol = input.lines().next().unwrap().len();
  let map = input.chars().filter(|c| *c != '\n').collect::<Vec<char>>();

  Ok((map, nrow, ncol))
}

fn get_empty_rows_and_cols(map: &Vec<char>, nrow: usize, ncol: usize) -> (Vec<u32>, Vec<u32>) {
  let mut empty_rows: Vec<u32> = Vec::new();
  let mut empty_cols: Vec<u32> = Vec::new();

  for row in 0..nrow {
    let mut is_empty = true;
    for col in 0..ncol {
      if map[row * ncol + col] != '.' {
        is_empty = false;
        break;
      }
    }
    if is_empty {
      empty_rows.push(row as u32);
    }
  }

  for col in 0..ncol {
    let mut is_empty = true;
    for row in 0..nrow {
      if map[row * ncol + col] != '.' {
        is_empty = false;
        break;
      }
    }
    if is_empty {
      empty_cols.push(col as u32);
    }
  }

  (empty_rows, empty_cols)
}
