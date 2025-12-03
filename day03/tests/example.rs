use day03::solve;

#[test]
fn example_solution() {
    let example = "\
987654321111111
811111111111119
234234234234278
818181911112111";
    let (p1, p2) = solve(example).unwrap();
    assert_eq!(p1, 0);
    assert_eq!(p2, 0);
}