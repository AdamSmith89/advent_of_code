use advent_of_code::year2019::day04::*;

const PUZZLE_INPUT: &str = include_str!("../../input/year2019/day04.txt");
const EXAMPLE_INPUT: &str = "245182-790572";

#[test]
fn parse_example() {
    let parsed = parse(EXAMPLE_INPUT).expect("Error parsing input");

    assert_eq!(parsed, ([2, 4, 5, 1, 8, 2], [7, 9, 0, 5, 7, 2]));
}

// No example for this puzzle
// #[test]
// fn part1_example() {
//     let input = parse(EXAMPLE_INPUT).expect("Error parsing input");
//     let answer = part1(&input).expect("Error solving part 1");

//     assert_eq!(answer, todo!());
// }

#[test]
fn part1_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let answer = part1(&parsed).expect("Error solving part 1");

    assert_eq!(answer, 1099);
}

// No example for this puzzle
// #[test]
// fn part2_example() {
//     let input = parse(EXAMPLE_INPUT).expect("Error parsing example input");
//     let answer = part2(&input).expect("Error solving part 2");

//     assert_eq!(answer, todo!());
// }

#[test]
fn part2_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let answer = part2(&parsed).expect("Error solving part 2");

    assert_eq!(answer, 710);
}
