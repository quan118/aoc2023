use std::fs;
use std::io;

#[derive(Debug, Clone, Copy)]
struct Point {
  x: i64,
  y: i64,
}

pub fn solve() {
  println!("Part 1: {}", part1("inputs/day18/part1").unwrap());
  println!("Part 2: {}", part2("inputs/day18/part1").unwrap());
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

  let points = build_points(&instructions);

  Ok(calculate_cubic_meters(&points))
}

fn part2(file: &str) -> io::Result<i64> {
  let input = fs::read_to_string(file)?;
  let mut instructions: Vec<(char, i64)> = Vec::new();
  for line in input.lines() {
    let last = line.split_whitespace().last().unwrap();
    let hex_code = last[2..last.len() - 1].to_string();
    let steps = &hex_code[..hex_code.len() - 1];
    let steps = i64::from_str_radix(steps, 16).unwrap();
    let dir = hex_code.chars().last().unwrap();
    // println!("{} {}", dir, steps);
    let dir = match dir {
      '0' => 'R',
      '1' => 'D',
      '2' => 'L',
      '3' => 'U',
      _ => panic!("Invalid direction"),
    };
    instructions.push((dir, steps));
  }

  let points = build_points(&instructions);

  Ok(calculate_cubic_meters(&points))
}

fn build_points(instructions: &Vec<(char, i64)>) -> Vec<Point> {
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

  points
}

fn calculate_cubic_meters(points: &Vec<Point>) -> i64 {
  // using shoelace formula to calculate area
  let mut area: i64 = 0;
  for i in 0..points.len() {
    let j = if i == points.len() - 1 { 0 } else { i + 1 };
    area += points[i].x * points[j].y - points[j].x * points[i].y;
  }
  area = area.abs() / 2;

  let perimeter = get_perimeter(&points);
  // using Pick's theorem to get inner points
  let inner_points = area - perimeter / 2 + 1;

  perimeter + inner_points
}

fn get_perimeter(points: &Vec<Point>) -> i64 {
  let mut perimeter: i64 = 0;
  for i in 0..points.len() {
    let j = if i == points.len() - 1 { 0 } else { i + 1 };
    perimeter += (points[i].x - points[j].x).abs() + (points[i].y - points[j].y).abs();
  }
  perimeter
}
