use anyhow::{anyhow,Result};

fn parse_ranges(input: &str) -> Result<Vec<(i64, i64)>> {
    input
        .split(',')
        .map(|range| {
            let (start_str, end_str) =
                range.split_once('-').ok_or_else(|| anyhow!("Invalid range `{range}`"))?;

            let start: i64 = start_str.trim().parse()?;
            let end: i64 = end_str.trim().parse()?;

            Ok((start, end))
        })
        .collect::<Result<Vec<_>>>()
}

fn check_invalid_part_1(i: i64) -> Result<bool> {
    let i_string = i.to_string();
    if i_string.len() % 2 == 1 {
        return Ok(false) // Skip all values with odd number of digits
    }
    let (left, right) = i_string.split_at(i_string.len() / 2);
    Ok(left == right)
}

fn check_if_chunks_equal(s: &String, n: usize) -> Result<bool> {
    if s.len() % n != 0 {
        return Ok(false);
    }

    let mut iter = s.as_bytes().chunks_exact(n);

    let first = match iter.next() {
        Some(c) => c,
        None => return Ok(true),
    };

    Ok(iter.all(|c| c == first))
}

fn check_invalid_part_2(i: i64) -> Result<bool> {
    let i_string = i.to_string();
    for i in 1..=i_string.len() / 2 {
        if i_string.len() % i != 0 {
            continue  // Skip substring lengths that can't divide equally
        }
        if check_if_chunks_equal(&i_string, i)? {
            return Ok(true);
        }
    }

    Ok(false)
}

fn solve_part_1(ranges: &Vec<(i64, i64)>) -> Result<i64> {
    let mut total: i64 = 0;
    for r in ranges {
        for i in r.0..=r.1 {
            if check_invalid_part_1(i)? {
                total += i;
            }
        }
    }
    Ok(total)
}


fn solve_part_2(ranges: &Vec<(i64, i64)>) -> Result<i64> {
    let mut total: i64 = 0;
    for r in ranges {
        for i in r.0..=r.1 {
            if check_invalid_part_2(i)? {
                total += i;
            }
        }
    }
    Ok(total)
}

pub fn solve(input: &str) -> Result<(i64, i64)> {
    let ranges = parse_ranges(&input)?;
    let part_1_solution = solve_part_1(&ranges)?;
    let part_2_solution = solve_part_2(&ranges)?;
    Ok((part_1_solution, part_2_solution))
}