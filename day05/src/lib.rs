struct Inputs {
    fresh_id_ranges: Vec<(u128, u128)>,
    available_ids: Vec<u128>,
}

fn parse_ranges(ranges_str: &str) -> Vec<(u128, u128)> {
    let mut ranges = Vec::new();
    for (_line_num, line) in ranges_str.lines().enumerate() {
        let (start, end) = line.split_once("-").unwrap();
        let start = start.parse().unwrap();
        let end = end.parse().unwrap();
        ranges.push((start, end));
    }
    ranges = merge_ranges(ranges);
    ranges
}

fn merge_ranges(mut ranges: Vec<(u128, u128)>) -> Vec<(u128, u128)> {
    ranges.sort_by(|a, b| a.cmp(b));

    let mut merged = Vec::new();
    let mut current = ranges[0];

    for (start, end) in ranges.into_iter().skip(1) {
        if start <= current.1 {
            current.1 = current.1.max(end)
        }
        else {
            merged.push(current);
            current = (start, end);
        }
    }
    merged.push(current);
    merged
}

fn parse_available_ingredients(available_ingredients_str: &str) -> Vec<u128> {
    let mut available_ingredients = Vec::new();
    for (_line_num, line) in available_ingredients_str.lines().enumerate() {
        let id = line.trim().parse().unwrap();
        available_ingredients.push(id);
    }
    available_ingredients
}

fn parse_input(input: &str) -> Inputs {
    let (id_range_str, available_ingredients_str) = input.split_once("\n\n").unwrap();
    let ranges = parse_ranges(id_range_str);
    let available_ingredients = parse_available_ingredients(available_ingredients_str);
    Inputs {fresh_id_ranges: ranges, available_ids: available_ingredients}
}

fn solve_part_1(inputs: &Inputs) -> usize {
    let mut fresh_available_ingredients = Vec::new();
    for ingredient_id in inputs.available_ids.iter().copied() {
        for (start, end) in inputs.fresh_id_ranges.iter().copied() {
            if start <= ingredient_id && ingredient_id <= end {
                fresh_available_ingredients.push(ingredient_id);
                break;
            }
        }
    }
    fresh_available_ingredients.len()
}

fn solve_part_2(inputs: &Inputs) -> u128 {
    let mut total_fresh_ids = 0;
    for (start, end) in inputs.fresh_id_ranges.iter().copied() {
        total_fresh_ids += end - start + 1;
    }
    total_fresh_ids
}

pub fn solve(input: &str) -> (usize, u128) {
    let input = parse_input(input);
    let part_1_solution = solve_part_1(&input);
    let part_2_solution = solve_part_2(&input);
    (part_1_solution, part_2_solution)
}
