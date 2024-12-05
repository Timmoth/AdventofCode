pub fn run(input: &str){
    println!("Executing day 5 part 1");

    let mut result: i32 = 0;

    let input = input.replace("\r\n", "\n");
    let parts: Vec<&str> = input.split("\n\n").collect();

    let mut rules: Vec<Vec<bool>> = vec![vec![false; 100]; 100];
    for line in parts[0].lines(){
        let rule_parts: Vec<usize> = line
            .split('|') 
            .map(|part| part.trim().parse::<usize>().expect("Invalid number")) // Parse as usize
            .collect();
        
        rules[rule_parts[0]][rule_parts[1]] = true;
    }

    for line in parts[1].lines(){
        let values: Vec<i32> = line.split(',').map(|val| val.trim().parse().unwrap()).collect();
        if is_in_order(&values, &rules) {
            result += values.get(values.len() / 2).unwrap();
        }
    }

    println!("result: {result}")
}

fn is_in_order(values: &Vec<i32>, rules: &Vec<Vec<bool>>) -> bool {
    for pair in values.windows(2) {
        if !rules[pair[0] as usize][pair[1] as usize] {
            return false;
        }
    }
    true
}