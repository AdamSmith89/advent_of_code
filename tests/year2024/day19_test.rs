use advent_of_code::year2024::day19::*;

const PUZZLE_INPUT: &str = include_str!("../../input/year2024/day19.txt");
const EXAMPLE_INPUT: &str = "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

#[test]
fn parse_example() {
    let (towels, designs) = parse(EXAMPLE_INPUT).expect("Error parsing input");

    assert_eq!(
        vec![
            String::from("r"),
            String::from("wr"),
            String::from("b"),
            String::from("g"),
            String::from("bwu"),
            String::from("rb"),
            String::from("gb"),
            String::from("br"),
        ],
        towels
    );

    assert_eq!(
        vec![
            String::from("brwrr"),
            String::from("bggr"),
            String::from("gbbr"),
            String::from("rrbgbr"),
            String::from("ubwu"),
            String::from("bwurrg"),
            String::from("brgr"),
            String::from("bbrgwb"),
        ],
        designs
    );
}

#[test]
fn part1_example() {
    let input = parse(EXAMPLE_INPUT).expect("Error parsing input");
    let actual = part1(&input).expect("Error solving part 1");

    assert_eq!(6, actual);
}

#[test]
fn part1_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let actual = part1(&parsed).expect("Error solving part 1");

    assert_eq!(342, actual);
}

#[test]
fn part2_example() {
    let input = parse(EXAMPLE_INPUT).expect("Error parsing example input");
    let actual = part2(&input).expect("Error solving part 2");

    assert_eq!(16, actual);
}

#[test]
fn part2_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let actual = part2(&parsed).expect("Error solving part 2");

    assert_eq!(891192814474630, actual);
}
