use std::fs::read_to_string;

pub fn run(){
    println!("Executing day 1 part 2");
    let file_path = "./inputs/01_actual.txt";

    let mut list_a: Vec<i32> = Vec::new();
    let mut counts: [i32; 100000] = [0; 100_000];

    let mut line_count = 0;
    for line in read_to_string(file_path).unwrap().lines(){
        let mut parts = line.split_whitespace();
        let a: i32 = parts.next().unwrap().parse().unwrap();
        let b: i32 = parts.next().unwrap().parse().unwrap();
        list_a.push(a);
        line_count += 1;

        counts[b as usize] += 1;
    }
    list_a.sort_unstable();

    let mut result = 0;

    for i in 0..line_count{
        let a = list_a[i];
        result += a * counts[a as usize];
    }
    

    println!("result: {result}")
}