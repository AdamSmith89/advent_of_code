use advent_of_code::year2019::day09::*;

const PUZZLE_INPUT: &str = include_str!("../../input/year2019/day09.txt");

// No examples specific to the day, they are covered by ICC tests
// const EXAMPLE_INPUT: &str = "\
// ";

// #[test]
// fn parse_example() {
//     let parsed = parse(EXAMPLE_INPUT).expect("Error parsing input");

//     assert_eq!(todo!("expected"), todo!("actual"));
// }

// #[test]
// fn part1_example() {
//     let input = parse(EXAMPLE_INPUT).expect("Error parsing input");
//     let answer = part1(&input).expect("Error solving part 1");

//     assert_eq!(todo!("expected"), todo!("actual"));
// }

#[test]
fn part1_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let answer = part1(&parsed).expect("Error solving part 1");

    assert_eq!(3013554615, answer);
}

// #[test]
// fn part2_example() {
//     let input = parse(EXAMPLE_INPUT).expect("Error parsing example input");
//     let answer = part2(&input).expect("Error solving part 2");

//     assert_eq!(todo!("expected"), todo!("actual"));
// }

#[test]
fn part2_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let answer = part2(&parsed).expect("Error solving part 2");

    assert_eq!(50158, answer);
}
