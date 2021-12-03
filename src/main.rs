use std::io::Read;

fn day1_part1(input: Vec<&str>) -> u16 {
    let input_nums: Vec<i32> = input.iter().map(|l| l.parse::<i32>().unwrap()).collect();
    let mut input_iter = input_nums.iter();

    let mut increases = 0;
    let mut previous = input_iter.next().unwrap();
    for i in &input_nums {
        if i > previous {
            increases += 1;
        }
        previous = i;
    }
    increases
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

    println!("{}", day1_part1(input));
}
