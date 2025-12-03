fn parse_input(input: &str) -> Vec<Vec<u32>> {
    let mut banks: Vec<Vec<u32>> = Vec::new();

    for (_line_num, line) in input.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let bank: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        banks.push(bank);
    }
    banks
}

fn find_max_voltage(bank: &Vec<u32>, number_of_batteries_on: usize) -> u64 {
    let len = bank.len();

    let mut result = Vec::with_capacity(number_of_batteries_on);

    let mut start = 0;

    for remaining in (0..number_of_batteries_on).rev() {
        let end = len - remaining;

        let mut max_digit = bank[start];
        let mut max_rel_idx = 0;

        for (i, &digit) in bank[start..end].iter().enumerate() {
            if digit > max_digit {
                max_digit = digit;
                max_rel_idx = i;
            }
        }

        result.push(max_digit);

        start = start + max_rel_idx + 1;
    }
    result.iter().fold(0, |acc, &d| acc * 10 + d as u64)
}

fn solve_part_1(input: &Vec<Vec<u32>>) -> u64 {
    let mut total_joltage: u64 = 0;
    // For each bank:
    for bank in input {
        total_joltage += find_max_voltage(bank, 2);
    }
    total_joltage
}

fn solve_part_2(input: &Vec<Vec<u32>>) -> u64 {
    let mut total_joltage: u64 = 0;
    for bank in input {
        total_joltage += find_max_voltage(bank, 12);
    }
    total_joltage
}

pub fn solve(input: &str) -> (u64, u64) {
    let input = parse_input(input);
    let part_1_solution = solve_part_1(&input);
    let part_2_solution = solve_part_2(&input);
    (part_1_solution, part_2_solution)
}
