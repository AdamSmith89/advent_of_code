use advent_of_code::{util::grid::Grid, year2023::day23::*};
use grid::grid;

const PUZZLE_INPUT: &str = include_str!("../../input/year2023/day23.txt");
const EXAMPLE_INPUT: &str = "\
#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

#[test]
fn parse_example() {
    let parsed = parse(EXAMPLE_INPUT).expect("Error parsing input");

    assert_eq!(
        parsed,
        Grid::from(grid![
            [Tile::Forest, Tile::Path,   Tile::Forest, Tile::Forest,     Tile::Forest,    Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest]
            [Tile::Forest, Tile::Path,   Tile::Path,   Tile::Path,       Tile::Path,      Tile::Path, Tile::Path, Tile::Path, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Path, Tile::Path, Tile::Path, Tile::Forest, Tile::Forest, Tile::Forest]
            [Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest,     Tile::Forest,    Tile::Forest, Tile::Forest, Tile::Path, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Path, Tile::Forest, Tile::Path, Tile::Forest, Tile::Forest, Tile::Forest]
            [Tile::Forest, Tile::Forest, Tile::Forest, Tile::Path,       Tile::Path,      Tile::Path, Tile::Path, Tile::Path, Tile::Forest, Tile::Path, Tile::SlopeEast, Tile::Path, Tile::SlopeEast, Tile::Path, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Path, Tile::Forest, Tile::Path, Tile::Forest, Tile::Forest, Tile::Forest]
            [Tile::Forest, Tile::Forest, Tile::Forest, Tile::SlopeSouth, Tile::Forest,    Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Path, Tile::Forest, Tile::SlopeSouth, Tile::Forest, Tile::Path, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Path, Tile::Forest, Tile::Path, Tile::Forest, Tile::Forest, Tile::Forest]
            [Tile::Forest, Tile::Forest, Tile::Forest, Tile::Path,       Tile::SlopeEast, Tile::Path, Tile::Path, Tile::Path, Tile::Forest, Tile::Path, Tile::Forest, Tile::Path, Tile::Forest, Tile::Path, Tile::Path, Tile::Path, Tile::Path, Tile::Path, Tile::Forest, Tile::Path, Tile::Path, Tile::Path, Tile::Forest]
            [Tile::Forest, Tile::Forest, Tile::Forest, Tile::SlopeSouth, Tile::Forest,    Tile::Forest, Tile::Forest, Tile::Path, Tile::Forest, Tile::Path, Tile::Forest, Tile::Path, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Path, Tile::Forest]
            [Tile::Forest, Tile::Forest, Tile::Forest, Tile::Path,       Tile::Path,      Tile::Path, Tile::Forest, Tile::Path, Tile::Forest, Tile::Path, Tile::Forest, Tile::Path, Tile::Path, Tile::Path, Tile::Path, Tile::Path, Tile::Path, Tile::Path, Tile::Forest, Tile::Path, Tile::Path, Tile::Path, Tile::Forest]
            [Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest,     Tile::Forest,    Tile::Path, Tile::Forest, Tile::Path, Tile::Forest, Tile::Path, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Path, Tile::Forest, Tile::Path, Tile::Forest, Tile::Forest, Tile::Forest]
            [Tile::Forest, Tile::Path,   Tile::Path,   Tile::Path,       Tile::Path,      Tile::Path, Tile::Forest, Tile::Path, Tile::Forest, Tile::Path, Tile::Forest, Tile::Path, Tile::Path, Tile::Path, Tile::Path, Tile::Path, Tile::Path, Tile::Path, Tile::Forest, Tile::Path, Tile::Path, Tile::Path, Tile::Forest]
            [Tile::Forest, Tile::Path,   Tile::Forest, Tile::Forest,     Tile::Forest,    Tile::Forest, Tile::Forest, Tile::Path, Tile::Forest, Tile::Path, Tile::Forest, Tile::Path, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest, Tile::SlopeSouth, Tile::Forest]
            [Tile::Forest, Tile::Path,   Tile::Forest, Tile::Path,       Tile::Path,      Tile::Path, Tile::Forest, Tile::Path, Tile::Path, Tile::Path, Tile::Forest, Tile::Path, Tile::Path, Tile::Path, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Path, Tile::Path, Tile::Path, Tile::SlopeEast, Tile::Path, Tile::Forest]
            [Tile::Forest, Tile::Path,   Tile::Forest, Tile::Path,       Tile::Forest,    Tile::SlopeSouth, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest, Tile::SlopeSouth, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Path, Tile::Forest, Tile::Forest, Tile::Forest, Tile::SlopeSouth, Tile::Forest]
            [Tile::Forest, Tile::Path,   Tile::Path,   Tile::Path,       Tile::Forest,    Tile::Path, Tile::SlopeEast, Tile::Path, Tile::Forest, Tile::Path, Tile::Path, Tile::Path, Tile::SlopeEast, Tile::Path, Tile::SlopeEast, Tile::Path, Tile::Forest, Tile::Path, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Path, Tile::Forest]
            [Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest,     Tile::Forest,    Tile::SlopeSouth, Tile::Forest, Tile::Path, Tile::Forest, Tile::Path, Tile::Forest, Tile::Forest, Tile::Forest, Tile::SlopeSouth, Tile::Forest, Tile::Path, Tile::Forest, Tile::Path, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Path, Tile::Forest]
            [Tile::Forest, Tile::Path,   Tile::Path,   Tile::Path,       Tile::Path,      Tile::Path,      Tile::Forest, Tile::Path, Tile::Path, Tile::Path, Tile::Forest, Tile::Path, Tile::Path, Tile::Path, Tile::Forest, Tile::Path, Tile::Forest, Tile::Path, Tile::Forest, Tile::Path, Tile::Path, Tile::Path, Tile::Forest]
            [Tile::Forest, Tile::Path,   Tile::Forest, Tile::Forest,     Tile::Forest,    Tile::Forest,    Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Path, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Path, Tile::Forest, Tile::Path, Tile::Forest, Tile::Path, Tile::Forest, Tile::Forest, Tile::Forest]
            [Tile::Forest, Tile::Path,   Tile::Path,   Tile::Path,       Tile::Forest,    Tile::Forest,    Tile::Forest, Tile::Path, Tile::Path, Tile::Path, Tile::Forest, Tile::Path, Tile::Path, Tile::Path, Tile::Forest, Tile::Path, Tile::Path, Tile::Path, Tile::Forest, Tile::Path, Tile::Forest, Tile::Forest, Tile::Forest]
            [Tile::Forest, Tile::Forest, Tile::Forest, Tile::Path,       Tile::Forest,    Tile::Forest,    Tile::Forest, Tile::Path, Tile::Forest, Tile::Path, Tile::Forest, Tile::Forest, Tile::Forest, Tile::SlopeSouth, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest, Tile::SlopeSouth, Tile::Forest, Tile::Forest, Tile::Forest]
            [Tile::Forest, Tile::Path,   Tile::Path,   Tile::Path,       Tile::Forest,    Tile::Path,      Tile::Path, Tile::Path, Tile::Forest, Tile::Path, Tile::Forest, Tile::Path, Tile::SlopeEast, Tile::Path, Tile::SlopeEast, Tile::Path, Tile::Forest, Tile::Path, Tile::SlopeEast, Tile::Path, Tile::Forest, Tile::Forest, Tile::Forest]
            [Tile::Forest, Tile::Path,   Tile::Forest, Tile::Forest,     Tile::Forest,    Tile::Path,      Tile::Forest, Tile::Forest, Tile::Forest, Tile::Path, Tile::Forest, Tile::Path, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Path, Tile::Forest, Tile::Path, Tile::Forest, Tile::SlopeSouth, Tile::Forest, Tile::Forest, Tile::Forest]
            [Tile::Forest, Tile::Path,   Tile::Path,   Tile::Path,       Tile::Path,      Tile::Path,      Tile::Forest, Tile::Forest, Tile::Forest, Tile::Path, Tile::Path, Tile::Path, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Path, Tile::Path, Tile::Path, Tile::Forest, Tile::Path, Tile::Path, Tile::Path, Tile::Forest]
            [Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest,     Tile::Forest,    Tile::Forest,    Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Forest, Tile::Path, Tile::Forest]
        ])
    );
}

#[test]
fn part1_example() {
    let input = parse(EXAMPLE_INPUT).expect("Error parsing input");
    let answer = part1(&input).expect("Error solving part 1");

    assert_eq!(94, answer);
}

#[test]
fn part1_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let answer = part1(&parsed).expect("Error solving part 1");

    assert_eq!(2134, answer);
}

#[test]
fn part2_example() {
    let input = parse(EXAMPLE_INPUT).expect("Error parsing example input");
    let _answer = part2(&input).expect("Error solving part 2");

    //assert_eq!(154, answer);
    todo!("assert_eq!(expected, actual);")
}

#[test]
fn part2_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let _answer = part2(&parsed).expect("Error solving part 2");

    todo!("assert_eq!(expected, actual);")
}
