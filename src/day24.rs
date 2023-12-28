use std::fs;
use std::io;

extern crate nalgebra as na;

#[derive(Debug, Clone, Copy)]
struct Hailstone {
  x: i64,
  y: i64,
  z: i64,
  vx: i64,
  vy: i64,
  vz: i64,
}

#[derive(Debug, Clone, Copy)]
struct CollisionResult {
  x: f64,
  y: f64,
  t1: f64,
  t2: f64,
}

pub fn solve() {
  println!("Part 1: {}", part1("inputs/day24/part1").unwrap());
  println!("Part 2: {}", part2("inputs/day24/part1").unwrap());
}

fn part1(file: &str) -> io::Result<u64> {
  let stones: Vec<Hailstone> = parse_file(file)?;

  let bound_min = 200000000000000.0;
  let bound_max = 400000000000000.0;
  let mut cnt: u64 = 0;
  for i in 0..stones.len() - 1 {
    for j in i + 1..stones.len() {
      if let Some(result) = get_collide_positions_ignore_time(&stones[i], &stones[j]) {
        if bound_min <= result.x
          && result.x <= bound_max
          && bound_min <= result.y
          && result.y <= bound_max
        {
          cnt += 1;
        }
      }
    }
  }

  Ok(cnt)
}

fn part2(file: &str) -> io::Result<u64> {
  let stones: Vec<Hailstone> = parse_file(file)?;

  let mut a: Vec<i64> = vec![0; 36];
  let mut b: Vec<i64> = vec![0; 6];

  for i in 1..4 {
    let r = (i - 1) * 2;
    a[r * 6 + 0] = stones[0].vy - stones[i].vy;
    a[r * 6 + 1] = -stones[0].vx + stones[i].vx;
    a[r * 6 + 2] = 0;
    a[r * 6 + 3] = -stones[0].y + stones[i].y;
    a[r * 6 + 4] = stones[0].x - stones[i].x;
    a[r * 6 + 5] = 0;
    b[r] = stones[0].x * stones[0].vy - stones[i].x * stones[i].vy - stones[0].y * stones[0].vx
      + stones[i].y * stones[i].vx;

    a[(r + 1) * 6 + 0] = stones[0].vz - stones[i].vz;
    a[(r + 1) * 6 + 1] = 0;
    a[(r + 1) * 6 + 2] = -stones[0].vx + stones[i].vx;
    a[(r + 1) * 6 + 3] = -stones[0].z + stones[i].z;
    a[(r + 1) * 6 + 4] = 0;
    a[(r + 1) * 6 + 5] = stones[0].x - stones[i].x;
    b[r + 1] = stones[0].x * stones[0].vz - stones[i].x * stones[i].vz - stones[0].z * stones[0].vx
      + stones[i].z * stones[i].vx;
  }

  let A = na::DMatrix::from_row_slice(6, 6, &a.iter().map(|&x| x as f64).collect::<Vec<f64>>());
  let B = na::DVector::from_row_slice(&b.iter().map(|&x| x as f64).collect::<Vec<f64>>());

  let mut total: u64 = 0;

  match A.try_inverse() {
    Some(inv) => {
      let x = inv * B;
      total = (x[0] + x[1] + x[2]) as u64;
    }
    None => {
      println!("A is not invertible");
    }
  }

  Ok(total)
}

fn get_collide_positions_ignore_time(
  stone1: &Hailstone,
  stone2: &Hailstone,
) -> Option<CollisionResult> {
  let a = stone1.vx;
  let b = -stone2.vx;
  let c = stone1.vy;
  let d = -stone2.vy;
  let u = stone2.x - stone1.x;
  let v = stone2.y - stone1.y;

  if let Some((t1, t2)) =
    solve_linear_system(a as f64, b as f64, c as f64, d as f64, u as f64, v as f64)
  {
    if t1 < 0.0 || t2 < 0.0 {
      return None;
    }

    return Some(CollisionResult {
      x: stone1.x as f64 + stone1.vx as f64 * t1,
      y: stone1.y as f64 + stone1.vy as f64 * t1,
      t1: t1,
      t2: t2,
    });
  }
  None
}

fn parse_file(file: &str) -> io::Result<Vec<Hailstone>> {
  let input = fs::read_to_string(file)?;

  Ok(
    input
      .lines()
      .map(|line| {
        let parts = line.split([',', '@']).collect::<Vec<&str>>();
        Hailstone {
          x: parts[0].trim().parse::<i64>().unwrap(),
          y: parts[1].trim().parse::<i64>().unwrap(),
          z: parts[2].trim().parse::<i64>().unwrap(),
          vx: parts[3].trim().parse::<i64>().unwrap(),
          vy: parts[4].trim().parse::<i64>().unwrap(),
          vz: parts[5].trim().parse::<i64>().unwrap(),
        }
      })
      .collect(),
  )
}

// A = [[a, b], [c, d]] B = [[u], [v]]. Solve X = [[t1], [t2]] so that AX = B
fn solve_linear_system(a: f64, b: f64, c: f64, d: f64, u: f64, v: f64) -> Option<(f64, f64)> {
  let determinant = a * d - b * c;

  // Check if the determinant is zero
  if determinant == 0.0 {
    return None;
  }

  // Calculate t1 and t2
  let t1 = (-b * v + d * u) / determinant;
  let t2 = (a * v - c * u) / determinant;

  Some((t1, t2))
}
