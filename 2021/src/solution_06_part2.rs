use std::fs::read_to_string;

pub fn run() {
    println!("Executing day 6 part 2");

    let file_path = "./inputs/06_actual.txt";
    let input = read_to_string(file_path).expect("Failed to read the file");

    let mut respawn_timers: [i64; 9] = [0; 9]; 

    for respawn_timer in input.split(",").map(|val| val.parse::<i64>().ok()){
        respawn_timers[respawn_timer.unwrap() as usize] += 1;
    }

    for i in 0..256 {

        let first = respawn_timers[0];

        // Shift elements to the left
        for i in 0..8 {
            respawn_timers[i] = respawn_timers[i + 1];
        }

        // Place the first element at the end
        respawn_timers[6] += first;
        respawn_timers[8] = first;
    }

    let total_fish: i64 = respawn_timers.iter().sum();


    let result = total_fish;
    println!("result: {}", result);
}