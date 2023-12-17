use std::collections::HashSet;
use std::fs;
use std::io;

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
enum Direction {
  Up,
  Down,
  Left,
  Right,
}

pub fn solve() {
  println!("Part 1: {}", part1("inputs/day16/part1").unwrap());
  println!("Part 2: {}", part2("inputs/day16/part1").unwrap());
}

fn part1(file: &str) -> io::Result<u64> {
  let (map, nrow, ncol) = read_map(file)?;
  let mut energized: HashSet<(u32, Direction)> = HashSet::new();
  traverse(&map, nrow, ncol, 0, 0, Direction::Right, &mut energized);

  let energized = energized
    .iter()
    .map(|(idx, _)| *idx)
    .collect::<Vec<u32>>()
    .into_iter()
    .collect::<HashSet<u32>>();
  return Ok(energized.len() as u64);
}

fn part2(file: &str) -> io::Result<u64> {
  let (map, nrow, ncol) = read_map(file)?;
  let mut total: u64 = 0;
  // top
  for col in 0..ncol {
    let mut energized: HashSet<(u32, Direction)> = HashSet::new();
    traverse(&map, nrow, ncol, 0, col, Direction::Down, &mut energized);

    let energized = energized
      .iter()
      .map(|(idx, _)| *idx)
      .collect::<Vec<u32>>()
      .into_iter()
      .collect::<HashSet<u32>>();
    total = total.max(energized.len() as u64);
  }
  // bottom
  for col in 0..ncol {
    let mut energized: HashSet<(u32, Direction)> = HashSet::new();
    traverse(
      &map,
      nrow,
      ncol,
      nrow - 1,
      col,
      Direction::Up,
      &mut energized,
    );

    let energized = energized
      .iter()
      .map(|(idx, _)| *idx)
      .collect::<Vec<u32>>()
      .into_iter()
      .collect::<HashSet<u32>>();

    total = total.max(energized.len() as u64);
  }

  // left
  for row in 0..nrow {
    let mut energized: HashSet<(u32, Direction)> = HashSet::new();
    traverse(&map, nrow, ncol, row, 0, Direction::Right, &mut energized);

    let energized = energized
      .iter()
      .map(|(idx, _)| *idx)
      .collect::<Vec<u32>>()
      .into_iter()
      .collect::<HashSet<u32>>();

    total = total.max(energized.len() as u64);
  }

  // right
  for row in 0..nrow {
    let mut energized: HashSet<(u32, Direction)> = HashSet::new();
    traverse(
      &map,
      nrow,
      ncol,
      row,
      ncol - 1,
      Direction::Left,
      &mut energized,
    );

    let energized = energized
      .iter()
      .map(|(idx, _)| *idx)
      .collect::<Vec<u32>>()
      .into_iter()
      .collect::<HashSet<u32>>();

    total = total.max(energized.len() as u64);
  }

  Ok(total)
}

fn traverse<'a, 'b>(
  map: &'a Vec<char>,
  nrow: usize,
  ncol: usize,
  row: usize,
  col: usize,
  dir: Direction,
  energized: &'b mut HashSet<(u32, Direction)>,
) -> &'b mut HashSet<(u32, Direction)> {
  let idx = row * ncol + col;
  if energized.contains(&(idx as u32, dir)) {
    return energized;
  }
  energized.insert((idx as u32, dir));

  match dir {
    Direction::Up => match map[idx] {
      '|' | '.' => {
        if row > 0 {
          traverse(map, nrow, ncol, row - 1, col, dir, energized);
        }
      }
      '/' => {
        if col < ncol - 1 {
          traverse(map, nrow, ncol, row, col + 1, Direction::Right, energized);
        }
      }
      '\\' => {
        if col > 0 {
          traverse(map, nrow, ncol, row, col - 1, Direction::Left, energized);
        }
      }
      '-' => {
        if col > 0 {
          traverse(map, nrow, ncol, row, col - 1, Direction::Left, energized);
        }

        if col < ncol - 1 {
          traverse(map, nrow, ncol, row, col + 1, Direction::Right, energized);
        }
      }
      _ => panic!("Invalid char: {}", map[idx]),
    },
    Direction::Down => match map[idx] {
      '|' | '.' => {
        if row < nrow - 1 {
          traverse(map, nrow, ncol, row + 1, col, dir, energized);
        }
      }
      '/' => {
        if col > 0 {
          traverse(map, nrow, ncol, row, col - 1, Direction::Left, energized);
        }
      }
      '\\' => {
        if col < ncol - 1 {
          traverse(map, nrow, ncol, row, col + 1, Direction::Right, energized);
        }
      }
      '-' => {
        if col > 0 {
          traverse(map, nrow, ncol, row, col - 1, Direction::Left, energized);
        }

        if col < ncol - 1 {
          traverse(map, nrow, ncol, row, col + 1, Direction::Right, energized);
        }
      }
      _ => panic!("Invalid char: {}", map[idx]),
    },
    Direction::Left => match map[idx] {
      '-' | '.' => {
        if col > 0 {
          traverse(map, nrow, ncol, row, col - 1, dir, energized);
        }
      }
      '/' => {
        if row < nrow - 1 {
          traverse(map, nrow, ncol, row + 1, col, Direction::Down, energized);
        }
      }
      '\\' => {
        if row > 0 {
          traverse(map, nrow, ncol, row - 1, col, Direction::Up, energized);
        }
      }
      '|' => {
        if row > 0 {
          traverse(map, nrow, ncol, row - 1, col, Direction::Up, energized);
        }

        if row < nrow - 1 {
          traverse(map, nrow, ncol, row + 1, col, Direction::Down, energized);
        }
      }
      _ => panic!("Invalid char: {}", map[idx]),
    },
    Direction::Right => match map[idx] {
      '-' | '.' => {
        if col < ncol - 1 {
          traverse(map, nrow, ncol, row, col + 1, dir, energized);
        }
      }
      '/' => {
        if row > 0 {
          traverse(map, nrow, ncol, row - 1, col, Direction::Up, energized);
        }
      }
      '\\' => {
        if row < nrow - 1 {
          traverse(map, nrow, ncol, row + 1, col, Direction::Down, energized);
        }
      }
      '|' => {
        if row > 0 {
          traverse(map, nrow, ncol, row - 1, col, Direction::Up, energized);
        }

        if row < nrow - 1 {
          traverse(map, nrow, ncol, row + 1, col, Direction::Down, energized);
        }
      }
      _ => panic!("Invalid char: {}", map[idx]),
    },
  }
  energized
}

fn read_map(file: &str) -> io::Result<(Vec<char>, usize, usize)> {
  let input = fs::read_to_string(file)?;
  let nrow = input.lines().count();
  let ncol = input.lines().next().unwrap().len();
  let map = input.chars().filter(|c| *c != '\n').collect::<Vec<char>>();

  Ok((map, nrow, ncol))
}
