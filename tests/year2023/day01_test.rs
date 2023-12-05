use advent_of_code::year2023::day01::*;

const PART1_EXAMPLE: &str = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

#[test]
fn part1_test() {
    let input = parse(PART1_EXAMPLE).expect("Failed to parse test input");

    let answer = part1(&input).expect("Failed to calculate answer");
    assert_eq!(answer, 142);
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
fn part2_test() {
    let input = parse(PART2_EXAMPLE).expect("Failed to parse test input");

    let answer = part2(&input).expect("Failed to calculate answer");
    assert_eq!(answer, 281);
}

const TROUBLESOME: &str = "\
1hggcqcstgpmg26lzxtltcgg
vfzmncfonexxkzlcstqhxvtwoplsglsix1kpkssfz
254zhtrlvpfs";

#[test]
fn troublesome_test() {
    let input = parse(TROUBLESOME).expect("Failed to parse test input");

    let answer = part2(&input).expect("Failed to calculate answer");
    assert_eq!(answer, 16 + 11 + 24);
}
