use advent_of_code::year2024::day07::*;

const PUZZLE_INPUT: &str = include_str!("../../input/year2024/day07.txt");
const EXAMPLE_INPUT: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

#[test]
fn parse_example() {
    let actual = parse(EXAMPLE_INPUT).expect("Error parsing input");

    let expected = vec![
        Equation::new(190, vec![10, 19]),
        Equation::new(3267, vec![81, 40, 27]),
        Equation::new(83, vec![17, 5]),
        Equation::new(156, vec![15, 6]),
        Equation::new(7290, vec![6, 8, 6, 15]),
        Equation::new(161011, vec![16, 10, 13]),
        Equation::new(192, vec![17, 8, 14]),
        Equation::new(21037, vec![9, 7, 18, 13]),
        Equation::new(292, vec![11, 6, 16, 20]),
    ];

    assert_eq!(expected, actual);
}

#[test]
fn part1_example() {
    let input = parse(EXAMPLE_INPUT).expect("Error parsing input");
    let actual = part1(&input).expect("Error solving part 1");

    assert_eq!(3749, actual);
}

#[test]
fn part1_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let actual = part1(&parsed).expect("Error solving part 1");

    assert_eq!(6231007345478, actual);
}

#[test]
fn part2_example() {
    let input = parse(EXAMPLE_INPUT).expect("Error parsing example input");
    let actual = part2(&input).expect("Error solving part 2");

    assert_eq!(11387, actual);
}

#[test]
fn part2_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let actual = part2(&parsed).expect("Error solving part 2");

    assert_eq!(333027885676693, actual);
}
