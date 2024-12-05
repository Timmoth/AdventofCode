pub fn run(input: &str){
    println!("Executing day 2 part 2");

    let mut result: i32 = 0;

    for line in input.lines() {
        let levels: Vec<i32> = line
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();

        if is_safe_report(&levels) {
            result += 1;
            continue;
        }

        for skip_index in 0..levels.len() {
            if is_safe_with_skip(&levels, skip_index) {
                result += 1;
                break;
            }
        }
    }

    println!("result: {}", result);
}

fn is_safe_report(levels: &[i32]) -> bool {
    let increasing = levels[1] > levels[0];

    for window in levels.windows(2) {
        let delta = window[1] - window[0];
        if is_invalid_delta(increasing, delta) {
            return false;
        }
    }

    true
}

fn is_safe_with_skip(levels: &[i32], skip_index: usize) -> bool {
    let mut prev = None;
    let increasing = if skip_index == 0 {
        levels[2] > levels[1]
    } else if skip_index == 1 {
        levels[2] > levels[0]
    } else {
        levels[1] > levels[0]
    };

    for (i, &current) in levels.iter().enumerate() {
        if i == skip_index {
            continue;
        }

        if let Some(previous) = prev {
            let delta = current - previous;
            if is_invalid_delta(increasing, delta) {
                return false;
            }
        }

        prev = Some(current);
    }

    true
}

#[inline(always)]
fn is_invalid_delta(increasing: bool, delta: i32) -> bool {
    if increasing {
        delta < 1 || delta > 3
    } else {
        delta > -1 || delta < -3
    }
}
