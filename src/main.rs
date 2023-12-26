mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
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
    Some("day13") => day13::solve(),
    Some("day14") => day14::solve(),
    Some("day15") => day15::solve(),
    Some("day16") => day16::solve(),
    Some("day17") => day17::solve(),
    Some("day18") => day18::solve(),
    Some("day19") => day19::solve(),
    Some("day20") => day20::solve(),
    Some("day21") => day21::solve(),
    Some("day22") => day22::solve(),
    _ => println!("Please specify a valid day"),
  }
}
