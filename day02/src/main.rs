use day02::solve;
use std::fs;

pub fn read_input(path: &str) -> String {
    fs::read_to_string(path).expect("Failed to read input")
}

fn main() {
    let input = read_input("input/day02.txt");
    let (part1, part2) = solve(&input);
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
