use advent_of_code::util::grid::Grid;
use advent_of_code::year2023::day13::*;
use grid::grid;

const PUZZLE_INPUT: &str = include_str!("../../input/year2023/day13.txt");
const EXAMPLE_INPUT: &str = "\
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

#[test]
fn parse_example() {
    let parsed = parse(EXAMPLE_INPUT).expect("Error parsing input");

    assert_eq!(
        parsed,
        vec![
            Grid::from(grid![
                ['#', '.', '#', '#', '.', '.', '#', '#', '.']
                ['.', '.', '#', '.', '#', '#', '.', '#', '.']
                ['#', '#', '.', '.', '.', '.', '.', '.', '#']
                ['#', '#', '.', '.', '.', '.', '.', '.', '#']
                ['.', '.', '#', '.', '#', '#', '.', '#', '.']
                ['.', '.', '#', '#', '.', '.', '#', '#', '.']
                ['#', '.', '#', '.', '#', '#', '.', '#', '.']
            ]),
            Grid::from(grid![
                ['#', '.', '.', '.', '#', '#', '.', '.', '#']
                ['#', '.', '.', '.', '.', '#', '.', '.', '#']
                ['.', '.', '#', '#', '.', '.', '#', '#', '#']
                ['#', '#', '#', '#', '#', '.', '#', '#', '.']
                ['#', '#', '#', '#', '#', '.', '#', '#', '.']
                ['.', '.', '#', '#', '.', '.', '#', '#', '#']
                ['#', '.', '.', '.', '.', '#', '.', '.', '#']
            ]),
        ]
    );
}

#[test]
fn part1_example() {
    let input = parse(EXAMPLE_INPUT).expect("Error parsing input");
    let answer = part1(&input).expect("Error solving part 1");

    assert_eq!(answer, 405);
}

#[test]
fn part1_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let answer = part1(&parsed).expect("Error solving part 1");

    assert_eq!(answer, 35521);
}

#[test]
fn part2_example() {
    let input = parse(EXAMPLE_INPUT).expect("Error parsing example input");
    let answer = part2(&input).expect("Error solving part 2");

    assert_eq!(answer, 400);
}

#[test]
fn part2_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let answer = part2(&parsed).expect("Error solving part 2");

    assert_eq!(answer, 34795);
}
