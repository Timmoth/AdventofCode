use std::fs::read_to_string;

pub fn run(){
    println!("Executing day 1 part 1");
    let file_path = "./inputs/01_actual.txt";

    let mut list_a: Vec<i32> = Vec::new();
    let mut list_b: Vec<i32> = Vec::new();

    let mut line_count = 0;
    for line in read_to_string(file_path).unwrap().lines(){
        let mut parts = line.split_whitespace();
        let a: i32 = parts.next().unwrap().parse().unwrap();
        let b: i32 = parts.next().unwrap().parse().unwrap();
        list_a.push(a);
        list_b.push(b);
        line_count += 1;
    }
    
    list_a.sort_unstable();
    list_b.sort_unstable();

    let mut result = 0;

    for i in 0..line_count{
        result += (list_a[i] - list_b[i]).abs();
    }
    

    println!("result: {result}")
}