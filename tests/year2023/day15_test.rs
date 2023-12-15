use advent_of_code::year2023::day15::*;

const PUZZLE_INPUT: &str = include_str!("../../input/year2023/day15.txt");
const EXAMPLE_INPUT: &str = "\
rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

#[test]
fn parse_example() {
    let parsed = parse(EXAMPLE_INPUT).expect("Error parsing input");

    assert_eq!(
        parsed,
        vec![
            "rn=1".to_string(),
            "cm-".to_string(),
            "qp=3".to_string(),
            "cm=2".to_string(),
            "qp-".to_string(),
            "pc=4".to_string(),
            "ot=9".to_string(),
            "ab=5".to_string(),
            "pc-".to_string(),
            "pc=6".to_string(),
            "ot=7".to_string()
        ]
    );
}

#[test]
fn part1_example() {
    let input = parse(EXAMPLE_INPUT).expect("Error parsing input");
    let answer = part1(&input).expect("Error solving part 1");

    assert_eq!(answer, 1320);
}

#[test]
fn part1_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let answer = part1(&parsed).expect("Error solving part 1");

    assert_eq!(answer, todo!());
}

#[test]
fn part2_example() {
    let input = parse(EXAMPLE_INPUT).expect("Error parsing example input");
    let answer = part2(&input).expect("Error solving part 2");

    assert_eq!(answer, todo!());
}

#[test]
fn part2_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let answer = part2(&parsed).expect("Error solving part 2");

    assert_eq!(answer, todo!());
}
