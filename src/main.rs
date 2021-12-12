use std::io::Read;

mod day1;
mod day2;
mod day3;

fn main() {
    let day1_input = load_input("day1.txt");

    println!("Day 1, Part 1: {}", day1::day1_part1(&day1_input));
    println!("Day 1, Part 2: {}", day1::day1_part2(&day1_input));

    let day2_input = load_input("day2.txt");
    let day2_part1_solution = day2::day2_part1(&day2_input);
    let day2_part2_solution = day2::day2_part2(&day2_input);

    println!("Day 2, Part 1: {}", day2_part1_solution);
    println!("Day 2, Part 2: {}", day2_part2_solution);

    let day3_input = load_input("day3.txt");
    let day3_part1_solution = day3::day3_part1(&day3_input);

    println!("Day 2, Part 1: {}", day3_part1_solution);
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
