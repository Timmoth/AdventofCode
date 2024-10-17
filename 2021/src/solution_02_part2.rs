use std::fs::read_to_string;

pub fn run() {
    println!("Executing day 2 part 2");

    let file_path = "./inputs/02_actual.txt";
    let input = read_to_string(file_path).expect("Failed to read the file");

    let mut aim = 0;
    // Parse the instructions and calculate in a single pass
    let (horizontal_position, depth) = input
        .lines()
        .filter_map(|line| {
            let mut parts = line.split_whitespace();
            let command = parts.next()?;
            let value: i32 = parts.next()?.parse().ok()?;

            // Map instruction type to tuple of changes to apply
            Some(match command {
                "forward" => {
                    // For "forward," we use the current aim to calculate depth
                    (value, aim * value) // (horizontal, depth)
                }
                "down" => {
                    aim += value; // Increase aim
                    (0, 0) // No horizontal or depth change directly here
                }
                "up" => {
                    aim -= value; // Decrease aim
                    (0, 0) // No horizontal or depth change directly here
                }
                _ => return None, // Ignore unknown commands
            })
        })
        // Sum up horizontal position and depth
        .fold((0, 0), |(h_pos, dep), (h_delta, d_delta)| {
            (h_pos + h_delta, dep + d_delta)
        });

    let result = horizontal_position * depth;
    println!("result: {}", result);
}
