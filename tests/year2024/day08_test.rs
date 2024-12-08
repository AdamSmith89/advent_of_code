use advent_of_code::year2024::day08::*;

const PUZZLE_INPUT: &str = include_str!("../../input/year2024/day08.txt");
const EXAMPLE_INPUT: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

#[test]
fn parse_example() {
    let (rows, cols, antenna) = parse(EXAMPLE_INPUT).expect("Error parsing input");

    assert_eq!(12, rows);
    assert_eq!(12, cols);

    let zero_antenna = antenna.get(&'0');
    assert!(zero_antenna.is_some());
    assert_eq!(&vec![(1, 8), (2, 5), (3, 7), (4, 4)], zero_antenna.unwrap());

    let a_antenna = antenna.get(&'A');
    assert!(a_antenna.is_some());
    assert_eq!(&vec![(5, 6), (8, 8), (9, 9)], a_antenna.unwrap());
}

#[test]
fn part1_example() {
    let input = parse(EXAMPLE_INPUT).expect("Error parsing input");
    let actual = part1(&input).expect("Error solving part 1");

    assert_eq!(14, actual);
}

#[test]
fn part1_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let actual = part1(&parsed).expect("Error solving part 1");

    assert_eq!(344, actual);
}

#[test]
fn part2_example() {
    let input = parse(EXAMPLE_INPUT).expect("Error parsing example input");
    let actual = part2(&input).expect("Error solving part 2");

    assert_eq!(34, actual);
}

#[test]
fn part2_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let actual = part2(&parsed).expect("Error solving part 2");

    assert_eq!(1182, actual);
}
