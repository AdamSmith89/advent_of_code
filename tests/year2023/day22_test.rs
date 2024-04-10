use advent_of_code::year2023::day22::*;

const PUZZLE_INPUT: &str = include_str!("../../input/year2023/day22.txt");
const EXAMPLE_INPUT: &str = "\
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

#[test]
fn parse_example() {
    let _parsed = parse(EXAMPLE_INPUT).expect("Error parsing input");

    // assert_eq!(
    //     parsed,
    //     vec![
    //         Brick {
    //             id: 1,
    //             x: 1..1 + 1,
    //             y: 0..2 + 1,
    //             z: 1..1 + 1,
    //             above: vec![],
    //             below: vec![],
    //         },
    //         Brick {
    //             id: 2,
    //             x: 0..2 + 1,
    //             y: 0..0 + 1,
    //             z: 2..2 + 1,
    //             above: vec![],
    //             below: vec![],
    //         },
    //         Brick {
    //             id: 3,
    //             x: 0..2 + 1,
    //             y: 2..2 + 1,
    //             z: 3..3 + 1,
    //             above: vec![],
    //             below: vec![],
    //         },
    //         Brick {
    //             id: 4,
    //             x: 0..0 + 1,
    //             y: 0..2 + 1,
    //             z: 4..4 + 1,
    //             above: vec![],
    //             below: vec![],
    //         },
    //         Brick {
    //             id: 5,
    //             x: 2..2 + 1,
    //             y: 0..2 + 1,
    //             z: 5..5 + 1,
    //             above: vec![],
    //             below: vec![],
    //         },
    //         Brick {
    //             id: 6,
    //             x: 0..2 + 1,
    //             y: 1..1 + 1,
    //             z: 6..6 + 1,
    //             above: vec![],
    //             below: vec![],
    //         },
    //         Brick {
    //             id: 7,
    //             x: 1..1 + 1,
    //             y: 1..1 + 1,
    //             z: 8..9 + 1,
    //             above: vec![],
    //             below: vec![],
    //         },
    //     ]
    // );
}

#[test]
fn part1_example() {
    let input = parse(EXAMPLE_INPUT).expect("Error parsing input");
    let answer = part1(&input).expect("Error solving part 1");

    assert_eq!(answer, 5);
}

#[test]
fn part1_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let answer = part1(&parsed).expect("Error solving part 1");

    assert_eq!(answer, 468);
}

#[test]
fn part2_example() {
    let input = parse(EXAMPLE_INPUT).expect("Error parsing example input");
    let answer = part2(&input).expect("Error solving part 2");

    assert_eq!(answer, 7);
}

#[test]
fn part2_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let answer = part2(&parsed).expect("Error solving part 2");

    assert_eq!(answer, 75358);
}
