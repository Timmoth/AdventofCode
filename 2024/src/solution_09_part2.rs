#[derive(Debug)]
struct Block {
    is_file: bool,
    data: Vec<usize>,
    free: usize,
}

impl Block {
    fn new(id: usize, size: usize) -> Self {
        if id % 2 == 0 {
            Block {
                is_file: true,
                free: 0,
                data: vec![id / 2; size],
            }
        } else {
            Block {
                is_file: false,
                free: size,
                data: Vec::new(),
            }
        }
    }

    fn fill(&mut self, data: &Vec<usize>) {
        self.data.extend(data);
        self.free -= data.len();
        if self.free == 0 {
            self.is_file = true;
        }
    }

    fn clear(&mut self) {
        self.free = self.data.len();
        self.data.clear();
        self.is_file = false;
    }
}

pub fn run(input: &str) {
    println!("Executing day 9 part 2");

    let mut disk: Vec<Block> = input
        .chars()
        .filter(|&c| c.is_digit(10))
        .filter_map(|c| c.to_digit(10))
        .map(|c| c as usize)
        .enumerate()
        .map(|(i, size)| Block::new(i, size))
        .collect();

    let mut right = disk.len() - 1;
    while right > 0 {
        if !disk[right].is_file {
            right -= 1;
            continue;
        }

        let mut left = 0;
        while left < right {
            if disk[left].is_file
                || (!disk[left].is_file && disk[left].free < disk[right].data.len())
            {
                left += 1;
                continue;
            }

            let r = disk[right].data.clone();
            disk[left].fill(&r);
            disk[right].clear();
            break;
        }
        right -= 1;
    }

    let result = disk
        .iter()
        .flat_map(|d| {
            d.data
                .iter()
                .cloned()
                .chain(std::iter::repeat(0).take(d.free))
        })
        .enumerate()
        .map(|(i, x)| x * i)
        .sum::<usize>();

    println!("result: {}", result);
}