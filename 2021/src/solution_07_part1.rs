use std::fs::read_to_string;

pub fn run() {
    println!("Executing day 7 part 1");

    let file_path = "./inputs/07_actual.txt";
    let input = read_to_string(file_path).expect("Failed to read the file");

    let mut numbers: Vec<i64> = input
        .split(',')
        .filter_map(|val| val.trim().parse::<i64>().ok()) // Parse and filter out invalid values
        .collect();
    
    numbers.sort_unstable();

    let len = numbers.len();
    let median = if len % 2 == 1 {
        // Odd length: the middle element is the median
        numbers[len / 2] as f64
    } else {
        // Even length: average of the two middle elements
        let mid1 = numbers[len / 2 - 1];
        let mid2 = numbers[len / 2];
        (mid1 + mid2) as f64 / 2.0
    };

    let fuel_cost: i64 = numbers.iter().map(|d| (d - median as i64).abs()).sum();

    let result = fuel_cost;
    println!("result: {}", result);
}