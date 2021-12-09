pub fn day2_part1(input: &[String]) -> i32 {
    let mut depth = 0;
    let mut horizontal = 0;

    for line in input.iter() {
        let split: Vec<&str> = line.split(' ').collect();
        if split.len() != 2 {
            eprintln!("Invalid input: {}", line);
            return 0;
        }
        let direction = split[0];
        let amount = split[1]
            .parse::<i32>()
            .expect("Failed to parse direction amount");

        match direction {
            "forward" => horizontal += amount,
            "up" => depth -= amount,
            "down" => depth += amount,
            _ => {
                eprintln!("Unrecognised direction")
            }
        }
    }

    depth * horizontal
}

pub fn day2_part2(input: &[String]) -> i32 {
    let mut aim = 0;
    let mut depth = 0;
    let mut horizontal = 0;

    for line in input.iter() {
        let split: Vec<&str> = line.split(' ').collect();
        if split.len() != 2 {
            eprintln!("Invalid input: {}", line);
            return 0;
        }
        let direction = split[0];
        let amount = split[1]
            .parse::<i32>()
            .expect("Failed to parse direction amount");

        match direction {
            "forward" => {
                horizontal += amount;
                depth += aim * amount;
            }
            "up" => aim -= amount,
            "down" => aim += amount,
            _ => {
                eprintln!("Unrecognised direction")
            }
        }
    }

    depth * horizontal
}
