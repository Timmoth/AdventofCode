use std::{env, fs::read_to_string, time::Instant};

mod solution_01_part1;
mod solution_01_part2;
mod solution_02_part1;
mod solution_02_part2;
mod solution_03_part1;
mod solution_03_part2;
mod solution_04_part1;
mod solution_04_part2;
mod solution_05_part1;
mod solution_05_part2;

fn main() {
    let args:Vec<String> = env::args().collect();

    if args.len() != 3 && args.len() != 4 {
        eprintln!("Usage: cargo run --release <day> <part> <test|actual>");
        std::process::exit(1);
    }

    let input_file =  if args.len() == 4 {
        args[3].as_str()
    }else{
        "actual"
    };

    let day: i32 = args[1].parse().expect("Please provide a day number.");
    let part: i32 = args[2].parse().expect("Please provide a part number.");
    let file_path = format!("./inputs/{day}_{input_file}.txt");
    let input = read_to_string(&file_path).expect(format!("Couldn't find input file '{file_path}'").as_str());
    let input = input.as_str();

    let start = Instant::now();
    match (day, part) {
        (1, 1) => solution_01_part1::run(input),
        (1, 2) => solution_01_part2::run(input),
        (2, 1) => solution_02_part1::run(input),
        (2, 2) => solution_02_part2::run(input),
        (3, 1) => solution_03_part1::run(input),
        (3, 2) => solution_03_part2::run(input),
        (4, 1) => solution_04_part1::run(input),
        (4, 2) => solution_04_part2::run(input),
        (5, 1) => solution_05_part1::run(input),
        (5, 2) => solution_05_part2::run(input),
        _ => {
            println!("Solution for day {} part {} not found.", day, part);
            std::process::exit(1);
        }
    }

    let duration = start.elapsed(); // Get the elapsed time
    println!("execution time: {:?}", duration);
}

