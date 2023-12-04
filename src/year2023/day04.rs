type ParsedInput = Vec<Card>;

#[derive(Clone, Debug)]
pub struct Card {
    id: usize,
    winners: Vec<u32>,
    numbers: Vec<u32>,
}

impl Card {
    fn get_matches(&self) -> Vec<u32> {
        self.numbers
            .iter()
            .filter_map(|&num| {
                if self.winners.contains(&num) {
                    Some(num)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }
}

pub fn parse(input: &str) -> color_eyre::Result<ParsedInput> {
    // it looks like each card has two lists of numbers separated by a vertical bar (|):
    // a list of winning numbers and then a list of numbers you have
    // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    use Year2023Day04Error::*;

    let mut cards = Vec::new();

    for line in input.lines() {
        let (id, card) = line.split_once(':').ok_or(Split(':'))?;

        let (_, id) = id.split_once(' ').ok_or(Split(' '))?;
        let id =
            id.to_string()
                .trim()
                .parse::<usize>()
                .map_err(|err: std::num::ParseIntError| {
                    ParseInt(format!("Parse error for '{id}': {}", err))
                })?;

        let (winners, numbers) = card.split_once('|').ok_or(Split('|'))?;

        let winners = winners
            .split_whitespace()
            .map(|s| s.to_string().parse::<u32>())
            .collect::<Result<Vec<_>, _>>()?;

        let numbers = numbers
            .split_whitespace()
            .map(|s| s.to_string().parse::<u32>())
            .collect::<Result<Vec<_>, _>>()?;

        cards.push(Card {
            id,
            winners,
            numbers,
        });
    }

    Ok(cards)
}

pub fn part1(cards: &ParsedInput) -> color_eyre::Result<u32> {
    // which of the numbers you have appear in the list of winning numbers.
    // The first match makes the card worth one point and each match after
    // the first doubles the point value of that card.

    let mut total_points = 0;

    for card in cards {
        let matches = card.get_matches().len() as u32;

        let points = match matches.cmp(&1) {
            std::cmp::Ordering::Equal => 1,
            std::cmp::Ordering::Greater => 2u32.pow(matches - 1),
            std::cmp::Ordering::Less => 0,
        };

        total_points += points;
    }

    Ok(total_points)
}

pub fn part2(cards: &ParsedInput) -> color_eyre::Result<usize> {
    // you win copies of the scratchcards below the winning card equal to the number of matches
    // how many total scratchcards do you end up with

    let mut card_results = Vec::new();
    for card in cards {
        let matches = card.get_matches();

        let mut match_ids = Vec::new();
        for offset in 0..matches.len() {
            let id = card.id + offset;

            if id == cards.len() {
                break;
            }

            match_ids.push(id);
        }

        card_results.push((match_ids, 1));
    }

    let mut idx = 0;
    loop {
        if idx == card_results.len() {
            break;
        }

        let match_ids = card_results[idx].0.clone();
        let multiple = card_results[idx].1;

        for id in match_ids {
            card_results[id].1 += multiple;
        }

        idx += 1;
    }

    Ok(card_results.iter().map(|(_, multiple)| multiple).sum())
}

#[derive(Debug, thiserror::Error, PartialEq)]
enum Year2023Day04Error {
    #[error("Failed to split on {0}")]
    Split(char),
    #[error("Failed to parse int {0}")]
    ParseInt(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn parse_test() {
        let cards = parse(EXAMPLE).expect("Error parsing example input");

        assert_eq!(cards.len(), 6);
        assert_eq!(cards[0].winners, vec![41, 48, 83, 86, 17]);
        assert_eq!(cards[1].numbers, vec![61, 30, 68, 82, 17, 32, 24, 19]);
        assert_eq!(cards[2].winners, vec![1, 21, 53, 59, 44]);
        assert_eq!(cards[2].numbers, vec![69, 82, 63, 72, 16, 21, 14, 1]);
    }

    #[test]
    fn part1_test() {
        let input = parse(EXAMPLE).expect("Failed to parse example input");
        let answer = part1(&input).expect("Error solving part 1");

        assert_eq!(answer, 13);
    }

    #[test]
    fn part2_test() {
        let input = parse(EXAMPLE).expect("Failed to parse example input");
        let answer = part2(&input).expect("Error solving part 2");

        assert_eq!(answer, 30);
    }
}
