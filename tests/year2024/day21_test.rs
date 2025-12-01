use advent_of_code::year2024::day21::*;

const PUZZLE_INPUT: &str = include_str!("../../input/year2024/day21.txt");
const EXAMPLE_INPUT: &str = "\
029A
980A
179A
456A
379A";

#[test]
fn parse_example() {
    let actual = parse(EXAMPLE_INPUT).expect("Error parsing input");

    assert_eq!(
        vec![
            String::from("029A"),
            String::from("980A"),
            String::from("179A"),
            String::from("456A"),
            String::from("379A"),
        ],
        actual
    );
}

#[test]
fn part1_example() {
    let input = parse(EXAMPLE_INPUT).expect("Error parsing input");
    let actual = part1(&input).expect("Error solving part 1");

    assert_eq!(126384, actual);
}

#[test]
fn part1_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    //let _actual = part1(&parsed).expect("Error solving part 1");

    //assert_eq!(expected, actual);
}

#[test]
fn part2_example() {
    let input = parse(EXAMPLE_INPUT).expect("Error parsing example input");
    let _actual = part2(&input).expect("Error solving part 2");

    //assert_eq!(expected, actual);
}

#[test]
fn part2_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    //let _actual = part2(&parsed).expect("Error solving part 2");

    //assert_eq!(expected, actual);
}
