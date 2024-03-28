use advent_of_code::year2019::day06::*;

const PUZZLE_INPUT: &str = include_str!("../../input/year2019/day06.txt");
const EXAMPLE_INPUT_PART1: &str = "\
COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L";

const EXAMPLE_INPUT_PART2: &str = "\
COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN";

#[test]
fn parse_example() {
    let parsed = parse(EXAMPLE_INPUT_PART1).expect("Error parsing input");

    assert_eq!(
        vec![
            (String::from("COM"), String::from("B")),
            (String::from("B"), String::from("C")),
            (String::from("C"), String::from("D")),
            (String::from("D"), String::from("E")),
            (String::from("E"), String::from("F")),
            (String::from("B"), String::from("G")),
            (String::from("G"), String::from("H")),
            (String::from("D"), String::from("I")),
            (String::from("E"), String::from("J")),
            (String::from("J"), String::from("K")),
            (String::from("K"), String::from("L")),
        ],
        parsed
    );
}

#[test]
fn part1_example() {
    let input = parse(EXAMPLE_INPUT_PART1).expect("Error parsing input");
    let answer = part1(&input).expect("Error solving part 1");

    assert_eq!(42, answer);
}

#[test]
fn orbits_out_of_order() {
    // COM -- A -- B -- C
    // For a total of 6 orbits
    // A orbits COM
    // B orbits A and COM
    // C orbits B, A, and COM
    // but input out of order
    const OUT_OF_ORDER_INPUT: &str = "\
COM)A
B)C
A)B";

    let input = parse(OUT_OF_ORDER_INPUT).expect("Error parsing input");
    let answer = part1(&input).expect("Error solving part 1");

    assert_eq!(6, answer);
}

#[test]
fn part1_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let answer = part1(&parsed).expect("Error solving part 1");

    assert_eq!(308790, answer);
}

#[test]
fn part2_example() {
    let input = parse(EXAMPLE_INPUT_PART2).expect("Error parsing example input");
    let answer = part2(&input).expect("Error solving part 2");

    assert_eq!(4, answer);
}

#[test]
fn part2_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let answer = part2(&parsed).expect("Error solving part 2");

    assert_eq!(472, answer);
}
