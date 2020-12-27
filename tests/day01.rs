use advent_2015::day1::{solve_a, solve_b};
use advent_2015::utils::read_lines;


#[test]
fn a() {
    let data = read_lines("./data/day01_test.txt");
    assert_eq!(solve_a(&data), 0);
}

#[test]
fn b() {
    let data = read_lines("./data/day01_test.txt");
    assert_eq!(solve_b(&data), 0);