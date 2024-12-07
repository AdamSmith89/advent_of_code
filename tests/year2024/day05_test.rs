use advent_of_code::year2024::day05::*;

const PUZZLE_INPUT: &str = include_str!("../../input/year2024/day05.txt");
const EXAMPLE_INPUT: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

#[test]
fn parse_example() {
    let (rules, updates) = parse(EXAMPLE_INPUT).expect("Error parsing input");

    use Order::*;
    assert!(rules.get(&47).is_some());
    assert_eq!(
        &vec![
            Before(53),
            After(97),
            Before(13),
            After(75),
            Before(61),
            Before(29)
        ],
        rules.get(&47).unwrap()
    );

    assert!(rules.get(&61).is_some());
    assert_eq!(
        &vec![
            After(97),
            Before(13),
            Before(53),
            Before(29),
            After(47),
            After(75)
        ],
        rules.get(&61).unwrap()
    );

    assert_eq!(
        vec![
            Update {
                pages: vec![75, 47, 61, 53, 29]
            },
            Update {
                pages: vec![97, 61, 53, 29, 13]
            },
            Update {
                pages: vec![75, 29, 13]
            },
            Update {
                pages: vec![75, 97, 47, 61, 53]
            },
            Update {
                pages: vec![61, 13, 29]
            },
            Update {
                pages: vec![97, 13, 75, 29, 47]
            },
        ],
        updates
    );
}

#[test]
fn part1_example() {
    let input = parse(EXAMPLE_INPUT).expect("Error parsing input");
    let actual = part1(&input).expect("Error solving part 1");

    assert_eq!(143, actual);
}

#[test]
fn part1_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let actual = part1(&parsed).expect("Error solving part 1");

    assert_eq!(4766, actual);
}

#[test]
fn part2_example() {
    let input = parse(EXAMPLE_INPUT).expect("Error parsing example input");
    let actual = part2(&input).expect("Error solving part 2");

    assert_eq!(123, actual);
}

#[test]
fn part2_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let _actual = part2(&parsed).expect("Error solving part 2");

    //assert_eq!(expected, actual);
}
