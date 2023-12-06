use std::fs;
use std::io;

struct Grid {
  data: Vec<char>,
  nrow: usize,
  ncol: usize,
}

impl Grid {
  fn build(file: &str) -> io::Result<Grid> {
    let input = fs::read_to_string(file)?;
    let mut data: Vec<char> = Vec::new();
    let mut nrow: usize = 0;
    let mut ncol: usize = 0;

    for line in input.lines() {
      let chars: Vec<char> = line.chars().collect();
      ncol = chars.len();
      data.extend(chars);
      nrow += 1;
    }

    Ok(Grid { data, nrow, ncol })
  }

  fn is_symbol(&self, row: i32, col: i32) -> bool {
    if row < 0 || col < 0 {
      return false;
    }
    if row >= self.nrow as i32 || col >= self.ncol as i32 {
      return false;
    }
    let idx = (row as usize)*self.ncol + (col as usize);
    if self.data[idx] != '.' && !self.data[idx].is_ascii_digit() {
      return true;
    }
    false
  }

  fn is_adjacent_to_a_symbol(&self, row: i32, start_col: i32, end_col: i32) -> bool {
    let mut adjacent;

    for c in start_col..end_col+1 {
      if c == start_col {
        adjacent = self.is_symbol(row-1, c-1) || self.is_symbol(row, c-1) || self.is_symbol(row+1, c-1);
        if adjacent {
          return true;
        }
      }

      adjacent = self.is_symbol(row-1, c) || self.is_symbol(row+1, c);
      if adjacent {
        return true;
      }

      if c == end_col {
        adjacent = self.is_symbol(row-1, c+1) || self.is_symbol(row, c+1) || self.is_symbol(row+1, c+1);
        if adjacent {
          return true;
        }
      }
    }
    
    false
  }

  fn is_ascii_digit(&self, row: i32, col: i32) -> bool {
    let idx = (row as usize)*self.ncol + (col as usize);
    self.data[idx].is_ascii_digit()
  }
}

pub fn solve() {
  println!("Part 1: {}", part1("inputs/day3/part1").unwrap());
}

fn part1(file: &str) -> io::Result<u32> {
  let grid = Grid::build(file)?;
  let mut total: u32 = 0;

  for i in 0..grid.nrow {
    let mut start_col: i32 = -1;
    for j in 0..grid.ncol {
      let is_digit = grid.is_ascii_digit(i as i32, j as i32);
      if !is_digit || j == grid.ncol-1{
        if start_col >= 0 {
          let end_col: i32 = if j == grid.ncol-1 && is_digit { j as i32} else { (j as i32) - 1 };
          if grid.is_adjacent_to_a_symbol(i as i32, start_col, end_col) {
            let s = i*grid.ncol + (start_col as usize);
            let e = i*grid.ncol + (end_col + 1) as usize;
            let str_slice: String = grid.data[s..e].iter().collect();
            // println!("number: {}", str_slice);
            if total == 0 {
              total = str_slice.parse::<u32>().unwrap();
            } else {
              total += str_slice.parse::<u32>().unwrap();
            }
          }
          start_col = -1;
        }
      } else if start_col < 0 {
        start_col = j as i32;
      }
    }
  }

  Ok(total)
}