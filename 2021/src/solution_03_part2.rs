use std::fs::read_to_string;

pub fn run() {
    println!("Executing day 3 part 2");

    let file_path = "./inputs/03_actual.txt";
    let input = read_to_string(file_path).expect("Failed to read the file");

    let numbers: Vec<&str> = input.lines().collect();
    let num_digits = numbers[0].len();

    // Find oxygen generator rating
    let oxy_rating_str = find_rating(&numbers, num_digits, |highs, lows| highs.len() >= lows.len());
    let oxy_rating = u32::from_str_radix(&oxy_rating_str, 2).expect("Failed to parse binary string");

    // Find CO2 scrubber rating
    let co2_rating_str = find_rating(&numbers, num_digits, |highs, lows| highs.len() < lows.len());
    let co2_rating = u32::from_str_radix(&co2_rating_str, 2).expect("Failed to parse binary string");

    // Calculate the result
    let result = oxy_rating * co2_rating;
    println!("result: {}", result);
}

pub fn find_rating<F>(numbers: &[&str], num_digits: usize, cmp_fn: F) -> String
where
    F: Fn(&Vec<&str>, &Vec<&str>) -> bool,
{
    let mut filtered_numbers = numbers.to_vec();
    let mut position = 0;

    // Filter the numbers based on the comparison function
    while position < num_digits && filtered_numbers.len() > 1 {
        let (highs, lows): (Vec<_>, Vec<_>) = filtered_numbers
            .into_iter()
            .partition(|number| number.chars().nth(position) == Some('1'));

        filtered_numbers = if cmp_fn(&highs, &lows) {
            highs
        } else {
            lows
        };

        position += 1;
    }

    // Return the first (and only) remaining number
    filtered_numbers.first().unwrap().to_string()
}
