use advent_of_code::error::AdventError;
use advent_of_code::year2023::day02::*;
use std::str::FromStr;

#[test]
fn colour_from_str() {
    use Colour::*;

    assert_eq!(Colour::from_str("9 red"), Ok(Red(9)));
    assert_eq!(Colour::from_str("3 blue"), Ok(Blue(3)));
    assert_eq!(Colour::from_str("0 green"), Ok(Green(0)));

    assert_eq!(
        Colour::from_str("wrong"),
        Err(AdventError::SplitOnce("wrong".into(), ' '.into()))
    );
    // assert_eq!(
    //     Colour::from_str("test wrong"),
    //     Err(AdventError::ParseInt(std::num::ParseIntError{ kind: std::num::IntErrorKind::InvalidDigit}))
    // );
    assert_eq!(
        Colour::from_str("1 wrong"),
        Err(AdventError::StringToEnum("wrong".into()))
    );
}

#[test]
fn parse_one() {
    use Colour::*;
    let parsed = parse("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green")
        .expect("Failed to parse test input");

    let expected = vec![Game {
        id: 5,
        draws: vec![
            Draw {
                colours: vec![Red(6), Blue(1), Green(3)],
            },
            Draw {
                colours: vec![Blue(2), Red(1), Green(2)],
            },
        ],
    }];

    assert_eq!(parsed, expected);
}

#[test]
fn parse_two_digit() {
    use Colour::*;
    let parsed = parse("Game 1: 20 red").expect("Failed to parse test input");

    let expected = vec![Game {
        id: 1,
        draws: vec![Draw {
            colours: vec![Red(20)],
        }],
    }];

    assert_eq!(parsed, expected);
}

#[test]
fn parse_multiple() {
    use Colour::*;

    let parsed = parse(
        "\
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
    )
    .expect("Failed to parse test input");

    let expected = vec![
        Game {
            id: 2,
            draws: vec![
                Draw {
                    colours: vec![Blue(1), Green(2)],
                },
                Draw {
                    colours: vec![Green(3), Blue(4), Red(1)],
                },
                Draw {
                    colours: vec![Green(1), Blue(1)],
                },
            ],
        },
        Game {
            id: 5,
            draws: vec![
                Draw {
                    colours: vec![Red(6), Blue(1), Green(3)],
                },
                Draw {
                    colours: vec![Blue(2), Red(1), Green(2)],
                },
            ],
        },
    ];

    assert_eq!(parsed, expected);
}

const EXAMPLE: &str = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE).expect("Failed to parse test input");

    let answer = part1(&input).expect("Failed to calculate answer");
    assert_eq!(answer, 8);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE).expect("Failed to parse test input");

    let answer = part2(&input).expect("Failed to calculate answer");
    assert_eq!(answer, 2286);
}
