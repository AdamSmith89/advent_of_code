use advent_of_code::year2023::day04::*;

const PUZZLE_INPUT: &str = include_str!("../../input/year2023/day04.txt");
const EXAMPLE_INPUT: &str = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

#[test]
fn parse_example() {
    let cards = parse(EXAMPLE_INPUT).expect("Error parsing example input");

    assert_eq!(cards.len(), 6);
    assert_eq!(cards[0].winners, vec![41, 48, 83, 86, 17]);
    assert_eq!(cards[1].numbers, vec![61, 30, 68, 82, 17, 32, 24, 19]);
    assert_eq!(cards[2].winners, vec![1, 21, 53, 59, 44]);
    assert_eq!(cards[2].numbers, vec![69, 82, 63, 72, 16, 21, 14, 1]);
}

#[test]
fn part1_example() {
    let input = parse(EXAMPLE_INPUT).expect("Failed to parse example input");
    let answer = part1(&input).expect("Error solving part 1");

    assert_eq!(answer, 13);
}

#[test]
fn part1_real() {
    let input = parse(PUZZLE_INPUT).expect("Failed to parse example input");
    let answer = part1(&input).expect("Error solving part 1");

    assert_eq!(answer, 26218);
}

#[test]
fn part2_example() {
    let input = parse(EXAMPLE_INPUT).expect("Failed to parse example input");
    let answer = part2(&input).expect("Error solving part 2");

    assert_eq!(answer, 30);
}

#[test]
fn part2_real() {
    let input = parse(PUZZLE_INPUT).expect("Failed to parse example input");
    let answer = part2(&input).expect("Error solving part 2");

    assert_eq!(answer, 9997537);
}
