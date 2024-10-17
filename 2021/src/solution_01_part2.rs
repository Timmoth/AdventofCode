use std::fs::read_to_string;

pub fn run() {
    println!("Executing day 1 part 2");
    let file_path = "./inputs/01_actual.txt";
    let depths: Vec<i32> = read_to_string(file_path)
        .unwrap()
        .lines()
        .filter_map(|line| line.parse::<i32>().ok())
        .collect();

    let mut result = 0;
    let mut prev_sum = None;

    for window in depths.windows(3) {
        let window_sum: i32 = window.iter().sum();

        if let Some(prev) = prev_sum {
            if window_sum > prev {
                result += 1;
            }
        }

        prev_sum = Some(window_sum);
    }

    println!("result: {result}");
}
