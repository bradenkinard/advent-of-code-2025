use day09::solve;

#[test]
fn test_example() {
    let example = "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    let (s1, s2) = solve(&example);
    assert_eq!(s1, 50);
    assert_eq!(s2, 0);
}