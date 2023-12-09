use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
use std::io;

struct Row {
  kind: u64,
  hand: String,
  bid: u64,
}

impl Row {
  fn new(kind: u64, hand: &str, bid: u64) -> Row {
    Row {
      kind,
      hand: String::from(hand),
      bid,
    }
  }
}

pub fn solve() {
  println!("Part 1: {}", part1("inputs/day7/part1").unwrap());
  println!("Part 2: {}", part2("inputs/day7/part1").unwrap());
}

fn part1(file: &str) -> io::Result<u64> {
  let input = fs::read_to_string(file)?;
  let mut rows: Vec<Row> = Vec::new();

  for line in input.lines() {
    let mut split = line.split_whitespace();
    let hand = split
      .next()
      .unwrap()
      .replace("A", "E")
      .replace("K", "D")
      .replace("Q", "C")
      .replace("J", "B")
      .replace("T", "A");
    let bid = split.next().unwrap().parse::<u64>().unwrap();
    let kind = get_kind(&hand);
    let row = Row::new(kind, &hand, bid);
    rows.push(row);
  }

  rows.sort_by(compare);
  let mut total = 0;
  for (idx, row) in rows.iter().enumerate() {
    total += (idx + 1) as u64 * row.bid;
  }

  Ok(total)
}

fn part2(file: &str) -> io::Result<u64> {
  let input = fs::read_to_string(file)?;
  let mut rows: Vec<Row> = Vec::new();

  for line in input.lines() {
    let mut split = line.split_whitespace();
    let hand = split
      .next()
      .unwrap()
      .replace("A", "E")
      .replace("K", "D")
      .replace("Q", "C")
      .replace("J", "0") // joker
      .replace("T", "A");
    let bid = split.next().unwrap().parse::<u64>().unwrap();
    let kind = get_kind_part_2(&hand);
    let row = Row::new(kind, &hand, bid);
    rows.push(row);
  }

  rows.sort_by(compare);
  let mut total = 0;
  for (idx, row) in rows.iter().enumerate() {
    total += (idx + 1) as u64 * row.bid;
  }

  Ok(total)
}

fn get_kind(hand: &str) -> u64 {
  let mut counts: HashMap<char, u64> = HashMap::new();
  for c in hand.chars() {
    let count = counts.entry(c).or_insert(0);
    *count += 1;
  }
  let mut is_five_of_a_kind = false;
  let mut is_four_of_a_kind = false;
  let mut has_triplet = false;
  let mut pair_count = 0;

  for (_, count) in counts {
    if count == 5 {
      is_five_of_a_kind = true;
      break;
    } else if count == 4 {
      is_four_of_a_kind = true;
      break;
    } else if count == 3 {
      has_triplet = true;
    } else if count == 2 {
      pair_count += 1;
    }
  }
  let mut kind: u64 = 0;
  if is_five_of_a_kind {
    kind = 6
  } else if is_four_of_a_kind {
    kind = 5;
  } else if has_triplet && pair_count == 1 {
    kind = 4;
  } else if has_triplet {
    kind = 3;
  } else if pair_count == 2 {
    kind = 2;
  } else if pair_count == 1 {
    kind = 1;
  }
  kind
}

fn get_kind_part_2(hand: &str) -> u64 {
  let mut counts: HashMap<char, u64> = HashMap::new();
  let mut joker_count = 0;

  let mut max_count = 0;
  let mut max_count_key: char = '0';
  for c in hand.chars() {
    if c == '0' {
      joker_count += 1;
      continue;
    }
    let count = counts.entry(c).or_insert(0);
    *count += 1;
    if *count > max_count {
      max_count = *count;
      max_count_key = c;
    }
  }
  let count = counts.entry(max_count_key).or_insert(0);
  *count += joker_count;

  let mut is_five_of_a_kind = false;
  let mut is_four_of_a_kind = false;
  let mut has_triplet = false;
  let mut pair_count = 0;

  if joker_count == 5 {
    return 6;
  }

  for (_, count) in counts {
    if count == 5 {
      is_five_of_a_kind = true;
      break;
    } else if count == 4 {
      is_four_of_a_kind = true;
      break;
    } else if count == 3 {
      has_triplet = true;
    } else if count == 2 {
      pair_count += 1;
    }
  }
  let mut kind: u64 = 0;
  if is_five_of_a_kind {
    kind = 6
  } else if is_four_of_a_kind {
    kind = 5;
  } else if has_triplet && pair_count == 1 {
    kind = 4;
  } else if has_triplet {
    kind = 3;
  } else if pair_count == 2 {
    kind = 2;
  } else if pair_count == 1 {
    kind = 1;
  }
  kind
}

fn compare(a: &Row, b: &Row) -> Ordering {
  if a.kind == b.kind {
    return a.hand.cmp(&b.hand);
  }
  a.kind.cmp(&b.kind)
}
