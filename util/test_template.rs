use advent_of_code::yearYYYY::dayDD::*;

const EXAMPLE: &str = "\
";

#[test]
fn parse_test() {
    let parsed = parse(EXAMPLE).expect("Error parsing example input");
}

#[test]
fn part1_test() {
    let input = parse(EXAMPLE).expect("Error parsing example input");
    let answer = part1(&input).expect("Error solving part 1");

    assert_eq!(answer, todo!());
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE).expect("Error parsing example input");
    let answer = part2(&input).expect("Error solving part 2");

    assert_eq!(answer, todo!());
}
