use std::collections::{HashSet, HashMap};

pub fn solve(input: &str) -> (u128, u128) {
    let tachyon = parse_input(&input);
    let s1 = solve_part_1(&tachyon);
    let s2 = solve_part_2(&tachyon);
    (s1, s2)
}

struct Tachyon {
    start: usize,
    layers: Vec<Vec<usize>>,
}

fn parse_input(input: &str) -> Tachyon {
    let mut layers = Vec::new();
    let lines: Vec<&str> = input.lines().collect();

    let start = lines[0].find('S').unwrap();

    for i in 1..lines.len() {
        let mut layer = Vec::new();
        for (j, c) in  lines[i].char_indices() {
            if c == '^' {
                layer.push(j);
            }
        }
        layers.push(layer);
    }
    Tachyon { start, layers }
}

fn solve_part_1(tachyon: &Tachyon) -> u128 {
    let mut beams = HashSet::new();
    beams.insert(tachyon.start);
    let mut n_splits = 0;

    for layer in &tachyon.layers {
        let mut new_beams = HashSet::new();
        if layer.len() == 0 {
            continue;
        }

        for beam in beams {
            if layer.contains(&beam) {
                new_beams.insert(beam - 1);
                new_beams.insert(beam + 1);
                n_splits += 1;
            }
            else {
                new_beams.insert(beam);
            }
        }
        beams = new_beams;
    }
    n_splits
}

fn solve_part_2(tachyon: &Tachyon) -> u128 {
    let mut beams = HashMap::new();
    beams.insert(tachyon.start, 1);

    for layer in &tachyon.layers {
        let mut new_beams = HashMap::new();
        if layer.len() == 0 {
            continue;
        }

        for (beam, count) in beams {
            if layer.contains(&beam) {
                *new_beams.entry(beam - 1).or_default() += count;
                *new_beams.entry(beam + 1).or_default() += count;
            }
            else {
                *new_beams.entry(beam).or_default() += count;
            }
        }
        beams = new_beams;
    }

    let mut timelines = 0;
    for (_beam, count) in beams {
        timelines += count;
    }
    timelines
}
