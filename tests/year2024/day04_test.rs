use advent_of_code::year2024::day04::*;

const PUZZLE_INPUT: &str = include_str!("../../input/year2024/day04.txt");
const EXAMPLE_INPUT: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

#[test]
fn parse_example() {
    let actual = parse(EXAMPLE_INPUT).expect("Error parsing input");

    assert_eq!(Some(&'M'), actual.get(0, 0));
    assert_eq!(Some(&'M'), actual.get(3, 8));
    assert_eq!(Some(&'S'), actual.get(6, 4));
    assert_eq!(Some(&'X'), actual.get(9, 9));
}

#[test]
fn part1_example() {
    let input = parse(EXAMPLE_INPUT).expect("Error parsing input");
    let actual = part1(&input).expect("Error solving part 1");

    assert_eq!(18, actual);
}

#[test]
fn part1_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let actual = part1(&parsed).expect("Error solving part 1");

    assert_eq!(2642, actual);
}

#[test]
fn part2_example() {
    let input = parse(EXAMPLE_INPUT).expect("Error parsing example input");
    let actual = part2(&input).expect("Error solving part 2");

    assert_eq!(9, actual);
}

#[test]
fn part2_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let actual = part2(&parsed).expect("Error solving part 2");

    assert_eq!(1974, actual);
}
