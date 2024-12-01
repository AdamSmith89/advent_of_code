use advent_of_code::year2024::day01::*;

const PUZZLE_INPUT: &str = include_str!("../../input/year2024/day01.txt");
const EXAMPLE_INPUT: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3";

#[test]
fn parse_example() {
    let parsed = parse(EXAMPLE_INPUT).expect("Error parsing input");
    let expected = (vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3]);

    assert_eq!(expected, parsed);
}

#[test]
fn part1_example() {
    let input = parse(EXAMPLE_INPUT).expect("Error parsing input");
    let answer = part1(&input).expect("Error solving part 1");

    assert_eq!(11, answer);
}

#[test]
fn part1_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let answer = part1(&parsed).expect("Error solving part 1");

    assert_eq!(2756096, answer);
}

#[test]
fn part2_example() {
    let input = parse(EXAMPLE_INPUT).expect("Error parsing example input");
    let answer = part2(&input).expect("Error solving part 2");

    assert_eq!(31, answer);
}

#[test]
fn part2_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let _answer = part2(&parsed).expect("Error solving part 2");

    //assert_eq!(expected, actual);
}
