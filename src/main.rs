use std::io::Read;

fn count_increases(nums: &Vec<i32>) -> u16 {
    let mut input_iter = nums.iter();

    let mut increases = 0;
    let mut previous = input_iter.next().unwrap();
    for i in nums {
        if i > previous {
            increases += 1;
        }
        previous = i;
    }
    increases
}

fn day1_part1(input: &Vec<&str>) -> u16 {
    let input_nums: Vec<i32> = input.iter().map(|l| l.parse::<i32>().unwrap()).collect();
    count_increases(&input_nums)
}

fn day1_part2(input: &Vec<&str>) -> u16 {
    let input_nums: Vec<i32> = input.iter().map(|l| l.parse::<i32>().unwrap()).collect();

    let mut sliding_windows: Vec<Vec<i32>> = Vec::new();
    let mut i: usize = 0;
    while i < input_nums.len() - 2 {
        let mut sliding_window: Vec<i32> = Vec::new();
        let mut num_taken = 0;
        while num_taken < 3 {
            sliding_window.push(input_nums[i + num_taken]);
            num_taken += 1;
        }
        sliding_windows.push(sliding_window);
        i += 1;
    }

    let sum_of_sliding_windows: Vec<i32> = sliding_windows.iter().map(|w| w.iter().sum()).collect();
    count_increases(&sum_of_sliding_windows)
}

fn main() {
    let input_path: String = std::env::args().skip(1).take(1).collect();
    if input_path.len() == 0 {
        eprintln!("Please specify an input file.");
        std::process::exit(1);
    }

    let mut input_string = String::new();
    std::fs::File::open(input_path)
        .expect("Failed to open input file")
        .read_to_string(&mut input_string)
        .expect("Failed to convert input file to string");
    let input: Vec<&str> = input_string.lines().collect();

    println!("Day 1, Part 1: {}", day1_part1(&input));
    println!("Day 1, Part 2: {}", day1_part2(&input));
}
