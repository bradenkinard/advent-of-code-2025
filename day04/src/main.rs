use day04::solve;
use std::fs;

pub fn read_input(path: &str) -> String {
    fs::read_to_string(path).expect("Unable to load inputs file")
}

fn main() {
    let input = read_input("input/day04.txt");
    let (part1, part2) = solve(&input);
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
