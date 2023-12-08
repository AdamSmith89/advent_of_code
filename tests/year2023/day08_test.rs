use std::collections::HashMap;

use advent_of_code::year2023::day08::*;

const PART1_EXAMPLE: &str = "\
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

#[test]
fn parse_test() {
    let parsed = parse(PART1_EXAMPLE).expect("Error parsing input");

    assert_eq!(parsed.steps, vec!['R', 'L'] );
    assert_hashmap(&parsed.network, "AAA", ("BBB", "CCC"));
    assert_hashmap(&parsed.network, "BBB", ("DDD", "EEE"));
    assert_hashmap(&parsed.network, "CCC", ("ZZZ", "GGG"));
    assert_hashmap(&parsed.network, "DDD", ("DDD", "DDD"));
    assert_hashmap(&parsed.network, "EEE", ("EEE", "EEE"));
    assert_hashmap(&parsed.network, "GGG", ("GGG", "GGG"));
    assert_hashmap(&parsed.network, "ZZZ", ("ZZZ", "ZZZ"));
}

fn assert_hashmap(map: &HashMap<String, (String, String)>, k: &str, v: (&str, &str)) {
    assert_eq!(map.get(&k.to_string()), Some(&(v.0.to_string(), v.1.to_string())));
}

#[test]
fn part1_example() {
    let input = parse(PART1_EXAMPLE).expect("Error parsing input");
    let answer = part1(&input).expect("Error solving part 1");

    assert_eq!(answer, 2);
}

#[test]
fn part1_real() {
    let input = include_str!("../../input/year2023/day08.txt");
    let parsed = parse(input).expect("Error parsing input");
    let answer = part1(&parsed).expect("Error solving part 1");

    assert_eq!(answer, 11911);
}

const PART2_EXAMPLE: &str = "\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

#[test]
fn part2_example() {
    let input = parse(PART2_EXAMPLE).expect("Error parsing example input");
    let answer = part2(&input).expect("Error solving part 2");

    assert_eq!(answer, 6);
}

#[test]
fn part2_real() {
    let input = include_str!("../../input/year2023/day08.txt");
    let parsed = parse(input).expect("Error parsing input");
    let answer = part2(&parsed).expect("Error solving part 2");

    assert_eq!(answer, todo!());
}
