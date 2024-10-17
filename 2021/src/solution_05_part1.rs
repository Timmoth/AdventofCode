use std::fs::read_to_string;

pub fn run() {
    println!("Executing day 5 part 1");

    let file_path = "./inputs/05_actual.txt";
    let input = read_to_string(file_path).expect("Failed to read the file");

    let mut size = 0;

    let coordinates: Vec<((i32, i32), (i32, i32))> = input.lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split(" -> ").collect();
            let from: (i32, i32) = parse_coordinates(parts[0]);
            let to: (i32, i32) = parse_coordinates(parts[1]);

            if from.0 > size {
                size = from.0;
            }
            if from.1 > size {
                size = from.1;
            }

            if(from.0 != to.0 && from.1 != to.1) {
                return None;
            }
            Some((from, to))
        })
        .collect();

    size += 1;

    let mut grid: Vec<Vec<i32>> = vec![vec![0; size as usize]; size as usize];

    for ((x1, y1), (x2, y2)) in coordinates {
        mark_line(&mut grid, x1, y1, x2, y2);
    }

    let mut sum = 0;
    for y in 0..size {
        for x in 0..size{
            let value = grid[y as usize][x as usize];
            if(value >= 2){
                sum += 1;
            }
        }
    }
    
    let result = sum;
    println!("result: {}", result);
}

fn parse_coordinates(coord: &str) -> (i32, i32) {
    let nums: Vec<i32> = coord.split(',')
        .filter_map(|num| num.trim().parse::<i32>().ok())
        .collect();
    (nums[0], nums[1])
}

fn mark_line(grid: &mut Vec<Vec<i32>>, x1: i32, y1: i32, x2: i32, y2: i32) {
    let dx = (x2 - x1).abs();
    let dy = (y2 - y1).abs();
    let sx = if x1 < x2 { 1 } else { -1 }; // Step in x direction
    let sy = if y1 < y2 { 1 } else { -1 }; // Step in y direction

    let mut err = dx - dy;

    let mut x = x1;
    let mut y = y1;

    // Mark the starting point
    if (y >= 0 && y < grid.len() as i32) && (x >= 0 && x < grid[0].len() as i32) {
        grid[y as usize][x as usize] += 1; // Mark the grid with 1
    }

    while (x != x2 || y != y2) {
        let err2 = err * 2;

        // Check horizontal step
        if err2 > -dy {
            err -= dy;
            x += sx;
        }
        
        // Check vertical step
        if err2 < dx {
            err += dx;
            y += sy;
        }

        // Mark the grid after moving to new x, y
        if (y >= 0 && y < grid.len() as i32) && (x >= 0 && x < grid[0].len() as i32) {
            grid[y as usize][x as usize] += 1; // Mark the grid with 1
        }
    }
    

}
