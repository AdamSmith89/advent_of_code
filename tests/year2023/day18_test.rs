use advent_of_code::util::grid::Direction;
use advent_of_code::year2023::day18::*;

const PUZZLE_INPUT: &str = include_str!("../../input/year2023/day18.txt");
const EXAMPLE_INPUT: &str = "\
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

#[test]
fn parse_example() {
    let parsed = parse(EXAMPLE_INPUT).expect("Error parsing input");

    assert_eq!(
        parsed,
        Input {
            part1: vec![
                DigStep::new(Direction::East, 6),
                DigStep::new(Direction::South, 5),
                DigStep::new(Direction::West, 2),
                DigStep::new(Direction::South, 2),
                DigStep::new(Direction::East, 2),
                DigStep::new(Direction::South, 2),
                DigStep::new(Direction::West, 5),
                DigStep::new(Direction::North, 2),
                DigStep::new(Direction::West, 1),
                DigStep::new(Direction::North, 2),
                DigStep::new(Direction::East, 2),
                DigStep::new(Direction::North, 3),
                DigStep::new(Direction::West, 2),
                DigStep::new(Direction::North, 2),
            ],
            part2: vec![
                DigStep::new(Direction::East, 461937),
                DigStep::new(Direction::South, 56407),
                DigStep::new(Direction::East, 356671),
                DigStep::new(Direction::South, 863240),
                DigStep::new(Direction::East, 367720),
                DigStep::new(Direction::South, 266681),
                DigStep::new(Direction::West, 577262),
                DigStep::new(Direction::North, 829975),
                DigStep::new(Direction::West, 112010),
                DigStep::new(Direction::South, 829975),
                DigStep::new(Direction::West, 491645),
                DigStep::new(Direction::North, 686074),
                DigStep::new(Direction::West, 5411),
                DigStep::new(Direction::North, 500254),
            ],
        }
    );
}

#[test]
fn part1_example() {
    let input = parse(EXAMPLE_INPUT).expect("Error parsing input");
    let answer = part1(&input).expect("Error solving part 1");

    assert_eq!(answer, 62);
}

#[test]
fn part1_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let answer = part1(&parsed).expect("Error solving part 1");

    assert_eq!(answer, 38188);
}

#[test]
fn part2_example() {
    let input = parse(EXAMPLE_INPUT).expect("Error parsing example input");
    let answer = part2(&input).expect("Error solving part 2");

    assert_eq!(answer, 952408144115);
}

#[test]
fn part2_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let answer = part2(&parsed).expect("Error solving part 2");

    assert_eq!(answer, 93325849869340);
}
