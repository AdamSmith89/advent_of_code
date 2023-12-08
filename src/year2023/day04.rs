use crate::error::AdventError;

type ParsedInput = Vec<Card>;

pub fn parse(input: &str) -> color_eyre::Result<ParsedInput> {
    // it looks like each card has two lists of numbers separated by a vertical bar (|):
    // a list of winning numbers and then a list of numbers you have
    // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    let mut cards = Vec::new();

    for line in input.lines() {
        let (id, card) = line
            .split_once(':')
            .ok_or(AdventError::SplitOnce(line.into(), ':'.into()))?;

        let (_, id) = id
            .split_once(' ')
            .ok_or(AdventError::SplitOnce(id.into(), ' '.into()))?;
        let id = id.to_string().trim().parse::<usize>()?;

        let (winners, numbers) = card
            .split_once('|')
            .ok_or(AdventError::SplitOnce(card.into(), '|'.into()))?;

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

#[derive(Clone, Debug)]
pub struct Card {
    pub id: usize,
    pub winners: Vec<u32>,
    pub numbers: Vec<u32>,
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
