use day06::solve;

#[test]
fn example_solution() {
    let example = "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
    let (s1, s2) = solve(&example);
    assert_eq!(s1, 4277556);
    assert_eq!(s2, 0);
}