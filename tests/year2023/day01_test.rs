use advent_of_code::year2023::day01::*;

const PUZZLE_INPUT: &str = include_str!("../../input/year2023/day01.txt");
const PART1_EXAMPLE: &str = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

#[test]
fn part1_example() {
    let input = parse(PART1_EXAMPLE).expect("Error parsing input");
    let answer = part1(&input).expect("Error solving part 1");

    assert_eq!(answer, 142);
}

#[test]
fn part1_real() {
    let input = parse(PUZZLE_INPUT).expect("Error parsing input");
    let answer = part1(&input).expect("Error solving part 1");

    assert_eq!(answer, 54388);
}

const PART2_EXAMPLE: &str = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

#[test]
fn part2_example() {
    let input = parse(PART2_EXAMPLE).expect("Error parsing input");
    let answer = part2(&input).expect("Error solving part 2");

    assert_eq!(answer, 281);
}

#[test]
fn part2_real() {
    let input = parse(PUZZLE_INPUT).expect("Error parsing input");
    let answer = part2(&input).expect("Error solving part 2");

    assert_eq!(answer, 53515);
}

const TROUBLESOME: &str = "\
1hggcqcstgpmg26lzxtltcgg
vfzmncfonexxkzlcstqhxvtwoplsglsix1kpkssfz
254zhtrlvpfs";

#[test]
fn part2_edge_case() {
    let input = parse(TROUBLESOME).expect("Error parsing input");
    let answer = part2(&input).expect("Error solving part 2");

    assert_eq!(answer, 16 + 11 + 24);
}
