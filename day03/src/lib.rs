use anyhow::Result;


fn parse_input(input: &str) -> Result<&str> {
    Ok("Input")
}

fn solve_part_1(input: &str) -> Result<i64> {
    Ok(0)
}

fn solve_part_2(input: &str) -> Result<i64> {
    Ok(0)
}


pub fn solve(input: &str) -> Result<(i64, i64)> {
    let input = parse_input(input)?;
    let part_1_solution = solve_part_1(&input)?;
    let part_2_solution = solve_part_2(&input)?;
    Ok((part_1_solution, part_2_solution))
}