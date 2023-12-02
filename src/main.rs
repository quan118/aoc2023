mod day1;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("day1") => day1::solve(),
        _ => println!("Please specify a valid day"),
    }
}
