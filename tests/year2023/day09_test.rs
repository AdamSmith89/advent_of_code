use advent_of_code::year2023::day09::*;

const PUZZLE_INPUT: &str = include_str!("../../input/year2023/day09.txt");
const EXAMPLE_INPUT: &str = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

#[test]
fn parse_example() {
    let parsed = parse(EXAMPLE_INPUT).expect("Error parsing input");

    assert_eq!(
        parsed,
        vec![
            Reading {
                history: vec![0, 3, 6, 9, 12, 15]
            },
            Reading {
                history: vec![1, 3, 6, 10, 15, 21]
            },
            Reading {
                history: vec![10, 13, 16, 21, 30, 45]
            },
        ]
    );
}

#[test]
fn parse_neg() {
    let parsed = parse("0 -1 4 -3 10 -35").expect("Error parsing input");

    assert_eq!(
        parsed,
        vec![Reading {
            history: vec![0, -1, 4, -3, 10, -35],
        }]
    );
}

#[test]
fn part1_example() {
    let input = parse(EXAMPLE_INPUT).expect("Error parsing input");
    let answer = part1(&input).expect("Error solving part 1");

    assert_eq!(answer, 114);
}

#[test]
fn part1_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let answer = part1(&parsed).expect("Error solving part 1");

    assert_eq!(answer, 1972648895);
}

#[test]
fn part2_example() {
    let input = parse(EXAMPLE_INPUT).expect("Error parsing example input");
    let answer = part2(&input).expect("Error solving part 2");

    assert_eq!(answer, 2);
}

#[test]
fn part2_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let answer = part2(&parsed).expect("Error solving part 2");

    assert_eq!(answer, 919);
}

#[test]
fn find_next_pos() {
    let reading = Reading {
        history: vec![0, 3, 6, 9, 12, 15],
    };
    let resolved = next_in_sequence(&reading).expect("Error resolving sequence");

    assert_eq!(resolved, 18);
}

#[test]
fn find_next_neg() {
    let reading = Reading {
        history: vec![0, 3, -2, 4, 1, 6],
    };
    let resolved = next_in_sequence(&reading).expect("Error resolving sequence");

    assert_eq!(resolved, 149);
}
