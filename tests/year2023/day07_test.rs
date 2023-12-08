use advent_of_code::year2023::day07::*;
use itertools::Itertools;

const EXAMPLE: &str = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

#[test]
fn parse_test() {
    let parsed = parse(EXAMPLE).expect("Error parsing example input");

    assert_eq!(parsed, vec![
        Round { hand: "32T3K".chars().collect_vec(), bid: 765},
        Round { hand: "T55J5".chars().collect_vec(), bid: 684},
        Round { hand: "KK677".chars().collect_vec(), bid: 28},
        Round { hand: "KTJJT".chars().collect_vec(), bid: 220},
        Round { hand: "QQQJA".chars().collect_vec(), bid: 483},
    ]);
}

#[test]
fn part1_test() {
    let input = parse(EXAMPLE).expect("Error parsing example input");
    let answer = part1(&input).expect("Error solving part 1");

    assert_eq!(answer, 6440);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE).expect("Error parsing example input");
    let answer = part2(&input).expect("Error solving part 2");

    assert_eq!(answer, 5905);
}

const EDGE_CASES: &str = "\
2345A 1
Q2KJJ 13
Q2Q2Q 19
T3T3J 17
T3Q33 11
2345J 3
J345A 2
32T3K 5
T55J5 29
KK677 7
KTJJT 34
QQQJA 31
JJJJJ 37
JAAAA 43
AAAAJ 59
AAAAA 61
2AAAA 23
2JJJJ 53
JJJJ2 41";

#[test]
fn part1_edge_case() {
    let input = parse(EDGE_CASES).expect("Error parsing edge case input");
    let answer = part1(&input).expect("Error solving part 1");

    assert_eq!(answer, 6592);
}

#[test]
fn part2_edge_case() {
    let input = parse(EDGE_CASES).expect("Error parsing edge case input");
    let answer = part2(&input).expect("Error solving part 1");

    assert_eq!(answer, 6839);
}

const FULL_HOUSE_EDGE: &str = "\
2233J 10
22JJJ 6
AKQT9 5";

#[test]
fn part2_full_house_edge() {
    let input = parse(FULL_HOUSE_EDGE).expect("Error parsing input");
    let answer = part2(&input).expect("Error solving part 2");

    assert_eq!(answer, 43);
}
