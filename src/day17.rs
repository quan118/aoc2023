use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fs;
use std::io;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum Direction {
  Up,
  Down,
  Left,
  Right,
}

impl Direction {
  fn get_drow_dcol(&self) -> (i64, i64) {
    match self {
      Direction::Up => (-1, 0),
      Direction::Down => (1, 0),
      Direction::Left => (0, -1),
      Direction::Right => (0, 1),
    }
  }

  fn get_cross_directions(&self) -> Vec<Direction> {
    match self {
      Direction::Up | Direction::Down => [Direction::Left, Direction::Right].to_vec(),
      Direction::Left | Direction::Right => [Direction::Up, Direction::Down].to_vec(),
    }
  }
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
struct Pos {
  row: usize,
  col: usize,
}

#[derive(Debug)]
struct State {
  heat_loss: i64,
  dir: Direction,
  pos: Pos,
}

impl Eq for State {}

impl PartialEq for State {
  fn eq(&self, other: &Self) -> bool {
    self.heat_loss == other.heat_loss
  }
}

impl Ord for State {
  fn cmp(&self, other: &Self) -> Ordering {
    other.heat_loss.cmp(&self.heat_loss)
  }
}

impl PartialOrd for State {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

pub fn solve() {
  println!("Part 1: {}", part1("inputs/day17/part1").unwrap());
  println!("Part 2: {}", part2("inputs/day17/part1").unwrap());
}

fn part1(file: &str) -> io::Result<i64> {
  let (map, nrow, ncol) = read_map(file)?;

  Ok(find_least_heat_loss(&map, nrow, ncol, 0, 3))
}

fn part2(file: &str) -> io::Result<i64> {
  let (map, nrow, ncol) = read_map(file)?;

  Ok(find_least_heat_loss(&map, nrow, ncol, 4, 10))
}

fn find_least_heat_loss(
  map: &Vec<i64>,
  nrow: usize,
  ncol: usize,
  min_streak: usize,
  max_streak: usize,
) -> i64 {
  let mut queue: BinaryHeap<State> = BinaryHeap::new();
  let mut heat_losses: Vec<HashMap<Direction, i64>> = vec![HashMap::new(); map.len()];

  let down = State {
    heat_loss: 0,
    dir: Direction::Down,
    pos: Pos { row: 0, col: 0 },
  };
  heat_losses[nrow].insert(Direction::Down, map[nrow]);
  queue.push(down);

  let right = State {
    heat_loss: 0,
    dir: Direction::Right,
    pos: Pos { row: 0, col: 0 },
  };
  heat_losses[1].insert(Direction::Right, map[1]);
  queue.push(right);

  while let Some(minimum_state) = queue.pop() {
    let mut streak_cnt = 1;
    let drow_dcol = minimum_state.dir.get_drow_dcol();
    let mut row = minimum_state.pos.row as i64 + drow_dcol.0;
    let mut col = minimum_state.pos.col as i64 + drow_dcol.1;
    let mut heat_loss = minimum_state.heat_loss;

    while streak_cnt < min_streak && 0 <= col && col < ncol as i64 && 0 <= row && row < nrow as i64
    {
      heat_loss += map[row as usize * ncol + col as usize];
      row += drow_dcol.0;
      col += drow_dcol.1;
      streak_cnt += 1;
    }

    while streak_cnt <= max_streak && 0 <= col && col < ncol as i64 && 0 <= row && row < nrow as i64
    {
      let mut idx = row as usize * ncol + col as usize;
      heat_loss += map[idx];

      for dir in minimum_state.dir.get_cross_directions() {
        if heat_loss >= *heat_losses[idx].get(&dir).unwrap_or(&i64::MAX) {
          continue;
        }
        heat_losses[idx].insert(dir, heat_loss);
        let new_state = State {
          heat_loss,
          dir,
          pos: Pos {
            row: row as usize,
            col: col as usize,
          },
        };
        queue.push(new_state);
      }

      row += drow_dcol.0;
      col += drow_dcol.1;
      streak_cnt += 1;
    }
  }

  heat_losses[map.len() - 1].values().min().unwrap().clone()
}

fn read_map(file: &str) -> io::Result<(Vec<i64>, usize, usize)> {
  let input = fs::read_to_string(file)?;
  let nrow = input.lines().count();
  let ncol = input.lines().next().unwrap().len();
  let map = input
    .chars()
    .filter(|c| *c != '\n')
    .map(|c| c as u8 as i64 - 48)
    .collect::<Vec<i64>>();

  Ok((map, nrow, ncol))
}
