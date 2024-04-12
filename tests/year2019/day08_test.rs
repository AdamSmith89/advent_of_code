use advent_of_code::year2019::day08::*;

const PUZZLE_INPUT: &str = include_str!("../../input/year2019/day08.txt");
//const EXAMPLE_INPUT: &str = "123456789012";

// ToDo: how best to test when the layer size is embedded in the parse function?
// #[test]
// fn parse_example() {
//     let parsed = parse(EXAMPLE_INPUT).expect("Error parsing input");

//     assert_eq!(
//         vec![
//             Layer::new(vec![1, 2, 3, 4, 5, 6], 25, 6),
//             Layer::new(vec![7, 8, 9, 0, 1, 2], 25, 6),
//         ],
//         parsed
//     );
// }

// #[test]
// fn part1_example() {
//     let input = parse(EXAMPLE_INPUT).expect("Error parsing input");
//     let answer = part1(&input).expect("Error solving part 1");
//
//     assert_eq!(1, answer);
// }

#[test]
fn part1_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let answer = part1(&parsed).expect("Error solving part 1");

    assert_eq!(2210, answer);
}

// No easily verifiable example for this puzzle
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

    assert_eq!("CGEGE", answer);
}
