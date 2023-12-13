use advent_of_code::year2023::day11::*;
use grid::grid;
use itertools::Itertools;

const PUZZLE_INPUT: &str = include_str!("../../input/year2023/day11.txt");
const EXAMPLE_INPUT: &str = "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

#[test]
fn parse_example() {
    let parsed = parse(EXAMPLE_INPUT).expect("Error parsing input");

    let mut grid = grid![];
    grid.push_row("..*.#.*...*..".chars().collect_vec());
    grid.push_row("..*...*..#*..".chars().collect_vec());
    grid.push_row("#.*...*...*..".chars().collect_vec());
    grid.push_row("*************".chars().collect_vec());
    grid.push_row("..*...*...*..".chars().collect_vec());
    grid.push_row("..*...*.#.*..".chars().collect_vec());
    grid.push_row(".#*...*...*..".chars().collect_vec());
    grid.push_row("..*...*...*.#".chars().collect_vec());
    grid.push_row("*************".chars().collect_vec());
    grid.push_row("..*...*...*..".chars().collect_vec());
    grid.push_row("..*...*..#*..".chars().collect_vec());
    grid.push_row("#.*..#*...*..".chars().collect_vec());

    assert_eq!(parsed, Universe { grid });
}

#[test]
fn part1_example() {
    let input = parse(EXAMPLE_INPUT).expect("Error parsing input");
    let answer = part1(&input).expect("Error solving part 1");

    assert_eq!(answer, 374);
}

#[test]
fn part1_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let answer = part1(&parsed).expect("Error solving part 1");

    assert_eq!(answer, 9521776);
}

#[test]
fn part2_example() {
    let input = parse(EXAMPLE_INPUT).expect("Error parsing example input");
    let answer = part2(&input).expect("Error solving part 2");

    // 10 times larger = 1030
    // 100 times larger = 8410
    // 1000000 times larger = 82000210
    assert_eq!(answer, 82000210);
}

#[test]
fn part2_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let answer = part2(&parsed).expect("Error solving part 2");

    assert_eq!(answer, 553224415344);
}
