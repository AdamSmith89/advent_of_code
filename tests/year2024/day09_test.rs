use advent_of_code::year2024::day09::*;

const PUZZLE_INPUT: &str = include_str!("../../input/year2024/day09.txt");
const EXAMPLE_INPUT: &str = "2333133121414131402";

#[test]
fn parse_example() {
    let actual = parse(EXAMPLE_INPUT).expect("Error parsing input");

    let expected = vec![
        File::new(Some(0), 2),
        File::new(None, 3),
        File::new(Some(1), 3),
        File::new(None, 3),
        File::new(Some(2), 1),
        File::new(None, 3),
        File::new(Some(3), 3),
        File::new(None, 1),
        File::new(Some(4), 2),
        File::new(None, 1),
        File::new(Some(5), 4),
        File::new(None, 1),
        File::new(Some(6), 4),
        File::new(None, 1),
        File::new(Some(7), 3),
        File::new(None, 1),
        File::new(Some(8), 4),
        File::new(None, 0),
        File::new(Some(9), 2),
    ];

    assert_eq!(expected, actual);
}

#[test]
fn part1_example() {
    let input = parse(EXAMPLE_INPUT).expect("Error parsing input");
    let actual = part1(&input).expect("Error solving part 1");

    assert_eq!(1928, actual);
}

#[test]
fn part1_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let actual = part1(&parsed).expect("Error solving part 1");

    assert_eq!(6399153661894, actual);
}

#[test]
fn part2_example() {
    let input = parse(EXAMPLE_INPUT).expect("Error parsing example input");
    let actual = part2(&input).expect("Error solving part 2");

    assert_eq!(2858, actual);
}

#[test]
fn part2_example2() {
    let input = parse("12345").expect("Error parsing example input");
    let actual = part2(&input).expect("Error solving part 2");

    assert_eq!(132, actual);
}

#[test]
fn part2_example3() {
    let input = parse("14113").expect("Error parsing example input");
    let actual = part2(&input).expect("Error solving part 2");

    assert_eq!(16, actual);
}

#[test]
fn part2_example4() {
    let input = parse("133").expect("Error parsing example input");
    let actual = part2(&input).expect("Error solving part 2");

    assert_eq!(6, actual);
}

#[test]
fn part2_example5() {
    let input = parse("1010101010101010101010").expect("Error parsing example input");
    let actual = part2(&input).expect("Error solving part 2");

    assert_eq!(385, actual);
}

#[test]
fn part2_example6() {
    let input = parse("354631466260").expect("Error parsing example input");
    let actual = part2(&input).expect("Error solving part 2");

    assert_eq!(1325, actual);
}

#[test]
fn part2_example7() {
    let input = parse("252").expect("Error parsing example input");
    let actual = part2(&input).expect("Error solving part 2");

    assert_eq!(5, actual);
}

#[test]
fn part2_example8() {
    let input = parse("171010402").expect("Error parsing example input");
    let actual = part2(&input).expect("Error solving part 2");

    assert_eq!(88, actual);
}

#[test]
fn narrow_down() {
    let input = parse("156769588015262926819037393").expect("Error parsing example input");
    let actual = part2(&input).expect("Error solving part 2");

    assert_eq!(10864, actual);
}

#[test]
fn part2_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let actual = part2(&parsed).expect("Error solving part 2");

    assert_eq!(6421724645083, actual);
}
