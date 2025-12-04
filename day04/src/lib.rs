fn parse_input(input: &str) -> Vec<Vec<char>> {
    let mut grid = Vec::new();
    for (_line_num, line) in input.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let line = line.chars().collect();
        grid.push(line);
    }

    grid
}

fn get_neighbors8(row_idx: usize, col_idx: usize, grid: &Vec<Vec<char>>) ->  Vec<(usize, usize)> {
    let directions = [
        (-1, -1), (-1,  0), (-1,  1),
        ( 0, -1),           ( 0,  1),
        ( 1, -1), ( 1,  0), ( 1,  1),
    ];

    let mut result = Vec::new();

    for (dr, dc) in directions {
        let new_r = row_idx as isize + dr;
        let new_c = col_idx as isize + dc;

        if new_r >= 0
            && new_c >= 0
            && (new_r as usize) < grid.len()
            && (new_c as usize) < grid[new_r as usize].len()
        {
            result.push((new_r as usize, new_c as usize));
        }
    }
    result
}

fn check_roll_movable(row_idx: usize, col_idx: usize, grid: &Vec<Vec<char>>) -> bool {
    let mut neighbors_rolls = 0;
    let neighbors: Vec<char> = get_neighbors8(row_idx, col_idx, grid)
        .into_iter()
        .map(|(nr, nc)| grid[nr][nc])
        .collect();
    for neighbor in neighbors {
        if neighbor == '@' {
            neighbors_rolls += 1;
        }
    }
    if grid[row_idx][col_idx] == '@' && neighbors_rolls < 4 {
        return true;
    }

    false
}

fn solve_part_1(grid: &Vec<Vec<char>>) -> u64 {
    let mut total = 0;
    
    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, _char) in row.iter().enumerate() {
            if check_roll_movable(row_idx, col_idx, grid) {
                total += 1;
            }
        }
    }
    total
}

fn solve_part_2(grid: &Vec<Vec<char>>) -> u64 {
    let mut grid = grid.clone();
    let mut total = 0;

    loop {
        let mut rolls_to_move: Vec<(usize, usize)> = Vec::new();
        for (row_idx, row) in grid.iter().enumerate() {
            for (col_idx, _char) in row.iter().enumerate() {
                if check_roll_movable(row_idx, col_idx, &grid) {
                    rolls_to_move.push((row_idx, col_idx));
                }
            }
        }
        if rolls_to_move.len() == 0 {
            break
        }
        total += rolls_to_move.len() as u64;
        for (r, c) in rolls_to_move {
            grid[r][c] = '.';
        }
    }
    total
}

pub fn solve(input: &str) -> (u64, u64) {
    let input = parse_input(input);
    let part_1_solution = solve_part_1(&input);
    let part_2_solution = solve_part_2(&input);
    (part_1_solution, part_2_solution)
}
