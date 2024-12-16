use advent_of_code::year2024::day16::*;

const PUZZLE_INPUT: &str = include_str!("../../input/year2024/day16.txt");
const EXAMPLE_INPUT_1: &str = "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";

const EXAMPLE_INPUT_2: &str = "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
";

#[test]
fn parse_example_1() {
    let actual = parse(EXAMPLE_INPUT_1).expect("Error parsing input");

    assert_eq!(Some(&'#'), actual.get(0, 0));
    assert_eq!(Some(&'S'), actual.get(13, 1));
    assert_eq!(Some(&'E'), actual.get(1, 13));
}

#[test]
fn parse_example_2() {
    let actual = parse(EXAMPLE_INPUT_2).expect("Error parsing input");

    assert_eq!(Some(&'#'), actual.get(0, 0));
    assert_eq!(Some(&'S'), actual.get(15, 1));
    assert_eq!(Some(&'E'), actual.get(1, 15));
}

#[test]
fn part1_example_1() {
    let input = parse(EXAMPLE_INPUT_1).expect("Error parsing input");
    let actual = part1(&input).expect("Error solving part 1");

    assert_eq!(7036, actual);
}

#[test]
fn part1_example_2() {
    let input = parse(EXAMPLE_INPUT_2).expect("Error parsing input");
    let actual = part1(&input).expect("Error solving part 1");

    assert_eq!(11048, actual);
}

#[test]
fn part1_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    //let _actual = part1(&parsed).expect("Error solving part 1");

    // Runs in about 5 mins...

    //assert_eq!(134588, actual);
}

#[test]
fn part2_example_1() {
    let input = parse(EXAMPLE_INPUT_1).expect("Error parsing example input");
    let actual = part2(&input).expect("Error solving part 2");

    assert_eq!(45, actual);
}

#[test]
fn part2_example_2() {
    let input = parse(EXAMPLE_INPUT_2).expect("Error parsing example input");
    let actual = part2(&input).expect("Error solving part 2");

    assert_eq!(64, actual);
}

#[test]
fn part2_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    //let _actual = part2(&parsed).expect("Error solving part 2");

    // Runs in about 11 mins...

    //assert_eq!(631, actual);
}
