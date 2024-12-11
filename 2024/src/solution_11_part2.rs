use std::collections::HashMap;

pub fn run(input: &str) {
    println!("Executing day 11 part 2");

    let mut result: i64 = 0;

    let mut cache: HashMap<(i64, u8), i64> = HashMap::new();

    for num in input.split(' ').map(|n| n.parse::<i64>().unwrap()) {
        result += count(num, 75,  &mut cache);
    }

    println!("result: {}", result);
}

pub fn count(num: i64, depth: u8, cache: &mut HashMap<(i64, u8), i64>) -> i64 {
    if depth == 0 {
        return 1;
    }

    if let Some(&cached_result) = cache.get(&(num, depth)) {
        return cached_result;
    }

    let result = if num == 0 {
        count(1, depth - 1, cache)
    } else {
        let digits = (num as f64).log10().floor() as u32 + 1;

        if digits % 2 != 0 {
            return count(num * 2024, depth - 1, cache)
        }
        
        let divisor = 10i64.pow(digits / 2);
    
        let left = num / divisor;
        let right = num % divisor;
        count(left, depth - 1, cache) + count(right, depth - 1, cache)
    };

    cache.insert((num, depth), result);
    result
}