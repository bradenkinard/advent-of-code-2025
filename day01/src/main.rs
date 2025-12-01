use std::fs;
use anyhow::{anyhow, Result};

struct Command {
    direction: Direction,
    steps: i32,
}

#[derive(PartialEq)]
enum Direction {
    R,
    L,
}

fn read_input(path: &str) -> Result<String> {
    Ok(fs::read_to_string(path)?)
}

fn parse_commands(input: &str) -> Result<Vec<Command>> {
    let mut commands = Vec::new();

    for (line_num, line) in input.lines().enumerate() {
        let trimmed = line.trim();

        // Skip empty lines
        if trimmed.is_empty() {
            continue;
        }

        // First character must be direction
        let (dir_char, steps_str) = trimmed.split_at(1);

        let direction = match dir_char {
            "R" => Direction::R,
            "L" => Direction::L,
            _ => return Err(anyhow!("Invalid direction '{}' on line {}", dir_char, line_num + 1)),
        };

        let steps: i32 = steps_str.parse().map_err(|_| {
            anyhow!("Invalid step count '{}' on line {}", steps_str, line_num + 1)
        })?;

        commands.push(Command { direction, steps });
    }
    Ok(commands)
}

fn determine_location(current_location: i32, cmd: &Command) -> Result<i32> {
    let mut location = current_location;
    if cmd.direction == Direction::R {
            location += cmd.steps;
            location = location % 100;

        } else {
            location -= cmd.steps;
            location = location % 100;
            if location < 0 {
                location = 100 + location;
            }
        }
    Ok(location)
}

fn solve_part_1(commands: &Vec<Command>) -> Result<i32> {
    let mut location = 50;
    let mut zeros = 0;
    for cmd in commands {
        location = determine_location(location, cmd)?;
        if location == 0 {
            zeros += 1;
        }
    }
    Ok(zeros)
}

fn solve_part_2(commands: &Vec<Command>) -> Result<i32> {
    Ok(0)
}

fn solve(input: &str) -> Result<(i32, i32)> {
    // TODO: parse and solve
    let commands = parse_commands(input)?;
    let part_1 = solve_part_1(&commands)?;
    let part_2 = solve_part_2(&commands)?;
    Ok((part_1, part_2))
}

fn main() -> Result<()> {
    let input = read_input("input/day01.txt")?;
    let (part1, part2) = solve(&input)?;
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_rotate_left() {
        let cmd = Command {
            direction: Direction::L,
            steps: 68,
        };
        let cmd2 = Command {
            direction: Direction::L,
            steps: 30,
        };
        let result_1 = determine_location(50, &cmd).unwrap();
        let result_2 = determine_location(82, &cmd2).unwrap();
        assert_eq!(result_1, 82);
        assert_eq!(result_2, 52);
    }

    #[test]
    fn example_solution() {
        let example = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";
        let (p1, p2) = solve(example).unwrap();
        assert_eq!(p1, 3);
        assert_eq!(p2, 0);
    }
}