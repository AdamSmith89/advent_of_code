use advent_of_code::YYYY::DD::*;

const PUZZLE_INPUT: &str = include_str!("../../input/YYYY/DD.txt");
const EXAMPLE_INPUT: &str = "\
";

#[test]
fn parse_example() {
    let _actual = parse(EXAMPLE_INPUT).expect("Error parsing input");

    //assert_eq!(expected, actual);
}

#[test]
fn part1_example() {
    let input = parse(EXAMPLE_INPUT).expect("Error parsing input");
    let _actual = part1(&input).expect("Error solving part 1");

    //assert_eq!(expected, actual);
}

#[test]
fn part1_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let _actual = part1(&parsed).expect("Error solving part 1");

    //assert_eq!(expected, actual);
}

#[test]
fn part2_example() {
    let input = parse(EXAMPLE_INPUT).expect("Error parsing example input");
    let _actual = part2(&input).expect("Error solving part 2");

    //assert_eq!(expected, actual);
}

#[test]
fn part2_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let _actual = part2(&parsed).expect("Error solving part 2");

    //assert_eq!(expected, actual);
}
