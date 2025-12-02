use anyhow::Result;
use std::fs;
use day02::solve;

pub fn read_input(path: &str) -> Result<String> {
    Ok(fs::read_to_string(path)?)
}

fn main() -> Result<()> {
    let input = read_input("input/day02.txt")?;
    let (part1, part2) = solve(&input)?;
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
    Ok(())
}