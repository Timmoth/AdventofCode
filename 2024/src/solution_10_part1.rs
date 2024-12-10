use std::collections::{HashSet, VecDeque};

pub fn run(input: &str) {
    println!("Executing day 10 part 1");

    let mut result: i64 = 0;
    let mut positions: Vec<Vec<i32>> = Vec::new();

    let mut starts: Vec<(i32, i32, i32)> = Vec::new();

    for (row_idx, line) in input.lines().enumerate() {
        let mut row: Vec<i32> = Vec::new();

        for (col_idx, ch) in line.trim().chars().enumerate() {
            if let Some(digit) = ch.to_digit(10) {
                row.push(digit as i32);

                if digit == 0 {
                    starts.push((col_idx as i32, row_idx as i32, digit as i32));
                }
            }
        }

        positions.push(row);
    }

    let width = positions[0].len();
    let height = positions.len();

    let mut queue: VecDeque<(i32, i32, i32)> = VecDeque::new();
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    let directions = [
        (-1, 0),  // Up
        (1, 0),   // Down
        (0, -1),  // Left
        (0, 1),   // Right
    ];

    for (start_x, start_y, start_z) in starts.iter() {
        queue.clear();
        visited.clear();

        queue.push_back((*start_x, *start_y, *start_z));
        visited.insert((*start_x, *start_y));
        let mut trials_fount = 0;
        while !queue.is_empty() {
            let (x, y, z) = queue.pop_front().unwrap();
            visited.insert((x, y));

            if z == 9 {
                trials_fount += 1;
            }

            for (dx, dy) in &directions {
                let new_x = x as isize + dx;
                let new_y = y as isize + dy;

                if new_x >= 0
                    && new_x < width as isize
                    && new_y >= 0
                    && new_y < height as isize
                {
                    let new_x = new_x as i32;
                    let new_y = new_y as i32;

                    if positions[new_y as usize][new_x as usize] == z + 1
                        && !visited.contains(&(new_x, new_y))
                    {
                        queue.push_back((new_x, new_y, z + 1));
                        visited.insert((new_x, new_y));
                    }
                }
            }
        }
        result += trials_fount;
    }

    println!("result: {}", result);
}
