pub fn day3_part1(input: &[String]) -> u32 {
    let mut counter: [u16; 12] = [0; 12];

    for binary in input {
        for (i, c) in binary.chars().enumerate() {
            if c == '1' {
                counter[i] += 1;
            }
        }
    }

    let half_len = (input.len() / 2) as u16;

    let mut gamma: u32 = 0;
    for count in counter {
        gamma = gamma << 1;
        if count > half_len {
            gamma += 1;
        }
    }
    let epsilon = !gamma & 0xFFF;

    gamma * epsilon
}

fn most_common(input: &[String]) -> [char; 12] {
    let mut counter_ones: [u16; 12] = [0; 12];
    let mut counter_zeroes: [u16; 12] = [0; 12];

    for binary in input {
        for (i, c) in binary.chars().enumerate() {
            if c == '1' {
                counter_ones[i] += 1;
            } else {
                counter_zeroes[i] += 1;
            }
        }
    }

    let mut most_common: [char; 12] = ['0'; 12];
    for (i, (ones, zeros)) in counter_ones.iter().zip(counter_zeroes.iter()).enumerate() {
        if *ones >= *zeros {
            most_common[i] = '1';
        }
    }

    most_common
}

fn least_common(input: &[String]) -> [char; 12] {
    let mut counter_ones: [u16; 12] = [0; 12];
    let mut counter_zeroes: [u16; 12] = [0; 12];

    for binary in input {
        for (i, c) in binary.chars().enumerate() {
            if c == '1' {
                counter_ones[i] += 1;
            } else {
                counter_zeroes[i] += 1;
            }
        }
    }

    let mut least_common: [char; 12] = ['0'; 12];
    for (i, (ones, zeros)) in counter_ones.iter().zip(counter_zeroes.iter()).enumerate() {
        if *ones < *zeros {
            least_common[i] = '1';
        }
    }

    least_common
}

fn binary_string_to_u16(s: &String) -> u16 {
    let mut num: u16 = 0;
    for c in s.chars() {
        num = num << 1;
        if c == '1' {
            num += 1;
        }
    }
    num
}

pub fn day3_part2(input: &[String]) -> u32 {
    let mut i = 0;

    let mut potential_oxygen = input.clone().to_vec();
    loop {
        let most_common = most_common(&potential_oxygen);
        potential_oxygen = potential_oxygen
            .iter()
            .filter(|s| s.chars().nth(i).expect("Failed to get char at index") == most_common[i])
            .map(|s| s.to_owned())
            .collect();
        if potential_oxygen.len() <= 1 {
            break;
        }
        i += 1;
    }

    let oxygen = binary_string_to_u16(potential_oxygen.first().unwrap());

    i = 0;
    let mut potential_co2 = input.clone().to_vec();
    loop {
        let least_common = least_common(&potential_co2);
        potential_co2 = potential_co2
            .iter()
            .filter(|s| s.chars().nth(i).expect("Failed to get char at index") == least_common[i])
            .map(|s| s.to_owned())
            .collect();
        if potential_co2.len() <= 1 {
            break;
        }
        i += 1;
    }

    let co2 = binary_string_to_u16(potential_co2.first().unwrap());

    oxygen as u32 * co2 as u32
}
