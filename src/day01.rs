pub fn solve_a(data: &[String]) -> i32 {
    data[0].chars().map(|c| if c == '(' { 1 } else { -1 }).sum()
}

pub fn solve_b(data: &[String]) -> usize {
    1 + data[0]
        .chars()
        .map(|c| if c == '(' { 1 } else { -1 })
        .scan(0, |state, x| {
            *state = *state + x;
            Some(*state)
        })
        .enumerate()
        .filter(|(_, x)| *x == -1)
        .next()
        .unwrap()
        .0
}
