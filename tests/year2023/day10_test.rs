use advent_of_code::{util::grid::Grid, year2023::day10::*};
use grid::grid;

const PUZZLE_INPUT: &str = include_str!("../../input/year2023/day10.txt");
const EXAMPLE_INPUT_PART1_1: &str = "\
-L|F7
7S-7|
L|7||
-L-J|
L|-JF";

const EXAMPLE_INPUT_PART1_2: &str = "\
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

const EXAMPLE_INPUT_PART2_1: &str = "\
..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";

const EXAMPLE_INPUT_PART2_2: &str = "\
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

#[test]
fn parse_example() {
    let parsed = parse(EXAMPLE_INPUT_PART1_1).expect("Error parsing input");

    assert_eq!(
        parsed,
        Grid::from(grid![
            [NodeType::Horizontal,    NodeType::BendNorthEast, NodeType::Vertical,      NodeType::BendSouthEast, NodeType::BendSouthWest]
            [NodeType::BendSouthWest, NodeType::Start,         NodeType::Horizontal,    NodeType::BendSouthWest, NodeType::Vertical]
            [NodeType::BendNorthEast, NodeType::Vertical,      NodeType::BendSouthWest, NodeType::Vertical,      NodeType::Vertical]
            [NodeType::Horizontal,    NodeType::BendNorthEast, NodeType::Horizontal,    NodeType::BendNorthWest, NodeType::Vertical]
            [NodeType::BendNorthEast, NodeType::Vertical,      NodeType::Horizontal,    NodeType::BendNorthWest, NodeType::BendSouthEast]
        ])
    );
}

#[test]
fn part1_example_1() {
    let input = parse(EXAMPLE_INPUT_PART1_1).expect("Error parsing input");
    let answer = part1(&input).expect("Error solving part 1");

    assert_eq!(answer, 4);
}

#[test]
fn part1_example_2() {
    let input = parse(EXAMPLE_INPUT_PART1_2).expect("Error parsing input");
    let answer = part1(&input).expect("Error solving part 1");

    assert_eq!(answer, 8);
}

#[test]
fn part1_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let answer = part1(&parsed).expect("Error solving part 1");

    assert_eq!(answer, 7173);
}

#[test]
fn part2_example_1() {
    let input = parse(EXAMPLE_INPUT_PART2_1).expect("Error parsing example input");
    let answer = part2(&input).expect("Error solving part 2");

    assert_eq!(answer, 4);
}

#[test]
fn part2_example_2() {
    let input = parse(EXAMPLE_INPUT_PART2_2).expect("Error parsing example input");
    let answer = part2(&input).expect("Error solving part 2");

    assert_eq!(answer, 10);
}

#[test]
fn part2_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let answer = part2(&parsed).expect("Error solving part 2");

    assert_eq!(answer, 291);
}
