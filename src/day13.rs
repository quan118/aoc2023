use std::fs;
use std::io;

pub fn solve() {
  println!("Part 1: {}", part1("inputs/day13/part1").unwrap());
  println!("Part 2: {}", part2("inputs/day13/part1").unwrap());
}

fn part1(file: &str) -> io::Result<u32> {
  let input = fs::read_to_string(file)?;

  let mut total: u32 = 0;
  let mut rows: Vec<String> = Vec::new();
  for line in input.lines() {
    if line.trim() == "" {
      if let Some(row_mirror) = find_mirror(&rows) {
        total += (row_mirror + 1) * 100;
      } else {
        let cols = get_cols(&rows);
        if let Some(col_mirror) = find_mirror(&cols) {
          total += (col_mirror + 1);
        }
      }
      rows.clear();
      continue;
    }
    rows.push(line.to_string());
  }

  if let Some(row_mirror) = find_mirror(&rows) {
    total += (row_mirror + 1) * 100;
  } else {
    let cols = get_cols(&rows);
    if let Some(col_mirror) = find_mirror(&cols) {
      total += (col_mirror + 1);
    }
  }
  Ok(total)
}

fn part2(file: &str) -> io::Result<u32> {
  let input = fs::read_to_string(file)?;

  let mut total: u32 = 0;
  let mut rows: Vec<String> = Vec::new();
  for line in input.lines() {
    if line.trim() == "" {
      if let Some(row_mirror) = find_smudged_mirror(&rows) {
        total += (row_mirror + 1) * 100;
      } else {
        let cols = get_cols(&rows);
        if let Some(col_mirror) = find_smudged_mirror(&cols) {
          total += (col_mirror + 1);
        }
      }
      rows.clear();
      continue;
    }
    rows.push(line.to_string());
  }

  if let Some(row_mirror) = find_smudged_mirror(&rows) {
    total += (row_mirror + 1) * 100;
  } else {
    let cols = get_cols(&rows);
    if let Some(col_mirror) = find_smudged_mirror(&cols) {
      total += (col_mirror + 1);
    }
  }
  Ok(total)
}

fn get_cols(rows: &Vec<String>) -> Vec<String> {
  let mut cols: Vec<String> = Vec::new();
  for row in rows {
    let chars = row.chars();
    for i in 0..row.len() {
      if cols.len() <= i {
        cols.push(String::new());
      }
      cols[i].push(row.chars().nth(i).unwrap());
    }
  }
  cols
}

fn find_mirror(strs: &Vec<String>) -> Option<u32> {
  for i in 0..strs.len() - 1 {
    let mut found_mirror = true;
    for j in i + 1..((i + 1) * 2).min(strs.len()) {
      if strs[j] != strs[i - (j - i - 1)] {
        found_mirror = false;
        break;
      }
    }
    if found_mirror {
      return Some(i as u32);
    }
  }

  return None;
}

fn find_smudged_mirror(strs: &Vec<String>) -> Option<u32> {
  let mut mirror_pos: i32 = -1;

  // find diff_pos and mirror_pos
  for i in 0..strs.len() - 1 {
    let mut count_diff = 0;
    let mut diff_pos: i32 = -1;
    for j in i + 1..((i + 1) * 2).min(strs.len()) {
      if strs[j] != strs[i - (j - i - 1)] {
        if count_diff == 0 {
          count_diff += 1;
          diff_pos = j as i32;
        } else {
          count_diff += 1;
          break;
        }
      }
    }
    if count_diff == 1 {
      let diff_pos = diff_pos as usize;
      let mut count_diff_in_2_strings: u32 = 0;
      let chars1 = strs[diff_pos].chars().collect::<Vec<char>>();
      let chars2 = strs[i - (diff_pos - i - 1)].chars().collect::<Vec<char>>();
      for i in 0..chars1.len() {
        if chars1[i] != chars2[i] {
          count_diff_in_2_strings += 1;
          if count_diff_in_2_strings > 1 {
            break;
          }
        }
      }
      if count_diff_in_2_strings == 1 {
        mirror_pos = i as i32;
        break;
      }
    }
  }

  if mirror_pos < 0 {
    return None;
  }

  Some(mirror_pos as u32)
}
