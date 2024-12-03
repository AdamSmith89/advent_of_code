use advent_of_code::year2024::day03::*;

const PUZZLE_INPUT: &str = include_str!("../../input/year2024/day03.txt");
const EXAMPLE_INPUT_PART1: &str =
    "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
const EXAMPLE_INPUT_PART2: &str =
    "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

#[test]
fn parse_example() {
    let actual = parse(EXAMPLE_INPUT_PART1).expect("Error parsing input");

    assert_eq!(EXAMPLE_INPUT_PART1.to_string(), actual);
}

#[test]
fn part1_example() {
    let input = parse(EXAMPLE_INPUT_PART1).expect("Error parsing input");
    let actual = part1(&input).expect("Error solving part 1");

    assert_eq!(161, actual);
}

#[test]
fn part1_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let actual = part1(&parsed).expect("Error solving part 1");

    assert_eq!(188192787, actual);
}

#[test]
fn part2_example() {
    let input = parse(EXAMPLE_INPUT_PART2).expect("Error parsing example input");
    let actual = part2(&input).expect("Error solving part 2");

    assert_eq!(48, actual);
}

#[test]
fn part2_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let actual = part2(&parsed).expect("Error solving part 2");

    assert_eq!(113965544, actual);
}
