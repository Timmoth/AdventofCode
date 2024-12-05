use regex::Regex;

pub fn run(input: &str){
    println!("Executing day 3 part 1");

    let mut result: i32 = 0;

    let pattern = r"mul\((\d+),(\d+)\)";
    let re = Regex::new(pattern).unwrap();

    for caps in re.captures_iter(input) {
        let x: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let y: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
        result += x * y;
    }

    println!("result: {result}")
}
