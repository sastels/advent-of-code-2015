use advent_2015::utils::{join_lines, read_lines, Grid};

#[test]
fn utils_join_lines() {
    let data = read_lines("./tests/utils_test_data.txt");
    let joined = join_lines(&data);
    assert_eq!(joined.len(), 4);
    assert_eq!(joined[0], "a bb ccc");
    assert_eq!(joined[3], "123 456")
}

#[test]
fn get_set_grid() {
    let mut grid = Grid::<i32>::new(10, -1);
    assert_eq!(grid.count_equals(-1), 100);
    assert_eq!(grid.count_equals(15), 0);
    assert_eq!(*grid.get(2, 1), -1);
    grid.set(2, 1, 15);
    assert_eq!(grid.count_equals(15), 1);

    assert_eq!(*grid.get(2, 1), 15);
}
