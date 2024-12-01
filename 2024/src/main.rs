use std::{env, time::Instant};

mod solution_01_part1;
mod solution_01_part2;

fn main() {
    let args:Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: cargo run <day> <part>");
        std::process::exit(1);
    }

    let day: i32 = args[1].parse().expect("Please provide a day number.");
    let part: i32 = args[2].parse().expect("Please provide a part number.");

    let start = Instant::now();
    match (day, part) {
        (1, 1) => solution_01_part1::run(),
        (1, 2) => solution_01_part2::run(),

        _ => {
            println!("Solution for day {} part {} not found.", day, part);
            std::process::exit(1);
        }
    }

    let duration = start.elapsed(); // Get the elapsed time
    println!("execution time: {:?}", duration);
}
