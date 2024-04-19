use advent_of_code::YYYY::DD::*;

const PUZZLE_INPUT: &str = include_str!("../../input/YYYY/DD.txt");
const EXAMPLE_INPUT: &str = "\
";

#[test]
fn parse_example() {
    let _parsed = parse(EXAMPLE_INPUT).expect("Error parsing input");

    todo!("assert_eq!(expected, parsed);")
}

#[test]
fn part1_example() {
    let input = parse(EXAMPLE_INPUT).expect("Error parsing input");
    let _answer = part1(&input).expect("Error solving part 1");

    todo!("assert_eq!(expected, answer);")
}

#[test]
fn part1_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let _answer = part1(&parsed).expect("Error solving part 1");

    todo!("assert_eq!(expected, actual);")
}

#[test]
fn part2_example() {
    let input = parse(EXAMPLE_INPUT).expect("Error parsing example input");
    let _answer = part2(&input).expect("Error solving part 2");

    todo!("assert_eq!(expected, actual);")
}

#[test]
fn part2_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let _answer = part2(&parsed).expect("Error solving part 2");

    todo!("assert_eq!(expected, actual);")
}
