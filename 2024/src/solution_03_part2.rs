use regex::Regex;

pub fn run() {
    println!("Executing day 3 part 2");

    let mut result: i32 = 0;
    let input = include_str!("../inputs/03_actual.txt");

    let pattern = r"(?:do(?:n't)?\(\))|mul\((\d+),(\d+)\)";
    let re = Regex::new(pattern).unwrap();

    let mut include: bool = true;
    for caps in re.captures_iter(input) {
        if let Some(m) = caps.get(0) {
            let match_text = m.as_str();
            if let (Some(x), Some(y)) = (caps.get(1), caps.get(2)) {
                if include {
                    let x: i32 = x.as_str().parse().unwrap();
                    let y: i32 = y.as_str().parse().unwrap();
                    result += x * y;
                }
            } else {
                include = !match_text.starts_with("don't");
            }
        }
    }

    println!("result: {result}")
}
