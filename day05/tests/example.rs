use day05::solve;

#[test]
fn example_solution() {
    let example = "\
    3-5
10-14
16-20
12-18

1
5
8
11
17
32";
    let (p1, p2) = solve(&example);
    assert_eq!(p1, 3);
    assert_eq!(p2, 14);
}