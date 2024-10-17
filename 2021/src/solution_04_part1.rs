use std::fs::read_to_string;

pub fn run() {
    println!("Executing day 4 part 1");

    let file_path = "./inputs/04_actual.txt";
    let input = read_to_string(file_path).expect("Failed to read the file");

    let mut lines = input.lines();

    let bingo_numbers: Vec<i32> = lines
    .next()
    .unwrap()
    .split(',')
    .filter_map(|num| num.trim().parse::<i32>().ok())
    .collect();
    let max_bingo_number = *bingo_numbers.iter().max().unwrap();

    let board_lines: Vec<&str> = lines.collect();

    let boards: Vec<&[&str]> = board_lines
    .split(|&line| line.trim().is_empty())
    .filter(|group| !group.is_empty())
    .collect();

    let board_count = boards.len();
    let mut bingo_number_boards: Vec<Vec<usize>> = vec![Vec::new(); (max_bingo_number + 1) as usize];
    for &bingo_number in &bingo_numbers {
        bingo_number_boards[bingo_number as usize] = Vec::new();
    }

    let mut winning_lines: Vec<i32> = vec![5; (board_count * 10) as usize];
    for (board_index, grid) in boards.iter().enumerate() {
        let board_offset = board_index * 10;
        let mut row_index= 0;

        for &line in *grid {
            let mut col_index = 0;
            for bingo_number in line.split(" ").filter_map(|num| num.trim().parse::<i32>().ok()){
                bingo_number_boards[bingo_number as usize].push(board_offset + row_index);
                bingo_number_boards[bingo_number as usize].push(board_offset + 5 + col_index);

                col_index += 1;
            }
            row_index += 1;
        }
    }

    let mut last_called_bingo_number_index = 0;
    let mut winning_line_index: i32 = -1;
    for bingo_number in &bingo_numbers{
        last_called_bingo_number_index += 1;

        if let Some(line_indices) = bingo_number_boards.get(*bingo_number as usize) {
            for &line_index in line_indices {
                winning_lines[line_index] -= 1;
                if  winning_lines[line_index] <= 0 {
                    winning_line_index = line_index as i32;
                    break;
                }
            }
        }
        
        if winning_line_index >= 0 {
            break;
        }
    }

    let board_index = winning_line_index / 10;
    let winning_board = boards.get(board_index as usize).unwrap();

    let winning_bingo_number = bingo_numbers.get(last_called_bingo_number_index - 1).unwrap();
    let called_bingo_numbers = &bingo_numbers[..last_called_bingo_number_index];

    let mut unmarked_numbers_sum = 0;

    for line in winning_board.iter() {
        for bingo_number in line.split_whitespace().filter_map(|num| num.trim().parse::<i32>().ok()) {
            if !called_bingo_numbers.contains(&bingo_number) {
                unmarked_numbers_sum += bingo_number;
            }
        }
    }

    let result = unmarked_numbers_sum * winning_bingo_number;
    println!("result: {}", result);
}