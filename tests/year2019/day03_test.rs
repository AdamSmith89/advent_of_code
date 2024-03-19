use advent_of_code::year2019::day03::*;

const PUZZLE_INPUT: &str = include_str!("../../input/year2019/day03.txt");
const EXAMPLE_INPUT: &str = "\
R8,U5,L5,D3
U7,R6,D4,L4
";

#[test]
fn parse_example() {
    let parsed = parse(EXAMPLE_INPUT).expect("Error parsing input");

    assert_eq!(
        parsed.0,
        vec![
            (1, 0),
            (2, 0),
            (3, 0),
            (4, 0),
            (5, 0),
            (6, 0),
            (7, 0),
            (8, 0),
            (8, 1),
            (8, 2),
            (8, 3),
            (8, 4),
            (8, 5),
            (7, 5),
            (6, 5),
            (5, 5),
            (4, 5),
            (3, 5),
            (3, 4),
            (3, 3),
            (3, 2)
        ],
        "First path parsed incorrectly"
    );

    assert_eq!(
        parsed.1,
        vec![
            (0, 1),
            (0, 2),
            (0, 3),
            (0, 4),
            (0, 5),
            (0, 6),
            (0, 7),
            (1, 7),
            (2, 7),
            (3, 7),
            (4, 7),
            (5, 7),
            (6, 7),
            (6, 6),
            (6, 5),
            (6, 4),
            (6, 3),
            (5, 3),
            (4, 3),
            (3, 3),
            (2, 3)
        ],
        "Second path parsed incorrectly"
    );

    assert_eq!(2, parsed.2.len(), "Incorrect number of intersections found");
    assert!(parsed.2.contains(&(3, 3)), "(3, 3) intersection not found");
    assert!(parsed.2.contains(&(6, 5)), "(6, 5) intersection not found");
}

#[test]
fn part1_example() {
    let input = parse(EXAMPLE_INPUT).expect("Error parsing input");
    let answer = part1(&input).expect("Error solving part 1");

    assert_eq!(answer, 6);
}

#[test]
fn part1_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let answer = part1(&parsed).expect("Error solving part 1");

    assert_eq!(answer, 731);
}

#[test]
fn part2_example() {
    let input = parse(EXAMPLE_INPUT).expect("Error parsing example input");
    let answer = part2(&input).expect("Error solving part 2");

    assert_eq!(answer, 30);
}

#[test]
fn part2_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let answer = part2(&parsed).expect("Error solving part 2");

    assert_eq!(answer, todo!());
}
