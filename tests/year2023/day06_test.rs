use advent_of_code::year2023::day06::*;

const PUZZLE_INPUT: &str = include_str!("../../input/year2023/day06.txt");
const EXAMPLE_INPUT: &str = "\
Time:      7  15   30
Distance:  9  40  200";

#[test]
fn parse_test() {
    let parsed = parse(EXAMPLE_INPUT).expect("Error parsing example input");

    assert_eq!(
        parsed,
        Input {
            part1: vec![
                Race { time: 7, dist: 9 },
                Race { time: 15, dist: 40 },
                Race {
                    time: 30,
                    dist: 200
                },
            ],
            part2: Race {
                time: 71530,
                dist: 940200
            }
        }
    );
}

#[test]
fn part1_example() {
    let input = parse(EXAMPLE_INPUT).expect("Error parsing example input");
    let answer = part1(&input).expect("Error solving part 1");

    assert_eq!(answer, 288);
}

#[test]
fn part1_real() {
    let input = parse(PUZZLE_INPUT).expect("Error parsing example input");
    let answer = part1(&input).expect("Error solving part 1");

    assert_eq!(answer, 275724);
}

#[test]
fn part2_example() {
    let input = parse(EXAMPLE_INPUT).expect("Error parsing example input");
    let answer = part2(&input).expect("Error solving part 2");

    assert_eq!(answer, 71503);
}

#[test]
fn part2_real() {
    let input = parse(PUZZLE_INPUT).expect("Error parsing example input");
    let answer = part2(&input).expect("Error solving part 2");

    assert_eq!(answer, 37286485);
}
