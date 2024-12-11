pub fn run(input: &str) {
    println!("Executing day 9 part 1");

    let mut disk: Vec<Option<usize>> = input
        .chars()
        .filter_map(|c| c.to_digit(10)) 
        .map(|digit| digit as usize) 
        .enumerate()
        .flat_map(|(i, size)| {
            if i % 2 == 0 {
                vec![Some(i / 2); size]
            } else {
                vec![None; size]
            }
        })
        .collect();

    let (mut left, mut right) = (0, disk.len() - 1);
    while left < right {
        while left < right && disk[left].is_some() {
            left += 1;
        }

        while left < right && disk[right].is_none() {
            right -= 1;
        }

        if left < right {
            disk.swap(left, right);
            left += 1;
            right -= 1;
        }
    }

    let result: usize = disk
        .iter()
        .enumerate()
        .filter_map(|(i, &x)| x.map(|value| value * i)) 
        .sum();

    println!("result: {}", result);
}
