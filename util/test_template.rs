use advent_of_code::yearYYYY::dayDD::*;

const EXAMPLE: &str = "\
";

#[test]
fn parse_test() {
    let parsed = parse(EXAMPLE).expect("Error parsing input");

    assert_eq!(parsed, todo!());
}

#[test]
fn part1_example() {
    let input = parse(EXAMPLE).expect("Error parsing input");
    let answer = part1(&input).expect("Error solving part 1");

    assert_eq!(answer, todo!());
}

#[test]
fn part1_real() {
    let input = include_str!("../../input/yearYYYY/dayDD.txt");
    let parsed = parse(input).expect("Error parsing input");
    let answer = part1(&parsed).expect("Error solving part 1");

    assert_eq!(answer, todo!());
}

#[test]
fn part2_example() {
    let input = parse(EXAMPLE).expect("Error parsing example input");
    let answer = part2(&input).expect("Error solving part 2");

    assert_eq!(answer, todo!());
}

#[test]
fn part2_real() {
    let input = include_str!("../../input/yearYYYY/dayDD.txt");
    let parsed = parse(input).expect("Error parsing input");
    let answer = part2(&parsed).expect("Error solving part 2");

    assert_eq!(answer, todo!());
}
