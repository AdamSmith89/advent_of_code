use advent_of_code::year2024::day11::*;

const PUZZLE_INPUT: &str = include_str!("../../input/year2024/day11.txt");
const EXAMPLE_INPUT: &str = "125 17";

#[test]
fn parse_example() {
    let actual = parse(EXAMPLE_INPUT).expect("Error parsing input");

    assert_eq!(2, actual.len());
    assert_eq!(Some(&1), actual.get(&125));
    assert_eq!(Some(&1), actual.get(&17));
}

#[test]
fn part1_example() {
    let input = parse(EXAMPLE_INPUT).expect("Error parsing input");
    let actual = part1(&input).expect("Error solving part 1");

    assert_eq!(55312, actual);
}

#[test]
fn part1_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let actual = part1(&parsed).expect("Error solving part 1");

    assert_eq!(194782, actual);
}

// No example answer provided
// #[test]
// fn part2_example() {
//     let input = parse(EXAMPLE_INPUT).expect("Error parsing example input");
//     let _actual = part2(&input).expect("Error solving part 2");

//     //assert_eq!(expected, actual);
// }

#[test]
fn part2_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let actual = part2(&parsed).expect("Error solving part 2");

    assert_eq!(233007586663131, actual);
}
