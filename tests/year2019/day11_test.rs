use advent_of_code::year2019::day11::*;

const PUZZLE_INPUT: &str = include_str!("../../input/year2019/day11.txt");
//const EXAMPLE_INPUT: &str = "";

// Input is another IntCode Program so no parse example
// #[test]
// fn parse_example() {
//     let _parsed = parse(EXAMPLE_INPUT).expect("Error parsing input");

//     todo!("assert_eq!(expected, parsed);")
// }

// No example program to generate example output so can't test
// #[test]
// fn part1_example() {
//     let input = parse(EXAMPLE_INPUT).expect("Error parsing input");
//     let _answer = part1(&input).expect("Error solving part 1");

//     todo!("assert_eq!(expected, answer);")
// }

#[test]
fn part1_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let answer = part1(&parsed).expect("Error solving part 1");

    assert_eq!(2041, answer);
}

// No example program to test against
// #[test]
// fn part2_example() {
//     let input = parse(EXAMPLE_INPUT).expect("Error parsing example input");
//     let _answer = part2(&input).expect("Error solving part 2");

//     //assert_eq!(expected, answer);
// }

#[test]
fn part2_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let answer = part2(&parsed).expect("Error solving part 2");

    assert_eq!("ZRZPKEZR", answer);
}
