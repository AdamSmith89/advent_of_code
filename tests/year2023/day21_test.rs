use advent_of_code::year2023::day21::*;

const PUZZLE_INPUT: &str = include_str!("../../input/year2023/day21.txt");
const EXAMPLE_INPUT: &str = "\
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

#[test]
fn parse_example() {
    let parsed = parse(EXAMPLE_INPUT).expect("Error parsing input");

    assert_eq!(parsed, todo!());
}

#[test]
fn part1_example() {
    let mut input = parse(EXAMPLE_INPUT).expect("Error parsing input");

    // We only have example output for 6 steps
    input.steps_override = Some(6);

    let answer = part1(&input).expect("Error solving part 1");

    assert_eq!(answer, 16);
}

#[test]
fn part1_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let answer = part1(&parsed).expect("Error solving part 1");

    assert_eq!(answer, todo!());
}

#[test]
fn part2_example() {
    let input = parse(EXAMPLE_INPUT).expect("Error parsing example input");
    let answer = part2(&input).expect("Error solving part 2");

    assert_eq!(answer, todo!());
}

#[test]
fn part2_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let answer = part2(&parsed).expect("Error solving part 2");

    assert_eq!(answer, todo!());
}
