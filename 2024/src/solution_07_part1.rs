pub fn run(input: &str) {
    println!("Executing day 7 part 1");

    let mut result: i64 = 0;
    let mut lines: Vec<(i64, Vec<i64>)> = Vec::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split(':').map(|part| part.trim()).collect();
        if parts.len() == 2 {
            let key = parts[0].parse::<i64>().expect("Invalid key");
            let values = parts[1]
                .split_whitespace()
                .map(|v| v.parse::<i64>().expect("Invalid number"))
                .collect();
            lines.push((key, values));
        }
    }

    for line in lines {
        let (key, values) = line;
        if is_possible(&values, key) {
            result += key;
        }
    }

    println!("result: {}", result);
}

pub fn is_possible(terms: &[i64], target: i64) -> bool {
    if terms.len() == 1 {
        return terms[0] == target;
    }

    if terms.len() >= 2 {
        let last = terms[terms.len() - 1];
        let rest = &terms[..terms.len() - 1];

        // Case 1: Division
        if target % last == 0 && is_possible(rest, target / last) {
            return true;
        }

        // Case 2: Subtraction
        if target > last && is_possible(rest, target - last) {
            return true;
        }
    }

    false
}