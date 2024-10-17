use std::fs::read_to_string;

pub fn run() {
    println!("Executing day 3 part 1");

    let file_path = "./inputs/03_actual.txt";
    let input = read_to_string(file_path).expect("Failed to read the file");

    // Split lines and collect them into a Vec<&str>
    let lines: Vec<&str> = input.lines().collect();
    let num_lines = lines.len() as i32;
    let num_digits = lines[0].len();

    // Create a counts vector initialized to zeros
    let mut counts = vec![0; num_digits];

    // Count the occurrences of '1's for each digit position
    for line in &lines {
        for (i, c) in line.chars().enumerate() {
            if c == '1' {
                counts[i] += 1;
            }
        }
    }

    let mid = num_lines / 2;
    let (mut gamma, mut epsilon) = (0, 0);

    // Calculate gamma and epsilon directly without reversing
    for (i, &count) in counts.iter().enumerate() {
        if count > mid {
            gamma |= 1 << (num_digits - 1 - i);
        } else {
            epsilon |= 1 << (num_digits - 1 - i);
        }
    }

    let result = gamma * epsilon;
    println!("result: {}", result);
}
