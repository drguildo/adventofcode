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
