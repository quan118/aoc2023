use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::io;

const EMPTY: u64 = u64::MAX;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Range {
  src: u64,
  dst: u64,
}

pub fn solve() {
  println!("Part 1: {}", part1("inputs/day5/part1").unwrap());
  println!("Part 2: {}", part2("inputs/day5/part1").unwrap());
}

fn part1(file: &str) -> io::Result<u64> {
  let (seeds, maps) = get_seeds_and_maps(file);
  let final_mapping: Vec<Range> = maps.iter().fold(Vec::new(), |acc, map| merge(&acc, map));

  let mut lowest_output = u64::MAX;

  for seed in seeds {
    let output = get_output(seed, &final_mapping);
    lowest_output = cmp::min(lowest_output, output);
  }

  Ok(lowest_output)
}

fn part2(file: &str) -> io::Result<u64> {
  let (seeds, maps) = get_seeds_and_maps(file);
  let final_mapping: Vec<Range> = maps.iter().fold(Vec::new(), |acc, map| merge(&acc, map));

  let mut lowest_output = u64::MAX;
  let pairs: Vec<(u64, u64)> = seeds.chunks(2).map(|chunk| (chunk[0], chunk[1])).collect();

  for pair in pairs {
    let srcs = get_srcs_in_range(pair.0, pair.1, &final_mapping);
    for src in srcs {
      let output = get_output(src, &final_mapping);
      lowest_output = cmp::min(lowest_output, output);
    }
  }
  Ok(lowest_output)
}

fn get_seeds_and_maps(file: &str) -> (Vec<u64>, Vec<Vec<Range>>) {
  let input = fs::read_to_string(file).unwrap();
  let mut seeds: Vec<u64> = Vec::new();
  let mut maps: Vec<Vec<Range>> = Vec::new();
  let mut tmp_src_to_dst: HashMap<u64, u64> = HashMap::new();

  for line in input.lines() {
    if line.starts_with("seeds:") {
      let mut split = line.split(":");
      split.next(); // skip the `seeds` part
      let seeds_str = split.next().unwrap();
      seeds = seeds_str
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    } else if line.ends_with("map:") {
      if tmp_src_to_dst.len() > 0 {
        let mut mapping_vec = tmp_src_to_dst
          .iter()
          .map(|(src, dst)| Range {
            src: *src,
            dst: if *dst == EMPTY { *src } else { *dst },
          })
          .collect::<Vec<Range>>();
        mapping_vec.sort_by(|a, b| a.src.cmp(&b.src));
        maps.push(mapping_vec);
      }
      tmp_src_to_dst.clear();
    } else if line.len() > 0 {
      let mut split = line.split_whitespace();
      let dst = split.next().unwrap().parse::<u64>().unwrap();
      let src = split.next().unwrap().parse::<u64>().unwrap();
      let len = split.next().unwrap().parse::<u64>().unwrap();
      tmp_src_to_dst.insert(src, dst);

      if tmp_src_to_dst.get(&(src + len)) == None {
        tmp_src_to_dst.insert(src + len, EMPTY);
      }
    }
  }
  if tmp_src_to_dst.len() > 0 {
    let mut mapping_vec = tmp_src_to_dst
      .iter()
      .map(|(src, dst)| Range {
        src: *src,
        dst: if *dst == EMPTY { *src } else { *dst },
      })
      .collect::<Vec<Range>>();
    mapping_vec.sort_by(|a, b| a.src.cmp(&b.src));
    maps.push(mapping_vec);
  }

  (seeds, maps)
}

fn merge(sorted_a: &Vec<Range>, sorted_b: &Vec<Range>) -> Vec<Range> {
  // build a set of all src
  let mut src_set: HashSet<u64> = HashSet::new();
  for range in sorted_a {
    src_set.insert(range.src);
  }
  for range in sorted_b {
    src_set.insert(range.src);
  }
  for range_b in sorted_b {
    for (idx, range_a) in sorted_a.iter().enumerate() {
      if idx == sorted_a.len() - 1 {
        break;
      }
      let len = sorted_a[idx + 1].src - sorted_a[idx].src;
      if range_a.dst < range_b.src && range_b.src < range_a.dst + len {
        src_set.insert(range_b.src - range_a.dst + range_a.src);
      }
    }
  }

  // get the sorted list of src
  let mut src_vec = src_set.iter().copied().collect::<Vec<u64>>();
  src_vec.sort();

  // build the mappping
  let mut output: Vec<Range> = Vec::new();
  for src in src_vec {
    let dst = get_output(get_output(src, sorted_a), sorted_b);
    output.push(Range { src, dst });
  }

  output
}

fn get_output(input: u64, sorted_mapping: &Vec<Range>) -> u64 {
  if sorted_mapping.len() == 0 {
    return input;
  }
  let mut output: u64 = input;
  // using binary search to find the output
  let mut start: usize = 0;
  let mut end = sorted_mapping.len() - 1;
  let mut mid: usize = (start + end) / 2;
  while start <= end {
    // println!("start: {}, mid: {}, end: {}", start, mid, end);
    if mid == 0 && input < sorted_mapping[mid].src {
      break;
    }
    if mid == sorted_mapping.len() - 1 && sorted_mapping[mid].src <= input {
      break;
    }

    if sorted_mapping[mid].src <= input && input < sorted_mapping[mid + 1].src {
      output = sorted_mapping[mid].dst + (input - sorted_mapping[mid].src);
      break;
    } else if input < sorted_mapping[mid].src {
      end = mid - 1;
    } else {
      start = mid + 1;
    }
    mid = (start + end) / 2;
  }

  output
}

fn get_srcs_in_range(from: u64, len: u64, ranges: &Vec<Range>) -> Vec<u64> {
  let mut srcs: Vec<u64> = vec![from];
  for range in ranges {
    if from + len < range.src {
      break;
    }

    if from < range.src && range.src < from + len {
      srcs.push(range.src);
    }
  }
  srcs.push(from + len);
  srcs
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_get_output() {
    let mut sorted_mapping: Vec<Mapping> = Vec::new();
    sorted_mapping.push(Mapping { src: 50, dest: 52 });
    sorted_mapping.push(Mapping { src: 98, dest: 50 });
    sorted_mapping.push(Mapping {
      src: 100,
      dest: 100,
    });
    assert_eq!(get_output(0, &sorted_mapping), 0);
    assert_eq!(get_output(40, &sorted_mapping), 40);
    assert_eq!(get_output(50, &sorted_mapping), 52);
    assert_eq!(get_output(52, &sorted_mapping), 54);
    assert_eq!(get_output(98, &sorted_mapping), 50);
    assert_eq!(get_output(99, &sorted_mapping), 51);
    assert_eq!(get_output(100, &sorted_mapping), 100);
    assert_eq!(get_output(101, &sorted_mapping), 101);
  }
}
