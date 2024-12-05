pub fn run() {
    println!("Executing day 4 part 2");

    let mut result: i32 = 0;

    let input: Vec<Vec<char>> = include_str!("../inputs/04_actual.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    for i in 1..input.len() - 1 {
        let line = &input[i];
        for j in 1..line.len() - 1 {
            let char_at_pos = line[j];
            if char_at_pos != 'A' {
                continue;
            }

            if is_match(&input, j, i) {
                result += 1;
            }
        }
    }

    println!("result: {result}")
}

pub fn is_match(input: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    return (input[y - 1][x - 1] == 'M' && input[y + 1][x + 1] == 'S' || input[y - 1][x - 1] == 'S' && input[y + 1][x + 1] == 'M') && (input[y + 1][x - 1] == 'M' && input[y - 1][x + 1] == 'S' || input[y + 1][x - 1] == 'S' && input[y - 1][x + 1] == 'M');
}
