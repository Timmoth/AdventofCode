use std::collections::{HashMap, HashSet};

pub fn run(input: &str) {
    println!("Executing day 10 part 2");

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

    let mut memo: HashMap<(i32, i32, i32), i32> = HashMap::new();

    let mut total_paths = 0;
    for (start_x, start_y, start_z) in starts.iter() {
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        total_paths += dfs_with_memo(
            *start_x,
            *start_y,
            *start_z,
            &positions,
            &mut visited,
            &mut memo,
            width,
            height,
        );
    }

    println!("result: {}", total_paths);
}

pub fn dfs_with_memo(
    x: i32,
    y: i32,
    z: i32,
    positions: &Vec<Vec<i32>>,
    visited: &mut HashSet<(i32, i32)>,
    memo: &mut HashMap<(i32, i32, i32), i32>,
    width: usize,
    height: usize,
) -> i32 {
    if let Some(&cached_result) = memo.get(&(x, y, z)) {
        return cached_result;
    }

    visited.insert((x, y));

    let mut paths_found = 0;
    if z == 9 {
        paths_found = 1;
    } else {
        for (dx, dy) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let new_x = x + dx;
            let new_y = y + dy;

            if new_x >= 0
                && new_x < width as i32
                && new_y >= 0
                && new_y < height as i32
                && positions[new_y as usize][new_x as usize] == z + 1
                && !visited.contains(&(new_x, new_y))
            {
                paths_found += dfs_with_memo(
                    new_x,
                    new_y,
                    z + 1,
                    positions,
                    visited,
                    memo,
                    width,
                    height,
                );
            }
        }
    }

    memo.insert((x, y, z), paths_found);

    visited.remove(&(x, y));
    paths_found
}
