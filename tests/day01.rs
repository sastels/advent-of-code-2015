use advent_2015::day01::{solve_a, solve_b};
use advent_2015::utils::read_lines;

#[test]
fn a() {
    let data = read_lines("./data/day01.txt");
    assert_eq!(solve_a(&data), 232);
}

#[test]
fn b() {
    let data = read_lines("./data/day01.txt");
    assert_eq!(solve_b(&data), 1783);
}
