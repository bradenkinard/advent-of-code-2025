fn parse_ranges(input: &str) -> Vec<(i64, i64)> {
    input
        .split(',')
        .map(|range| {
            let (start_str, end_str) = range
                .split_once('-')
                .expect("Invalid rgne format");

            let start: i64 = start_str.trim().parse().unwrap();
            let end: i64 = end_str.trim().parse().unwrap();

            (start, end)
        })
        .collect::<Vec<_>>()
}

fn check_invalid_part_1(i: i64) -> bool {
    let i_string = i.to_string();
    if i_string.len() % 2 == 1 {
        return false; // Skip all values with odd number of digits
    }
    let (left, right) = i_string.split_at(i_string.len() / 2);
    left == right
}

fn check_if_chunks_equal(s: &String, n: usize) -> bool {
    if s.len() % n != 0 {
        return false;
    }

    let mut iter = s.as_bytes().chunks_exact(n);

    let first = match iter.next() {
        Some(c) => c,
        None => return true,
    };

    iter.all(|c| c == first)
}

fn check_invalid_part_2(i: i64) -> bool {
    let i_string = i.to_string();
    for i in 1..=i_string.len() / 2 {
        if i_string.len() % i != 0 {
            continue; // Skip substring lengths that can't divide equally
        }
        if check_if_chunks_equal(&i_string, i) {
            return true;
        }
    }

    false
}

fn solve_part_1(ranges: &Vec<(i64, i64)>) -> i64 {
    let mut total: i64 = 0;
    for r in ranges {
        for i in r.0..=r.1 {
            if check_invalid_part_1(i) {
                total += i;
            }
        }
    }
    total
}

fn solve_part_2(ranges: &Vec<(i64, i64)>) -> i64 {
    let mut total: i64 = 0;
    for r in ranges {
        for i in r.0..=r.1 {
            if check_invalid_part_2(i) {
                total += i;
            }
        }
    }
    total
}

pub fn solve(input: &str) -> (i64, i64) {
    let ranges = parse_ranges(&input);
    let part_1_solution = solve_part_1(&ranges);
    let part_2_solution = solve_part_2(&ranges);
    (part_1_solution, part_2_solution)
}
