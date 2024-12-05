pub fn run() {
    println!("Executing day 4 part 1");

    let mut result: i32 = 0;

    let input: Vec<Vec<char>> = include_str!("../inputs/04_actual.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    for i in 0..input.len() {
        let line = &input[i];
        for j in 0..line.len() {
            let char_at_pos = line[j];
            if char_at_pos != 'X' {
                continue;
            }

            result += count_matches(&input, j, i);
        }
    }

    println!("result: {result}")
}

pub fn count_matches(input: &Vec<Vec<char>>, x: usize, y: usize) -> i32 {
    let mut count = 0;

    let rows = input.len();
    let cols = input[0].len();

    let within_bounds =
        |x: isize, y: isize| -> bool { x >= 0 && x < cols as isize && y >= 0 && y < rows as isize };

    // Right
    if x + 3 < cols && input[y][x + 1] == 'M' && input[y][x + 2] == 'A' && input[y][x + 3] == 'S' {
        count += 1;
    }

    // Left
    if x >= 3 && input[y][x - 1] == 'M' && input[y][x - 2] == 'A' && input[y][x - 3] == 'S' {
        count += 1;
    }
    // Down
    if y + 3 < rows && input[y + 1][x] == 'M' && input[y + 2][x] == 'A' && input[y + 3][x] == 'S' {
        count += 1;
    }
    // Up
    if y >= 3 && input[y - 1][x] == 'M' && input[y - 2][x] == 'A' && input[y - 3][x] == 'S' {
        count += 1;
    }
    // Down-right (Diagonal)
    if within_bounds((x as isize) + 3, (y as isize) + 3)
        && input[y + 1][x + 1] == 'M'
        && input[y + 2][x + 2] == 'A'
        && input[y + 3][x + 3] == 'S'
    {
        count += 1;
    }
    // Down-left (Diagonal)
    if within_bounds((x as isize) - 3, (y as isize) + 3)
        && input[y + 1][x - 1] == 'M'
        && input[y + 2][x - 2] == 'A'
        && input[y + 3][x - 3] == 'S'
    {
        count += 1;
    }
    // Up-right (Diagonal)
    if within_bounds((x as isize) + 3, (y as isize) - 3)
        && input[y - 1][x + 1] == 'M'
        && input[y - 2][x + 2] == 'A'
        && input[y - 3][x + 3] == 'S'
    {
        count += 1;
    }
    // Up-left (Diagonal)
    if within_bounds((x as isize) - 3, (y as isize) - 3)
        && input[y - 1][x - 1] == 'M'
        && input[y - 2][x - 2] == 'A'
        && input[y - 3][x - 3] == 'S'
    {
        count += 1;
    }

    return count;
}
