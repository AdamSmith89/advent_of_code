use advent_of_code::year2024::day06::*;

const PUZZLE_INPUT: &str = include_str!("../../input/year2024/day06.txt");
const EXAMPLE_INPUT: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

#[test]
fn parse_example() {
    let actual = parse(EXAMPLE_INPUT).expect("Error parsing input");

    assert_eq!(Some(&'.'), actual.get(0, 0));
    assert_eq!(Some(&'#'), actual.get(0, 4));
    assert_eq!(Some(&'^'), actual.get(6, 4));
    assert_eq!(Some(&'.'), actual.get(9, 9));
}

#[test]
fn part1_example() {
    let input = parse(EXAMPLE_INPUT).expect("Error parsing input");
    let actual = part1(&input).expect("Error solving part 1");

    assert_eq!(41, actual);
}

#[test]
fn part1_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let actual = part1(&parsed).expect("Error solving part 1");

    assert_eq!(5516, actual);
}

#[test]
fn part2_example() {
    let input = parse(EXAMPLE_INPUT).expect("Error parsing example input");
    let actual = part2(&input).expect("Error solving part 2");

    assert_eq!(6, actual);
}

#[test]
fn part2_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let _actual = part2(&parsed).expect("Error solving part 2");

    //assert_eq!(expected, actual);
}
