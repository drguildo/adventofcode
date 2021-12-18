pub fn day4_part1(input: &[String]) -> u32 {
    let draw: Vec<u8> = input
        .first()
        .expect("Failed to read number draw from input")
        .split(",")
        .map(|s| s.parse::<u8>().expect("Failed to parse draw number"))
        .collect();

    println!("{:?}", draw);

    0
}
