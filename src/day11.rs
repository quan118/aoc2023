use std::fs;
use std::io;

pub fn solve() {
  println!("Part 1: {}", part1("inputs/day11/part1").unwrap());
}

fn part1(file: &str) -> io::Result<u32> {
  let input = fs::read_to_string(file)?;
  let nrow = input.lines().count();
  let ncol = input.lines().next().unwrap().len();
  let map = input.chars().filter(|c| *c != '\n').collect::<Vec<char>>();

  // find empty row and empty col
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

  // find expanded rows and cols
  let expanded_rows: Vec<u32> = empty_rows
    .iter()
    .enumerate()
    .map(|(idx, &value)| idx as u32 + value + 1)
    .collect();
  let expanded_cols: Vec<u32> = empty_cols
    .iter()
    .enumerate()
    .map(|(idx, &value)| idx as u32 + value + 1)
    .collect();

  // create expanded map
  let expanded_map_nrow = nrow + empty_rows.len();
  let expanded_map_ncol = ncol + empty_cols.len();
  let mut expanded_map = vec![' '; expanded_map_nrow * expanded_map_ncol];

  for row in expanded_rows {
    for col in 0..expanded_map_ncol {
      expanded_map[row as usize * expanded_map_ncol + col as usize] = '.';
    }
  }

  for col in expanded_cols {
    for row in 0..expanded_map_nrow {
      expanded_map[row as usize * expanded_map_ncol + col as usize] = '.';
    }
  }

  let mut j = 0;
  for i in 0..expanded_map.len() {
    if expanded_map[i] != '.' {
      expanded_map[i] = map[j];
      j += 1;
    }
  }

  // print expanded map
  // for row in 0..expanded_map_nrow {
  //   for col in 0..expanded_map_ncol {
  //     print!("{}", expanded_map[row as usize* expanded_map_ncol + col as usize]);
  //   }
  //   println!();
  // }

  // find all galaxies
  let galaxies = expanded_map
    .iter()
    .enumerate()
    .filter(|(_, &value)| value == '#')
    .map(|(idx, _)| idx)
    .collect::<Vec<usize>>();

  // find shortest paths between 2 galaxies
  let mut total: u32 = 0;
  for i in 0..galaxies.len() - 1 {
    for j in i + 1..galaxies.len() {
      let row1 = galaxies[i] / expanded_map_ncol;
      let col1 = galaxies[i] % expanded_map_ncol;
      let row2 = galaxies[j] / expanded_map_ncol;
      let col2 = galaxies[j] % expanded_map_ncol;
      total += (row1 as i32 - row2 as i32).abs() as u32 + (col1 as i32 - col2 as i32).abs() as u32;
    }
  }

  Ok(total)
}
