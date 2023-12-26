use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::io;
#[derive(Debug, Clone, Copy)]
struct Brick {
  x1: usize,
  y1: usize,
  z1: usize,
  x2: usize,
  y2: usize,
  z2: usize,
}

pub fn solve() {
  println!("Part 1: {}", part1("inputs/day22/part1").unwrap());
  println!("Part 2: {}", part2("inputs/day22/part1").unwrap());
}

fn part1(file: &str) -> io::Result<usize> {
  let mut bricks: Vec<Brick> = parse_input(file)?;

  settle_down(&mut bricks);

  let (xz_map, _, ncol) = project_to_xz(&bricks);
  let (yz_map, _, _) = project_to_yz(&bricks);

  let (above, below) = get_above_and_below_map(&bricks, &xz_map, &yz_map, ncol);

  let mut cnt = 0;

  for idx in 0..bricks.len() {
    if above.get(&idx).unwrap().len() == 0 {
      cnt += 1;
    } else {
      let above_bricks = above.get(&idx).unwrap();
      let mut is_disintegratable = true;
      for above_brick_idx in above_bricks.iter() {
        if below.get(above_brick_idx).unwrap().len() <= 1 {
          is_disintegratable = false;
          break;
        }
      }
      if is_disintegratable {
        cnt += 1;
      }
    }
  }

  Ok(cnt)
}

fn part2(file: &str) -> io::Result<usize> {
  let mut bricks: Vec<Brick> = parse_input(file)?;

  settle_down(&mut bricks);

  let (xz_map, _, ncol) = project_to_xz(&bricks);
  let (yz_map, _, _) = project_to_yz(&bricks);

  let (above, below) = get_above_and_below_map(&bricks, &xz_map, &yz_map, ncol);

  let mut total = 0;
  for idx in 0..bricks.len() {
    total += count_affected_blocks(idx, &above, &mut below.clone());
  }

  Ok(total)
}

fn parse_input(file: &str) -> io::Result<Vec<Brick>> {
  let input = fs::read_to_string(file)?;
  let mut bricks: Vec<Brick> = Vec::new();
  for line in input.lines() {
    let s = line.split([',', '~']).collect::<Vec<&str>>();
    let x1 = s[0].parse::<usize>().unwrap();
    let y1 = s[1].parse::<usize>().unwrap();
    let z1 = s[2].parse::<usize>().unwrap();
    let x2 = s[3].parse::<usize>().unwrap();
    let y2 = s[4].parse::<usize>().unwrap();
    let z2 = s[5].parse::<usize>().unwrap();
    bricks.push(Brick {
      x1,
      y1,
      z1,
      x2,
      y2,
      z2,
    });
  }

  bricks.sort_by_key(|b| b.z1);

  Ok(bricks)
}

fn project_to_xz(bricks: &Vec<Brick>) -> (Vec<Vec<usize>>, usize, usize) {
  let ncol = 10;
  // get the largest z2 in bricks
  let nrow = bricks.iter().max_by_key(|b| b.z2).unwrap().z2 + 2;

  let mut map: Vec<Vec<usize>> = vec![vec![]; nrow * ncol];

  for (idx, brick) in bricks.iter().enumerate() {
    for z in brick.z1..=brick.z2 {
      for x in brick.x1..=brick.x2 {
        map[z as usize * ncol + x as usize].push(idx);
      }
    }
  }

  (map, nrow, ncol)
}

fn project_to_yz(bricks: &Vec<Brick>) -> (Vec<Vec<usize>>, usize, usize) {
  let ncol = 10;
  let nrow = bricks.iter().max_by_key(|b| b.z2).unwrap().z2 + 2;
  let mut map: Vec<Vec<usize>> = vec![vec![]; nrow * ncol];

  for (idx, brick) in bricks.iter().enumerate() {
    for z in brick.z1..=brick.z2 {
      for y in brick.y1..=brick.y2 {
        map[z as usize * ncol + y as usize].push(idx);
      }
    }
  }

  (map, nrow, ncol)
}

fn settle_down(bricks: &mut Vec<Brick>) -> bool {
  let (mut xz_map, _, ncol) = project_to_xz(&bricks);
  let (mut yz_map, _, _) = project_to_yz(&bricks);
  let mut changed = false;
  for (idx, brick) in bricks.iter_mut().enumerate() {
    // find lowest z of brick
    let mut lowest_z = brick.z1;

    while lowest_z > 1 {
      let mut below_bricks_x: HashSet<usize> = HashSet::new();
      for x in brick.x1..=brick.x2 {
        let idx = (lowest_z - 1) * ncol + x;
        for brick_idx in xz_map[idx].iter() {
          below_bricks_x.insert(*brick_idx);
        }
      }

      let mut below_bricks_y: HashSet<usize> = HashSet::new();
      for y in brick.y1..=brick.y2 {
        let idx = (lowest_z - 1) * ncol + y;
        for brick_idx in yz_map[idx].iter() {
          below_bricks_y.insert(*brick_idx);
        }
      }

      let common_below_bricks = below_bricks_x.intersection(&below_bricks_y);
      if common_below_bricks.count() > 0 {
        break;
      }
      lowest_z -= 1;
    }
    if lowest_z < brick.z1 {
      changed = true;
    }

    if changed {
      // clear xz_map
      clear_brick_from_map(idx, *brick, &mut xz_map, &mut yz_map, ncol);

      // update brick
      brick.z2 = brick.z2 - (brick.z1 - lowest_z);
      brick.z1 = lowest_z;

      // update xz_map
      put_brick_to_map(idx, *brick, &mut xz_map, &mut yz_map, ncol);
    }
  }

  bricks.sort_by_key(|b| b.z1);

  // bricks
  changed
}

fn clear_brick_from_map(
  idx: usize,
  brick: Brick,
  xz_map: &mut Vec<Vec<usize>>,
  yz_map: &mut Vec<Vec<usize>>,
  ncol: usize,
) {
  for z in brick.z1..=brick.z2 {
    for x in brick.x1..=brick.x2 {
      xz_map[z as usize * ncol + x as usize].retain(|e| *e != idx);
    }
  }

  for z in brick.z1..=brick.z2 {
    for y in brick.y1..=brick.y2 {
      yz_map[z as usize * ncol + y as usize].retain(|e| *e != idx);
    }
  }
}

fn put_brick_to_map(
  idx: usize,
  brick: Brick,
  xz_map: &mut Vec<Vec<usize>>,
  yz_map: &mut Vec<Vec<usize>>,
  ncol: usize,
) {
  for z in brick.z1..=brick.z2 {
    for x in brick.x1..=brick.x2 {
      xz_map[z as usize * ncol + x as usize].push(idx);
    }
  }

  for z in brick.z1..=brick.z2 {
    for y in brick.y1..=brick.y2 {
      yz_map[z as usize * ncol + y as usize].push(idx);
    }
  }
}

fn get_above_and_below_map(
  bricks: &Vec<Brick>,
  xz_map: &Vec<Vec<usize>>,
  yz_map: &Vec<Vec<usize>>,
  ncol: usize,
) -> (
  HashMap<usize, HashSet<usize>>,
  HashMap<usize, HashSet<usize>>,
) {
  let mut above: HashMap<usize, HashSet<usize>> = HashMap::new();
  let mut below: HashMap<usize, HashSet<usize>> = HashMap::new();

  for (idx, brick) in bricks.iter().enumerate() {
    let mut above_bricks_x: HashSet<usize> = HashSet::new();

    for x in brick.x1..=brick.x2 {
      let idx = (brick.z2 + 1) * ncol + x;
      for brick_idx in xz_map[idx].iter() {
        above_bricks_x.insert(*brick_idx);
      }
    }

    let mut above_bricks_y: HashSet<usize> = HashSet::new();
    for y in brick.y1..=brick.y2 {
      let idx = (brick.z2 + 1) * ncol + y;
      for brick_idx in yz_map[idx].iter() {
        above_bricks_y.insert(*brick_idx);
      }
    }

    let supported_bricks: HashSet<usize> = above_bricks_x
      .intersection(&above_bricks_y)
      .cloned()
      .collect();

    for supported_brick_idx in supported_bricks.iter() {
      let below_bricks = below.entry(*supported_brick_idx).or_insert(HashSet::new());
      below_bricks.insert(idx);
    }

    above.insert(idx, supported_bricks);
  }

  (above, below)
}

fn count_affected_blocks(
  idx: usize,
  above: &HashMap<usize, HashSet<usize>>,
  below: &mut HashMap<usize, HashSet<usize>>,
) -> usize {
  let mut visited: HashSet<usize> = HashSet::new();
  let mut processing_queue: VecDeque<usize> = VecDeque::new();
  let mut pending_set: HashSet<usize> = HashSet::new();

  processing_queue.push_back(idx);
  loop {
    while processing_queue.len() > 0 {
      let block_idx = processing_queue.pop_front().unwrap();
      let above_blocks = above.get(&block_idx).unwrap();
      for above_block_idx in above_blocks.iter() {
        below
          .get_mut(above_block_idx)
          .unwrap()
          .retain(|e| *e != block_idx);
        pending_set.insert(*above_block_idx);
      }
      visited.insert(block_idx);
    }

    if pending_set.len() == 0 {
      break;
    }

    processing_queue.extend(
      pending_set
        .iter()
        .filter(|p| below.get(p).unwrap().len() == 0),
    );
    pending_set.clear();
  }

  visited.len() - 1
}
