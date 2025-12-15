pub fn solve(input: &str) -> (u128, u128) {
    let coords = parse_input(input);
    let s1 = solve_part_1(&coords);
    let s2 = solve_part_2(&coords);
    (s1, s2)
}

fn parse_input(input: &str) -> Vec<(u128, u128)> {
    let mut coords = Vec::new();
    for (_line_num, line) in input.lines().enumerate() {
        let (x, y) = line.split_once(",").unwrap();
        let x = x.parse().unwrap();
        let y = y.parse().unwrap();
        coords.push((x, y));
    }
    coords
}

fn solve_part_1(coords: &Vec<(u128, u128)>) -> u128 {
    let mut max_area = 0;
    for i in 0..coords.len() {
        for j in 0..coords.len() {
            if i == j {
                continue
            }
            let w = coords[j].0.abs_diff(coords[i].0) + 1;
            let h = coords[j].1.abs_diff(coords[i].1) + 1;
            let area = w * h;
            if area > max_area {
                max_area = area;
            }
        }
    }
    max_area
}

fn solve_part_2(coords: &Vec<(u128, u128)>) -> u128 {
    0
}