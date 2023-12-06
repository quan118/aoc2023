mod day1;
mod day2;
mod day3;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("day1") => day1::solve(),
        Some("day2") => day2::solve(),
        Some("day3") => day3::solve(),
        _ => println!("Please specify a valid day"),
    }
}
