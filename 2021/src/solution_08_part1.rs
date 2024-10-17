use std::fs::read_to_string;

pub fn run() {
    println!("Executing day 8 part 1");

    let file_path = "./inputs/08_actual.txt";
    let input = read_to_string(file_path).expect("Failed to read the file");
    let lines: Vec<&str> = input.lines().collect();
    let entries: Vec<(Vec<Vec<char>>, Vec<Vec<char>>)> = lines
        .iter()
        .map(|line| {
            // Split each line by " | "
            let parts: Vec<&str> = line.split(" | ").collect();
            
            // Split each part by spaces and convert to Vec<char>
            let a: Vec<Vec<char>> = parts[0]
                .split_whitespace() // Split by spaces
                .map(|s| s.chars().collect()) // Convert each word to Vec<char>
                .collect(); // Collect into Vec<Vec<char>>

            let b: Vec<Vec<char>> = parts[1]
                .split_whitespace() // Split by spaces
                .map(|s| s.chars().collect()) // Convert each word to Vec<char>
                .collect(); // Collect into Vec<Vec<char>>

            (a, b) // Return a tuple of Vec<Vec<char>> for a and b
        })
        .collect(); // Collect all results into a Vec

    let mut unique_digit_count = 0;
    for (signal, output) in entries {
        unique_digit_count += output.iter().filter(|o| {
            let segment_count = o.len();

            match segment_count {
                2 => true,
                4 => true,
                3 => true,
                7 => true,
                _ => false,
            }
        }).count()
    }

    let result = unique_digit_count;
    println!("result: {}", result);
}