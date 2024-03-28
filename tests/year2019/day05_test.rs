use advent_of_code::year2019::day05::*;

const PUZZLE_INPUT: &str = include_str!("../../input/year2019/day05.txt");
const EXAMPLE_INPUT: &str = "3,9,101,12,9,10,4,10,99,0,0";
// 3, 9             Input 1 into [9]
// 101, 12, 9, 10   Add 12 to [9], store at [10]
// 4, 10            Output [10]
// 99               End

#[test]
fn parse_example() {
    let parsed = parse(EXAMPLE_INPUT).expect("Error parsing input");

    assert_eq!(parsed, vec![3, 9, 101, 12, 9, 10, 4, 10, 99, 0, 0]);
}

#[test]
fn part1_example() {
    let input = parse(EXAMPLE_INPUT).expect("Error parsing input");
    let answer = part1(&input).expect("Error solving part 1");

    assert_eq!(answer, 13);
}

#[test]
fn part1_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let answer = part1(&parsed).expect("Error solving part 1");

    assert_eq!(answer, 14522484);
}

// Part 2 tests are handled by tests in int_code_computer.rs
// #[test]
// fn part2_example1() {
//     let input = parse(EXAMPLE_INPUT).expect("Error parsing example input");
//     let answer = part2(&input).expect("Error solving part 2");

//     assert_eq!(answer, todo!());
// }

#[test]
fn part2_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let answer = part2(&parsed).expect("Error solving part 2");

    assert_eq!(answer, 4655956);
}
