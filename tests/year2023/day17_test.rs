use advent_of_code::{util::grid::Grid, year2023::day17::*};
use grid::grid;

const PUZZLE_INPUT: &str = include_str!("../../input/year2023/day17.txt");
const EXAMPLE_INPUT: &str = "\
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

#[test]
fn parse_example() {
    let parsed = parse(EXAMPLE_INPUT).expect("Error parsing input");

    assert_eq!(
        parsed,
        Grid::from(grid![
            [2, 4, 1, 3, 4, 3, 2, 3, 1, 1, 3, 2, 3]
            [3, 2, 1, 5, 4, 5, 3, 5, 3, 5, 6, 2, 3]
            [3, 2, 5, 5, 2, 4, 5, 6, 5, 4, 2, 5, 4]
            [3, 4, 4, 6, 5, 8, 5, 8, 4, 5, 4, 5, 2]
            [4, 5, 4, 6, 6, 5, 7, 8, 6, 7, 5, 3, 6]
            [1, 4, 3, 8, 5, 9, 8, 7, 9, 8, 4, 5, 4]
            [4, 4, 5, 7, 8, 7, 6, 9, 8, 7, 7, 6, 6]
            [3, 6, 3, 7, 8, 7, 7, 9, 7, 9, 6, 5, 3]
            [4, 6, 5, 4, 9, 6, 7, 9, 8, 6, 8, 8, 7]
            [4, 5, 6, 4, 6, 7, 9, 9, 8, 6, 4, 5, 3]
            [1, 2, 2, 4, 6, 8, 6, 8, 6, 5, 5, 6, 3]
            [2, 5, 4, 6, 5, 4, 8, 8, 8, 7, 7, 3, 5]
            [4, 3, 2, 2, 6, 7, 4, 6, 5, 5, 5, 3, 3]
        ])
    );
}

#[test]
fn part1_example() {
    let input = parse(EXAMPLE_INPUT).expect("Error parsing input");
    let answer = part1(&input).expect("Error solving part 1");

    assert_eq!(answer, 102);
}

#[test]
fn part1_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let answer = part1(&parsed).expect("Error solving part 1");

    assert_eq!(answer, 959);
}

#[test]
fn part2_example() {
    let input = parse(EXAMPLE_INPUT).expect("Error parsing example input");
    let answer = part2(&input).expect("Error solving part 2");

    assert_eq!(answer, 94);
}

#[test]
fn part2_example2() {
    let input = "\
111111111111
999999999991
999999999991
999999999991
999999999991";

    let input = parse(input).expect("Error parsing example input");
    let answer = part2(&input).expect("Error solving part 2");

    assert_eq!(answer, 71);
}

#[test]
fn part2_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let answer = part2(&parsed).expect("Error solving part 2");

    assert_eq!(answer, 1135);
}
