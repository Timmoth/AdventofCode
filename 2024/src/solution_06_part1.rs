use std::collections::HashSet;

pub fn run(input: &str){
    println!("Executing day 6 part 1");

    let mut result: i32 = 0;
    let mut curr_x = 0;
    let mut curr_y = 0;
    let mut curr_dir = 0; // [0,1,2,3] = [up,right,down,left]
    let mut map: Vec<Vec<char>> = Vec::new();
    for (line_index, line) in input.lines().enumerate() {
        let chars: Vec<char> = line.chars().collect();
        if let Some(index) = chars.iter().position(|&ch| ch == '^') {
            curr_x = index;
            curr_y = line_index;
        }
        map.push(chars);
    }

    let height = map.len();
    let width = if height > 0 { map[0].len() } else { 0 };

    let mut history: HashSet<(usize, usize)> = HashSet::new();
    loop {
        if curr_dir == 0 {
            for y in (0..=curr_y).rev() {
                if map[y][curr_x] == '#' {
                    break;
                }
                curr_y = y;
            
                if history.insert((curr_x, curr_y)) {
                    result += 1;
                }
            }

            if curr_y == 0 {
                break;
            }
        }else if curr_dir == 1{
            for x in curr_x..width {
                if map[curr_y][x] == '#' {
                    break;
                }
                curr_x = x;
            
                if history.insert((curr_x, curr_y)) {
                    result += 1;
                }
            }

            if curr_x == width - 1 {
                break;
            }
        }else if curr_dir == 2{
            for y in curr_y..height {
                if map[y][curr_x] == '#' {
                    break;
                }
                curr_y = y;
            
                if history.insert((curr_x, curr_y)) {
                    result += 1;
                }
            }

            if curr_y == height - 1 {
                break;
            }
        }else if curr_dir == 3{
            for x in (0..=curr_x).rev() {
                if map[curr_y][x] == '#' {
                    break;
                }
                curr_x = x;
            
                if history.insert((curr_x, curr_y)) {
                    result += 1;
                }
            }

            if curr_x == 0 {
                break;
            }
        }

        curr_dir = (curr_dir + 1) % 4;
    }

    println!("result: {result}")
}
