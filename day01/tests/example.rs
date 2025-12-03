use day01::solve;

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
    let (p1, p2) = solve(example);
    assert_eq!(p1, 3);
    assert_eq!(p2, 6);
}
