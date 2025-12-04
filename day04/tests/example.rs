use day04::solve;

#[test]
fn example_solution() {
    let example = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
    let (p1, p2) = solve(example);
    assert_eq!(p1, 13);
    assert_eq!(p2, 43);
}
