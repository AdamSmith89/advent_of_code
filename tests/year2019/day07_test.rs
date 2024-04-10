use advent_of_code::year2019::day07::*;

const PUZZLE_INPUT: &str = include_str!("../../input/year2019/day07.txt");
const EXAMPLE_INPUT: &str = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";

#[test]
fn parse_example() {
    let parsed = parse(EXAMPLE_INPUT).expect("Error parsing input");

    assert_eq!(
        vec![3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0],
        parsed
    );
}

#[test]
fn part1_example1() {
    let answer = part1(&vec![
        3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
    ])
    .expect("Error solving part 1");

    assert_eq!(43210, answer);
}

#[test]
fn part1_example2() {
    let answer = part1(&vec![
        3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23, 99,
        0, 0,
    ])
    .expect("Error solving part 1");

    assert_eq!(54321, answer);
}

#[test]
fn part1_example3() {
    let answer = part1(&vec![
        3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1, 33,
        31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
    ])
    .expect("Error solving part 1");

    assert_eq!(65210, answer);
}

#[test]
fn part1_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let answer = part1(&parsed).expect("Error solving part 1");

    assert_eq!(14902, answer);
}

#[test]
fn part2_example1() {
    let answer = part2(&vec![
        3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1, 28,
        1005, 28, 6, 99, 0, 0, 5,
    ])
    .expect("Error solving part 2");

    assert_eq!(139629729, answer);
}

#[test]
fn part2_example2() {
    let answer = part2(&vec![
        3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54, -5,
        54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4, 53,
        1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
    ])
    .expect("Error solving part 2");

    assert_eq!(18216, answer);
}

#[test]
fn part2_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let answer = part2(&parsed).expect("Error solving part 2");

    assert_eq!(6489132, answer);
}
