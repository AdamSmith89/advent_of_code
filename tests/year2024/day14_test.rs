use advent_of_code::year2024::day14::*;

const PUZZLE_INPUT: &str = include_str!("../../input/year2024/day14.txt");
// The "example" line is to cater for this comment in the puzzle:
// "However, in this example, the robots are in a space which is only 11 tiles wide and 7 tiles tall."
const EXAMPLE_INPUT: &str = "\
example
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

#[test]
fn parse_example() {
    let actual = parse(EXAMPLE_INPUT).expect("Error parsing input");
    let expected = (
        vec![
            Guard::new((0, 4).into(), (3, -3).into()),
            Guard::new((6, 3).into(), (-1, -3).into()),
            Guard::new((10, 3).into(), (-1, 2).into()),
            Guard::new((2, 0).into(), (2, -1).into()),
            Guard::new((0, 0).into(), (1, 3).into()),
            Guard::new((3, 0).into(), (-2, -2).into()),
            Guard::new((7, 6).into(), (-1, -3).into()),
            Guard::new((3, 0).into(), (-1, -2).into()),
            Guard::new((9, 3).into(), (2, 3).into()),
            Guard::new((7, 3).into(), (-1, 2).into()),
            Guard::new((2, 4).into(), (2, -3).into()),
            Guard::new((9, 5).into(), (-3, -3).into()),
        ],
        11,
        7,
    );

    assert_eq!(expected, actual);
}

#[test]
fn part1_example() {
    let input = parse(EXAMPLE_INPUT).expect("Error parsing input");
    let actual = part1(&input).expect("Error solving part 1");

    assert_eq!(12, actual);
}

#[test]
fn part1_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let actual = part1(&parsed).expect("Error solving part 1");

    assert_eq!(229868730, actual);
}

// No example provided
// #[test]
// fn part2_example() {
//     let input = parse(EXAMPLE_INPUT).expect("Error parsing example input");
//     let _actual = part2(&input).expect("Error solving part 2");

//     //assert_eq!(expected, actual);
// }

#[test]
fn part2_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let actual = part2(&parsed).expect("Error solving part 2");

    assert_eq!(7861, actual);
}
