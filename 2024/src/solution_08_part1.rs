use std::collections::{HashMap, HashSet};

pub fn run(input: &str) {
    println!("Executing day 8 part 1");

    let mut result: i64 = 0;
    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    let mut y = 0;
    let mut x = 0;
    for line in input.lines() {
        x = 0;
        for char in line.chars() {
            if char != '.' {
                antennas
                    .entry(char)
                    .or_insert_with(Vec::new)
                    .push((x, y));
            }
            x += 1;
        }
        y += 1;
    }

    let mut p: HashSet<(i32, i32)> = HashSet::new();

    for (key, positions) in &antennas {
        for i in 0..positions.len() {
            for j in (i + 1)..positions.len() {
                
                let a_x = positions[i].0 +  positions[i].0 - positions[j].0;
                let a_y = positions[i].1 +  positions[i].1 - positions[j].1;

                let b_x = positions[j].0 +  positions[j].0 - positions[i].0;
                let b_y = positions[j].1 +  positions[j].1 - positions[i].1;

                if a_x >= 0 && a_x < x && a_y >= 0 && a_y < y && p.insert((a_x, a_y)) {
                    result += 1;
                }

                if b_x >= 0 && b_x < x && b_y >= 0 && b_y < y && p.insert((b_x, b_y)) {
                    result += 1;
                }
            }
        }
    }

    println!("result: {}", result);
}
