use advent_of_code::year2019::day02::*;

const PUZZLE_INPUT: &str = include_str!("../../input/year2019/day02.txt");
const EXAMPLE_INPUT: &str = "1,9,10,3,2,3,11,0,99,30,40,50,60";

#[test]
fn parse_example() {
    let parsed = parse(EXAMPLE_INPUT).expect("Error parsing input");

    assert_eq!(parsed, vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50, 60]);
}

#[test]
fn part1_example() {
    let input = parse(EXAMPLE_INPUT).expect("Error parsing input");
    let answer = part1(&input).expect("Error solving part 1");

    assert_eq!(answer, 3100); // Subtly different from example as code replaces [1] with 12, and [2] with 2
}

#[test]
fn part1_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let answer = part1(&parsed).expect("Error solving part 1");

    assert_eq!(answer, 6327510);
}

// There isn't an example for part 2
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

    assert_eq!(answer, 4112);
}
