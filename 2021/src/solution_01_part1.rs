use std::fs::read_to_string;

pub fn run(){
    println!("Executing day 1 part 1");
    let file_path = "./inputs/01_actual.txt";
    let depths: Vec<i32> = read_to_string(file_path).unwrap().lines()
    .filter_map(|line| line.trim().parse::<i32>().ok())
    .collect();
    let mut result = 0;

    if let Some(&first) = depths.first(){
        let mut curr = first;
        for &depth in depths.iter(){
            if depth > curr{
                result+=1;
            }
            curr = depth;
        }
    }

    print!("result: {result}")
}