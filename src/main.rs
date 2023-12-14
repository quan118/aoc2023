mod day1;
mod day10;
mod day11;
mod day12;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
  let args: Vec<String> = std::env::args().collect();
  match args.get(1).map(|s| s.as_str()) {
    Some("day1") => day1::solve(),
    Some("day2") => day2::solve(),
    Some("day3") => day3::solve(),
    Some("day4") => day4::solve(),
    Some("day5") => day5::solve(),
    Some("day6") => day6::solve(),
    Some("day7") => day7::solve(),
    Some("day8") => day8::solve(),
    Some("day9") => day9::solve(),
    Some("day10") => day10::solve(),
    Some("day11") => day11::solve(),
    Some("day12") => day12::solve(),
    _ => println!("Please specify a valid day"),
  }
}
