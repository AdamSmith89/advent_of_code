use advent_of_code::{util::grid::Grid, year2019::day10::*};
use grid::grid;
use simple_logger::SimpleLogger;

const PUZZLE_INPUT: &str = include_str!("../../input/year2019/day10.txt");
const EXAMPLE_INPUT_PART1: &str = "\
.#..#
.....
#####
....#
...##";

const EXAMPLE_INPUT_PART2: &str = "\
.#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";

#[test]
fn parse_example() {
    let parsed = parse(EXAMPLE_INPUT_PART1).expect("Error parsing input");

    assert_eq!(
        Grid::from(grid![
        ['.', '#', '.', '.', '#']
        ['.', '.', '.', '.', '.']
        ['#', '#', '#', '#', '#']
        ['.', '.', '.', '.', '#']
        ['.', '.', '.', '#', '#']]),
        parsed
    );
}

#[test]
fn part1_example() {
    let input = parse(EXAMPLE_INPUT_PART1).expect("Error parsing input");
    let answer = part1(&input).expect("Error solving part 1");

    assert_eq!(8, answer);
}

#[test]
fn part1_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let answer = part1(&parsed).expect("Error solving part 1");

    assert_eq!(329, answer);
}

#[test]
fn part2_example() {
    let input = parse(EXAMPLE_INPUT_PART2).expect("Error parsing example input");
    let answer = part2(&input).expect("Error solving part 2");

    assert_eq!(802, answer);
}

#[test]
fn part2_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let answer = part2(&parsed).expect("Error solving part 2");

    assert_eq!(512, answer);
}
