fn count_increases(nums: &[i32]) -> u16 {
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

pub fn day1_part1(input: &[String]) -> u16 {
    let input_nums: Vec<i32> = input.iter().map(|l| l.parse::<i32>().unwrap()).collect();
    count_increases(&input_nums)
}

pub fn day1_part2(input: &[String]) -> u16 {
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
