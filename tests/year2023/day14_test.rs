use advent_of_code::util::grid::Grid;
use advent_of_code::year2023::day14::*;
use grid::grid;

const PUZZLE_INPUT: &str = include_str!("../../input/year2023/day14.txt");
const EXAMPLE_INPUT: &str = "\
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

#[test]
fn parse_example() {
    let parsed = parse(EXAMPLE_INPUT).expect("Error parsing input");

    assert_eq!(
        parsed,
        Grid::from(grid![
            ['O', '.', '.', '.', '.', '#', '.', '.', '.', '.']
            ['O', '.', 'O', 'O', '#', '.', '.', '.', '.', '#']
            ['.', '.', '.', '.', '.', '#', '#', '.', '.', '.']
            ['O', 'O', '.', '#', 'O', '.', '.', '.', '.', 'O']
            ['.', 'O', '.', '.', '.', '.', '.', 'O', '#', '.']
            ['O', '.', '#', '.', '.', 'O', '.', '#', '.', '#']
            ['.', '.', 'O', '.', '.', '#', 'O', '.', '.', 'O']
            ['.', '.', '.', '.', '.', '.', '.', 'O', '.', '.']
            ['#', '.', '.', '.', '.', '#', '#', '#', '.', '.']
            ['#', 'O', 'O', '.', '.', '#', '.', '.', '.', '.']
        ])
    );
}

#[test]
fn part1_example() {
    let input = parse(EXAMPLE_INPUT).expect("Error parsing input");
    let answer = part1(&input).expect("Error solving part 1");

    assert_eq!(answer, 136);
}

#[test]
fn part1_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let answer = part1(&parsed).expect("Error solving part 1");

    assert_eq!(answer, 105249);
}

#[test]
fn part2_example() {
    let input = parse(EXAMPLE_INPUT).expect("Error parsing example input");
    let answer = part2(&input).expect("Error solving part 2");

    assert_eq!(answer, 64);
}

#[test]
fn part2_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let answer = part2(&parsed).expect("Error solving part 2");

    assert_eq!(answer, 88680);
}
