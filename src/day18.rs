use std::collections::HashMap;
use std::fs;
use std::io;

#[derive(Eq, PartialEq, Debug, Clone, Copy, Hash)]
struct Point {
  x: i64,
  y: i64,
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
enum State {
  Out,
  In,
  InEdge,
}

pub fn solve() {
  println!("Part 1: {}", part1("inputs/day18/part1").unwrap());
}

fn part1(file: &str) -> io::Result<i64> {
  let input = fs::read_to_string(file)?;
  let mut instructions: Vec<(char, i64)> = Vec::new();
  for line in input.lines() {
    let s = line.split_whitespace().collect::<Vec<&str>>();
    let dir = s[0].chars().next().unwrap();
    let steps = s[1].parse::<i64>().unwrap();
    instructions.push((dir, steps));
  }

  let mut points: Vec<Point> = Vec::new();
  let mut x: i64 = 0;
  let mut y: i64 = 0;

  points.push(Point { x, y });

  for ins in instructions.iter() {
    match ins.0 {
      'R' => {
        x += ins.1;
        points.push(Point { x, y });
      }
      'D' => {
        y += ins.1;
        points.push(Point { x, y });
      }
      'L' => {
        x -= ins.1;
        points.push(Point { x, y });
      }
      'U' => {
        y -= ins.1;
        points.push(Point { x, y });
      }
      _ => {}
    }
  }
  points.pop(); // Remove the last item in points

  let smallest_x = points.iter().min_by_key(|p| p.x).unwrap().x;
  let smallest_y = points.iter().min_by_key(|p| p.y).unwrap().y;
  points.iter_mut().for_each(|p| {
    p.x -= smallest_x;
    p.y -= smallest_y;
  });

  // build point to index mapping
  let mut point_to_idx: HashMap<Point, usize> = HashMap::new();
  for (idx, p) in points.iter().enumerate() {
    point_to_idx.insert(*p, idx);
  }

  let largest_x = points.iter().max_by_key(|p| p.x).unwrap().x;
  let largest_y = points.iter().max_by_key(|p| p.y).unwrap().y;
  let width = largest_x + 1;
  let height = largest_y + 1;
  const EMPTY: i64 = 0;
  const VERTEX: i64 = 1;
  const EDGE: i64 = 2;
  let mut grid: Vec<i64> = vec![EMPTY; ((largest_x + 1) * (largest_y + 1)) as usize];

  let mut x = points[0].x;
  let mut y = points[0].y;

  for ins in instructions.iter() {
    match ins.0 {
      'R' => {
        for i in 0..ins.1 {
          grid[(y * width + x + i) as usize] = if i == 0 { VERTEX } else { EDGE };
        }
        x += ins.1;
      }
      'D' => {
        for i in 0..ins.1 {
          grid[((y + i) * width + x) as usize] = if i == 0 { VERTEX } else { EDGE };
        }
        y += ins.1;
      }
      'L' => {
        for i in 0..ins.1 {
          grid[(y * width + x - i) as usize] = if i == 0 { VERTEX } else { EDGE };
        }
        x -= ins.1;
      }
      'U' => {
        for i in 0..ins.1 {
          grid[((y - i) * width + x) as usize] = if i == 0 { VERTEX } else { EDGE };
        }
        y -= ins.1;
      }
      _ => {}
    }
  }

  let mut total: i64 = 0;

  for y in 0..height {
    let mut state = State::Out;
    let mut prev_state = State::Out;
    let mut has_top_neighbor1 = false;
    let mut has_top_neighbor2 = false;
    for x in 0..width {
      let tile = grid[(y * width + x) as usize];
      if tile != EMPTY {
        total += 1;
        match state {
          State::Out => {
            if tile == EDGE {
              state = State::In;
            } else {
              state = State::InEdge;
              prev_state = State::Out;
              let idx = point_to_idx.get(&Point { x, y }).unwrap();
              has_top_neighbor1 = has_top_neighbor(&points, *idx);
            }
          }
          State::In => {
            if tile == EDGE {
              state = State::Out;
            } else {
              state = State::InEdge;
              prev_state = State::In;
              let idx = point_to_idx.get(&Point { x, y }).unwrap();
              has_top_neighbor1 = has_top_neighbor(&points, *idx);
            }
          }
          State::InEdge => {
            if tile == EDGE {
              state = State::InEdge;
            } else {
              let idx = point_to_idx.get(&Point { x, y }).unwrap();

              has_top_neighbor2 = has_top_neighbor(&points, *idx);
              if has_top_neighbor1 != has_top_neighbor2 {
                state = if prev_state == State::In {
                  State::Out
                } else {
                  State::In
                };
              } else {
                state = prev_state;
              }
            }
          }
        }
      } else {
        if state == State::In {
          total += 1;
        }
      }
    }
  }

  Ok(total)
}

fn has_top_neighbor(points: &Vec<Point>, idx: usize) -> bool {
  let prev_idx = if idx == 0 { points.len() - 1 } else { idx - 1 };
  let next_idx = if idx == points.len() - 1 { 0 } else { idx + 1 };

  if points[prev_idx].x == points[idx].x && points[prev_idx].y < points[idx].y {
    return true;
  }

  if points[next_idx].x == points[idx].x && points[next_idx].y < points[idx].y {
    return true;
  }

  false
}
