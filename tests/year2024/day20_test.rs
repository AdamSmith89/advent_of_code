use advent_of_code::year2024::day20::*;

const PUZZLE_INPUT: &str = include_str!("../../input/year2024/day20.txt");
const EXAMPLE_INPUT: &str = "\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

#[test]
fn parse_example() {
    let actual = parse(EXAMPLE_INPUT).expect("Error parsing input");

    assert_eq!(Some(&'#'), actual.get(0, 0));
    assert_eq!(Some(&'S'), actual.get(3, 1));
    assert_eq!(Some(&'E'), actual.get(7, 5));
    assert_eq!(Some(&'#'), actual.get(14, 14));
}

// No test example for 100 steps saved
// #[test]
// fn part1_example() {
//     let input = parse(EXAMPLE_INPUT).expect("Error parsing input");
//     let actual = part1(&input).expect("Error solving part 1");

//     assert_eq!(0, actual);
// }

#[test]
fn part1_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let actual = part1(&parsed).expect("Error solving part 1");

    assert_eq!(1346, actual);
}

// No test example for 100 steps saved
// #[test]
// fn part2_example() {
//     let input = parse(EXAMPLE_INPUT).expect("Error parsing example input");
//     let actual = part2(&input).expect("Error solving part 2");

//     assert_eq!(0, actual);
// }

#[test]
fn part2_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let actual = part2(&parsed).expect("Error solving part 2");

    assert_eq!(985482, actual);
}
