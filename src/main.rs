use std::io::Read;

mod day1;

fn main() {
    let day1_input = load_input("day1_part1.txt");

    println!("Day 1, Part 1: {}", day1::day1_part1(&day1_input));
    println!("Day 1, Part 2: {}", day1::day1_part2(&day1_input));
}

fn load_input(input_path: &str) -> Vec<String> {
    let mut input_string = String::new();
    std::fs::File::open(input_path)
        .expect("Failed to open input file")
        .read_to_string(&mut input_string)
        .expect("Failed to convert input file to string");
    let input: Vec<String> = input_string.lines().map(|s| s.to_owned()).collect();
    input
}
