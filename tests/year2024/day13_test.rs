use advent_of_code::year2024::day13::*;

const PUZZLE_INPUT: &str = include_str!("../../input/year2024/day13.txt");
const EXAMPLE_INPUT: &str = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

#[test]
fn parse_example() {
    let actual = parse(EXAMPLE_INPUT).expect("Error parsing input");
    let expected = vec![
        Game::new((94, 34), (22, 67), (8400, 5400)),
        Game::new((26, 66), (67, 21), (12748, 12176)),
        Game::new((17, 86), (84, 37), (7870, 6450)),
        Game::new((69, 23), (27, 71), (18641, 10279)),
    ];

    assert_eq!(expected, actual);
}

#[test]
fn part1_example() {
    let input = parse(EXAMPLE_INPUT).expect("Error parsing input");
    let actual = part1(&input).expect("Error solving part 1");

    assert_eq!(480, actual);
}

#[test]
fn part1_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let actual = part1(&parsed).expect("Error solving part 1");

    assert_eq!(29436, actual);
}

// No example answer given
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

    assert_eq!(103729094227877, actual);
}
