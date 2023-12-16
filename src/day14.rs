use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::io;

pub fn solve() {
  println!("Part 1: {}", part1("inputs/day14/part1").unwrap());
  println!("Part 2: {}", part2("inputs/day14/part1").unwrap());
}

fn part1(file: &str) -> io::Result<u64> {
  let (mut map, nrow, ncol) = read_map(file)?;
  up(&mut map, nrow, ncol);
  let total = get_total_load(&map, nrow, ncol);
  Ok(total)
}

fn part2(file: &str) -> io::Result<u64> {
  let (original_map, nrow, ncol) = read_map(file)?;
  let mut all_maps: HashSet<String> = HashSet::new();
  let mut maps_to_index: HashMap<String, usize> = HashMap::new();
  let mut map = original_map.clone();
  let mut loop_start: u64 = 0;
  let mut loop_end: u64 = 0;
  const CYCLES_TIMES: usize = 1000000000;
  for i in 0..CYCLES_TIMES {
    tilt_1_cycle(&mut map, nrow, ncol);
    let map_str = map.iter().collect::<String>();
    if all_maps.contains(&map_str) {
      loop_end = i as u64 - 1;
      loop_start = *maps_to_index.get(&map_str).unwrap() as u64;
      break;
    }
    all_maps.insert(map_str.clone());
    maps_to_index.insert(map_str.clone(), i);
  }

  let times = loop_start + ((CYCLES_TIMES as u64 - loop_start) % (loop_end - loop_start + 1));
  let mut map = original_map.clone();
  for _ in 0..times {
    tilt_1_cycle(&mut map, nrow, ncol);
  }

  let total = get_total_load(&map, nrow, ncol);

  Ok(total)
}

fn get_total_load(map: &Vec<char>, nrow: usize, ncol: usize) -> u64 {
  let mut total: u64 = 0;
  for r in 0..nrow {
    for c in 0..ncol {
      let tile = map[r * ncol + c];
      if tile == 'O' {
        total += (nrow - r) as u64;
      }
    }
  }
  total
}

fn up(map: &mut Vec<char>, nrow: usize, ncol: usize) -> &Vec<char> {
  for c in 0..ncol {
    let mut border = 0;
    for r in 0..nrow {
      let idx = r * ncol + c;
      let tile = map[idx];
      if tile == '#' {
        border = r + 1;
      } else if tile == 'O' {
        // move 'O' to the top
        map[border * ncol + c] = 'O';
        if border != r {
          map[idx] = '.';
        }
        border += 1;
      }
    }
  }
  map
}

fn down(map: &mut Vec<char>, nrow: usize, ncol: usize) -> &Vec<char> {
  for c in 0..ncol {
    let mut border: i32 = nrow as i32 - 1;
    for r in (0..nrow).rev() {
      let idx = r * ncol + c;
      let tile = map[idx];
      if tile == '#' {
        border = r as i32 - 1;
      } else if tile == 'O' {
        // move 'O' to the bottom
        map[border as usize * ncol + c] = 'O';
        if border != r as i32 {
          map[idx] = '.';
        }
        border -= 1;
      }
    }
  }
  map
}

fn left(map: &mut Vec<char>, nrow: usize, ncol: usize) -> &Vec<char> {
  for r in 0..nrow {
    let mut border = 0;
    for c in 0..ncol {
      let idx = r * ncol + c;
      let tile = map[idx];
      if tile == '#' {
        border = c + 1;
      } else if tile == 'O' {
        // move 'O' to the left
        map[r * ncol + border] = 'O';
        if border != c {
          map[idx] = '.';
        }
        border += 1;
      }
    }
  }
  map
}

fn right(map: &mut Vec<char>, nrow: usize, ncol: usize) -> &Vec<char> {
  for r in 0..nrow {
    let mut border: i32 = ncol as i32 - 1;
    for c in (0..ncol).rev() {
      let idx = r * ncol + c;
      let tile = map[idx];
      if tile == '#' {
        border = c as i32 - 1;
      } else if tile == 'O' {
        // move 'O' to the right
        map[r * ncol + border as usize] = 'O';
        if border != c as i32 {
          map[idx] = '.';
        }
        border -= 1;
      }
    }
  }
  map
}

fn tilt_1_cycle(map: &mut Vec<char>, nrow: usize, ncol: usize) -> &Vec<char> {
  up(map, nrow, ncol);
  left(map, nrow, ncol);
  down(map, nrow, ncol);
  right(map, nrow, ncol);

  map
}

fn read_map(file: &str) -> io::Result<(Vec<char>, usize, usize)> {
  let input = fs::read_to_string(file)?;
  let nrow = input.lines().count();
  let ncol = input.lines().next().unwrap().len();
  let map = input.chars().filter(|c| *c != '\n').collect::<Vec<char>>();

  Ok((map, nrow, ncol))
}
