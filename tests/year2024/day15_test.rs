use advent_of_code::{util::direction::Direction, year2024::day15::*};

const PUZZLE_INPUT: &str = include_str!("../../input/year2024/day15.txt");
const EXAMPLE_INPUT_1: &str = "\
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

const EXAMPLE_INPUT_2: &str = "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

const EDGE_CASE: &str = "\
########
#......#
#.OO...#
#..OOO@#
#.O.O..#
#..OO..#
#......#
########

<vvv<<<<^^^";

// const EDGE_CASE: &str = "\
// ################
// ##............##
// ##..[][]......##
// ##....[][][]@.##
// ##..[]..[]....##
// ##....[][]....##
// ##............##
// ################

// <vvv<<<<^^^";

/*
  0000000000111111
  0123456789023456
00################
01##..[][]......##
02##...[][].....##
03##....[].[]...##
04##..[].@[]....##
05##......[]....##
06##............##
07################

1,4
1,6
2,5
2,7
3,6
3,9
4,4
4,8
5,8
*/

#[test]
fn parse_example_1() {
    use Direction::*;

    let (map, steps) = parse(EXAMPLE_INPUT_1).expect("Error parsing example 1 input");
    assert_eq!(Some(&'#'), map.get(0, 0));
    assert_eq!(Some(&'O'), map.get(1, 3));
    assert_eq!(Some(&'@'), map.get(2, 2));
    assert_eq!(Some(&'#'), map.get(7, 7));
    assert_eq!(
        vec![
            West, North, North, East, East, East, South, South, West, South, East, East, South,
            West, West
        ],
        steps
    );
}

#[test]
fn parse_example_2() {
    use Direction::*;

    let (map, steps) = parse(EXAMPLE_INPUT_2).expect("Error parsing example 2 input");
    assert_eq!(Some(&'#'), map.get(0, 0));
    assert_eq!(Some(&'O'), map.get(1, 3));
    assert_eq!(Some(&'@'), map.get(4, 4));
    assert_eq!(Some(&'#'), map.get(9, 9));
    
    // 73 is enough to get onto the 2nd line
    assert_eq!(
        vec![
            West, South, South, East, North, West, South, North, East, South, East, North, South,
            South, North, South, East, South, West, East, South, North, South, West, South, West,
            North, South, South, West, West, West, North, East, West, West, East, West, East, East,
            South, West, South, South, South, West, East, North, South, North, East, North, West,
            West, West, East, West, West, South, West, West, West, South, North, South, South,
            North, South, East, North, South, South, South
        ],
        steps[..73]
    );
}

#[test]
fn part1_example_1() {
    let input = parse(EXAMPLE_INPUT_1).expect("Error parsing input");
    let actual = part1(&input).expect("Error solving part 1");

    assert_eq!(2028, actual);
}

#[test]
fn part1_example_2() {
    let input = parse(EXAMPLE_INPUT_2).expect("Error parsing input");
    let actual = part1(&input).expect("Error solving part 1");

    assert_eq!(10092, actual);
}

#[test]
fn part1_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let actual = part1(&parsed).expect("Error solving part 1");

    assert_eq!(1526673, actual);
}

#[test]
fn part2_example() {
    let input = parse(EXAMPLE_INPUT_2).expect("Error parsing example input");
    let actual = part2(&input).expect("Error solving part 2");

    assert_eq!(9021, actual);
}

#[test]
fn part2_edge_case() {
    let input = parse(EDGE_CASE).expect("Error parsing example input");
    let actual = part2(&input).expect("Error solving part 2");

    assert_eq!(2557, actual);
}

#[test]
fn part2_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let actual = part2(&parsed).expect("Error solving part 2");

    assert_eq!(1535509, actual);
}
