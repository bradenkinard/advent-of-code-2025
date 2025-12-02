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

fn rotate_to_position(current_location: i32, cmd: &Command) -> Result<i32> {
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

fn count_times_zero_is_passed(current_location: i32, cmd: &Command) -> Result<i32> {
    // No rotation case
    if cmd.steps == 0 {
        return Ok(0);
    }
    // Less than a full turn from 0 case
    if (current_location == 0) & (cmd.steps < 100) {
        return Ok(0);
    }
    let mut distance_to_zero = current_location;
    if cmd.direction == Direction::R {
        distance_to_zero = 100 - current_location;
    }
    else if current_location == 0 {
        distance_to_zero = 100;
    }
    if cmd.steps < distance_to_zero {
        return Ok(0);
    }

    let mut times_at_zero = 1;
    let adjusted_steps = cmd.steps - distance_to_zero;
    times_at_zero += adjusted_steps / 100;
    Ok(times_at_zero)
}

fn solve_part_1(commands: &Vec<Command>) -> Result<i32> {
    let mut location = 50;
    let mut zeros = 0;
    for cmd in commands {
        location = rotate_to_position(location, cmd)?;
        if location == 0 {
            zeros += 1;
        }
    }
    Ok(zeros)
}

fn solve_part_2(commands: &Vec<Command>) -> Result<i32> {
   let mut location = 50;
   let mut times_at_zero = 0;
   for cmd in commands {
      times_at_zero += count_times_zero_is_passed(location, cmd)?;
      location = rotate_to_position(location, cmd)?;
   }
   Ok(times_at_zero)
}

pub fn solve(input: &str) -> Result<(i32, i32)> {
    let commands = parse_commands(input)?;
    let part_1 = solve_part_1(&commands)?;
    let part_2 = solve_part_2(&commands)?;
    Ok((part_1, part_2))
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
        let result_1 = rotate_to_position(50, &cmd).unwrap();
        let result_2 = rotate_to_position(82, &cmd2).unwrap();
        assert_eq!(result_1, 82);
        assert_eq!(result_2, 52);
    }

    #[test]
    fn passed_zero_left() {
        let cmd = &Command { direction: Direction::L, steps: 68 };
        let result = count_times_zero_is_passed(50, cmd).unwrap();
        assert_eq!(result, 1);
    }

    #[test]
    fn passed_zero_left_twice() {
        let cmd = &Command { direction: Direction::L, steps: 168 };
        let result = count_times_zero_is_passed(50, cmd).unwrap();
        assert_eq!(result, 2);
    }

    #[test]
    fn passed_zero_right() {
        let cmd = &Command { direction: Direction::R, steps: 68 };
        let result = count_times_zero_is_passed(50, cmd).unwrap();
        assert_eq!(result, 1);
    }

    #[test]
    fn passed_zero_right_twice() {
        let cmd = &Command { direction: Direction::R, steps: 168 };
        let result = count_times_zero_is_passed(50, cmd).unwrap();
        assert_eq!(result, 2);
    }

    #[test]
    fn passed_zero_from_zero_right() {
        let cmd = &Command { direction: Direction::R, steps: 100 };
        let result = count_times_zero_is_passed(0, cmd).unwrap();
        assert_eq!(result, 1);
    }

    #[test]
    fn passed_zero_from_zero_right_twice() {
        let cmd = &Command { direction: Direction::R, steps: 200 };
        let result = count_times_zero_is_passed(0, cmd).unwrap();
        assert_eq!(result, 2);
    }

    #[test]
    fn passed_zero_from_zero_left() {
        let cmd = &Command { direction: Direction::L, steps: 100 };
        let result = count_times_zero_is_passed(0, cmd).unwrap();
        assert_eq!(result, 1);
    }

    #[test]
    fn passed_zero_from_zero_left_twice() {
        let cmd = &Command { direction: Direction::L, steps: 200 };
        let result = count_times_zero_is_passed(0, cmd).unwrap();
        assert_eq!(result, 2);
    }
}