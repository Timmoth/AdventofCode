use std::fs::read_to_string;

pub fn run() {
    println!("Executing day 7 part 2");

    let file_path = "./inputs/07_actual.txt";
    let input = read_to_string(file_path).expect("Failed to read the file");

    let numbers: Vec<i64> = input
        .split(',')
        .filter_map(|val| val.trim().parse::<i64>().ok()) // Parse and filter out invalid values
        .collect();
    let len = numbers.len();

    let mean: f64 = (numbers.iter().sum::<i64>() as f64 / len as f64).floor();

   let fuel_cost: f64 = numbers.iter().map(|&d| {
        let n = (d as f64 - mean).abs();
        n * (n + 1.0) / 2.0
    }).sum();

    let result = fuel_cost.round();
    println!("result: {}", result);
}