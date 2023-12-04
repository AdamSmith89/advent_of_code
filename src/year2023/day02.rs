use std::{num::ParseIntError, str::FromStr};

pub fn parse(input: &str) -> color_eyre::Result<Vec<Game>> {
    // E.g.: Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    use Year2023Day02Error::*;

    let mut games: Vec<Game> = Vec::new();

    for line in input.lines() {
        let (game, draws) = line.split_once(':').ok_or(Split(':'))?;

        let (_, id) = game.split_once(' ').ok_or(Split(' '))?;
        let mut game = Game::new(id.to_string().parse()?);

        let info = draws.trim();
        for draw_s in info.split(';') {
            let draw_s = draw_s.trim();
            let mut draw = Draw::new();

            for colour in draw_s.split(',') {
                let colour = colour.trim();
                draw.colours.push(Colour::from_str(colour)?);
            }

            game.draws.push(draw);
        }

        games.push(game);
    }

    Ok(games)
}

pub fn part1(games: &[Game]) -> color_eyre::Result<u32> {
    // once a bag has been loaded with cubes, the Elf will reach into the bag,
    // grab a handful of random cubes, show them to you, and then put them back
    // in the bag. He'll do this a few times per game.
    // The Elf would first like to know which games would have been possible if the bag
    // contained only 12 red cubes, 13 green cubes, and 14 blue cubes?

    let red_limit = 12;
    let green_limit = 13;
    let blue_limit = 14;

    let mut games = games.to_vec();
    games.retain(|game| {
        game.draws.iter().all(|draw| {
            draw.colours.iter().all(|colour| match colour {
                Colour::Red(num) => *num <= red_limit,
                Colour::Green(num) => *num <= green_limit,
                Colour::Blue(num) => *num <= blue_limit,
            })
        })
    });

    Ok(games.iter().map(|game| game.id).sum())
}

pub fn part2(games: &[Game]) -> color_eyre::Result<u32> {
    // what is the fewest number of cubes of each color that could have
    // been in the bag to make the game possible?
    // The power of a set of cubes is equal to the numbers of red, green,
    // and blue cubes multiplied together.
    // What is the sum of the power of these sets?
    use Colour::*;

    let sum: u32 = games
        .iter()
        .map(|game| {
            let (r, g, b) =
                game.draws
                    .iter()
                    .fold((0, 0, 0), |(mut max_r, mut max_g, mut max_b), draw| {
                        draw.colours.iter().for_each(|colour| match colour {
                            Red(draw_r) => max_r = std::cmp::max(max_r, *draw_r),
                            Green(draw_g) => max_g = std::cmp::max(max_g, *draw_g),
                            Blue(draw_b) => max_b = std::cmp::max(max_b, *draw_b),
                        });

                        (max_r, max_g, max_b)
                    });

            r * g * b // power is max of each set of cubes
        })
        .sum();

    Ok(sum)
}

#[derive(Debug, thiserror::Error, PartialEq)]
enum Year2023Day02Error {
    #[error("Failed to split on {0}")]
    Split(char),
    #[error("Failed to parse int: {0}")]
    ParseInt(String),
    #[error("Failed to parse colour: {0}")]
    ParseColour(String),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Game {
    id: u32,
    draws: Vec<Draw>,
}

impl Game {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            draws: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Draw {
    colours: Vec<Colour>,
}

impl Draw {
    fn new() -> Self {
        Self {
            colours: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Colour {
    Red(u32),
    Green(u32),
    Blue(u32),
}

impl FromStr for Colour {
    type Err = Year2023Day02Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Colour::*;
        use Year2023Day02Error::*;

        let (num, colour) = s.split_once(' ').ok_or(Split(' '))?;
        let num: u32 = num
            .to_string()
            .parse()
            .map_err(|err: ParseIntError| ParseInt(format!("Parse error for '{num}': {}", err)))?;

        match colour {
            "red" => Ok(Red(num)),
            "green" => Ok(Green(num)),
            "blue" => Ok(Blue(num)),
            _ => Err(ParseColour(colour.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colour_from_str() {
        use Colour::*;
        use Year2023Day02Error::*;

        assert_eq!(Colour::from_str("9 red"), Ok(Red(9)));
        assert_eq!(Colour::from_str("3 blue"), Ok(Blue(3)));
        assert_eq!(Colour::from_str("0 green"), Ok(Green(0)));

        assert_eq!(Colour::from_str("wrong"), Err(Split(' ')));
        assert_eq!(
            Colour::from_str("test wrong"),
            Err(ParseInt(
                "Parse error for 'test': invalid digit found in string".to_string()
            ))
        );
        assert_eq!(
            Colour::from_str("1 wrong"),
            Err(ParseColour("wrong".to_string()))
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
}
