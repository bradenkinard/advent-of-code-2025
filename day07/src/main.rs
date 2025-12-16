use std::fs;
use day07::solve;

fn read_input(path: &str) -> String {
    fs::read_to_string(&path).expect("Couldn't read from input file")
}

fn main() {
    let input = read_input("input/day07.txt");
    let (s1, s2) = solve(&input);
    println!("Part 1: {}", s1);
    println!("Part 2: {}", s2);
}
