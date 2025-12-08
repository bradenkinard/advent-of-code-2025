use day06::solve;
use std::fs;

fn read_input(path: &str) -> String {
    fs::read_to_string(path).expect("Unable to load inputs")
}

fn main() {
    let input = read_input("input/day06.txt");
    let (s1, s2) = solve(&input);
    println!("Part 1: {}", s1);
    println!("Part 2: {}", s2);
}
