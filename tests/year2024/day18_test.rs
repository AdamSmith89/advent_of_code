use advent_of_code::{util::point::Point, year2024::day18::*};

const PUZZLE_INPUT: &str = include_str!("../../input/year2024/day18.txt");
const EXAMPLE_INPUT: &str = "\
example
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

#[test]
fn parse_example() {
    let (grid, byte_locs, num_bytes) = parse(EXAMPLE_INPUT).expect("Error parsing input");

    assert_eq!(7, grid.rows());
    assert_eq!(7, grid.cols());
    assert_eq!(12, num_bytes);

    assert_eq!(
        vec![
            Point::from((5, 4)),
            Point::from((4, 2)),
            Point::from((4, 5)),
            Point::from((3, 0)),
            Point::from((2, 1)),
            Point::from((6, 3)),
            Point::from((2, 4)),
            Point::from((1, 5)),
            Point::from((0, 6)),
            Point::from((3, 3)),
            Point::from((2, 6)),
            Point::from((5, 1)),
            Point::from((1, 2)),
            Point::from((5, 5)),
            Point::from((2, 5)),
            Point::from((6, 5)),
            Point::from((1, 4)),
            Point::from((0, 4)),
            Point::from((6, 4)),
            Point::from((1, 1)),
            Point::from((6, 1)),
            Point::from((1, 0)),
            Point::from((0, 5)),
            Point::from((1, 6)),
            Point::from((2, 0)),
        ],
        byte_locs
    );
}

#[test]
fn part1_example() {
    let input = parse(EXAMPLE_INPUT).expect("Error parsing input");
    let actual = part1(&input).expect("Error solving part 1");

    assert_eq!(22, actual);
}

#[test]
fn part1_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let actual = part1(&parsed).expect("Error solving part 1");

    assert_eq!(316, actual);
}

#[test]
fn part2_example() {
    let input = parse(EXAMPLE_INPUT).expect("Error parsing example input");
    let actual = part2(&input).expect("Error solving part 2");

    assert_eq!("6,1".to_string(), actual);
}

#[test]
fn part2_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let actual = part2(&parsed).expect("Error solving part 2");

    assert_eq!("45,18".to_string(), actual);
}
