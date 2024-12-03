pub fn run() {
    println!("Executing day 2 part 1");

    let mut result: i32 = 0;
    for line in include_str!("../inputs/02_actual.txt").lines() {
        let mut parts = line.split_whitespace();
        let prev: i32 = parts.next().unwrap().parse().unwrap();
        let curr: i32 = parts.next().unwrap().parse().unwrap();
        let delta = prev - curr;
        let increasing = delta > 0;
        if is_invalid_delta(increasing, delta) {
            continue;
        }

        let mut prev = curr;
        result += 1;
        for level in parts {
            let curr: i32 = level.parse().unwrap();
            if is_invalid_delta(increasing, prev - curr) {
                result -= 1;
                break;
            }
            prev = curr;
        }
    }

    println!("result: {result}")
}

fn is_invalid_delta(increasing: bool, delta: i32) -> bool {
    if increasing {
        return delta < 1 || delta > 3;
    } else {
        return delta > -1 || delta < -3;
    }
}
