use std::str::FromStr;

use itertools::Itertools;

use crate::error::AdventError;

type ParsedInput = Vec<Round>;

pub fn parse(input: &str) -> color_eyre::Result<ParsedInput> {
    Ok(input
        .lines()
        .map(Round::from_str)
        .collect::<Result<_, _>>()?)
}

pub fn part1(rounds: &ParsedInput) -> color_eyre::Result<u64> {
    // Each hand wins an amount equal to its bid multiplied by its rank,
    // where the weakest hand gets rank 1, the second-weakest hand gets rank 2,
    // and so on up to the strongest hand.
    // determine the total winnings of this set of hands by adding up the result of multiplying each hand's bid with its rank

    get_winnings(rounds, false)
}

pub fn part2(rounds: &ParsedInput) -> color_eyre::Result<u64> {
    // Now, J cards are jokers - wildcards that can act like whatever card would make the hand the strongest type possible.

    get_winnings(rounds, true)
}

fn get_winnings(rounds: &ParsedInput, jacks_wild: bool) -> color_eyre::Result<u64> {
    let mut rounds = rounds
        .iter()
        .map(|round| Ok((Hand::new(&round.hand, jacks_wild)?, round.bid)))
        .collect::<Result<Vec<(Hand, u64)>, AdventError>>()?;

    rounds.sort();

    let winnings = rounds
        .iter()
        .enumerate()
        .map(|(rank, (_, bid))| {
            bid * (rank as u64 + 1)
        })
        .sum();

    Ok(winnings)
}

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Round {
    pub hand: Vec<char>,
    pub bid: u64,
}

impl FromStr for Round {
    type Err = AdventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand, bid) = s.split_once(' ').ok_or(AdventError::SplitOnce(s.into(), ' '.into()))?;

        Ok(Round {
            hand: hand.chars().collect_vec(),
            bid: bid.parse()?,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord)]
pub enum Hand {
    FiveKind(Vec<u32>),
    FourKind(Vec<u32>),
    FullHouse(Vec<u32>),
    ThreeKind(Vec<u32>),
    TwoPair(Vec<u32>),
    OnePair(Vec<u32>),
    HighCard(Vec<u32>),
}

impl Hand {
    fn as_rank(&self) -> u8 {
        match self {
            Hand::FiveKind(_) => 7,
            Hand::FourKind(_) => 6,
            Hand::FullHouse(_) => 5,
            Hand::ThreeKind(_) => 4,
            Hand::TwoPair(_) => 3,
            Hand::OnePair(_) => 2,
            Hand::HighCard(_) => 1,
        }
    }

    fn raw(&self) -> Vec<u32> {
        match self {
            Hand::FiveKind(raw)
            | Hand::FourKind(raw)
            | Hand::FullHouse(raw)
            | Hand::ThreeKind(raw)
            | Hand::TwoPair(raw)
            | Hand::OnePair(raw)
            | Hand::HighCard(raw) => raw.clone(),
        }
    }

    fn new(cards: &Vec<char>, jacks_wild: bool) -> Result<Self, AdventError> {
        let hand = cards
            .iter()
            .filter_map(|ch| card_as_digit(ch, jacks_wild))
            .collect_vec();
        let mut card_counts = cards.iter().sorted().dedup_with_count().collect_vec();
        card_counts.sort();

        let (max, _) = card_counts.last().unwrap();
        let num_jokers = if jacks_wild {
            if let Some((count, _)) = card_counts.iter().find(|(_, &card)| card == 'J') {
                *count
            } else {
                0
            }
        } else {
            0
        };
        let num_pairs = card_counts.iter().filter(|(count, _)| *count == 2).count();

        let hand = match max {
            5 => Ok(Hand::FiveKind(hand)),
            4 => {
                if num_jokers == 1 || num_jokers == 4 {
                    Ok(Hand::FiveKind(hand))
                } else {
                    Ok(Hand::FourKind(hand))
                }
            }
            3 => {
                if num_jokers == 3 {
                    if num_pairs == 1 {
                        Ok(Hand::FiveKind(hand))
                    } else {
                        Ok(Hand::FourKind(hand))
                    }
                } else if num_jokers == 2 {
                    Ok(Hand::FiveKind(hand))
                } else if num_jokers == 1 {
                    Ok(Hand::FourKind(hand))
                } else if num_pairs == 1 {
                    Ok(Hand::FullHouse(hand))
                } else {
                    Ok(Hand::ThreeKind(hand))
                }
            }
            2 => {
                if num_pairs == 2 {
                    if num_jokers == 2 {
                        Ok(Hand::FourKind(hand))
                    } else if num_jokers == 1 {
                        Ok(Hand::FullHouse(hand))
                    } else {
                        Ok(Hand::TwoPair(hand))
                    }
                } else {
                    if num_jokers >= 1 {
                        Ok(Hand::ThreeKind(hand))
                    } else {
                        Ok(Hand::OnePair(hand))
                    }
                }
            }
            1 => {
                if num_jokers == 1 {
                    Ok(Hand::OnePair(hand))
                } else {
                    Ok(Hand::HighCard(hand))
                }
            }
            _ => Err(AdventError::UnknownPattern(max.to_string())),
        };

        hand
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.as_rank() > other.as_rank() {
            Some(std::cmp::Ordering::Greater)
        } else if self.as_rank() < other.as_rank() {
            Some(std::cmp::Ordering::Less)
        } else {
            if self.raw().gt(&other.raw()) {
                Some(std::cmp::Ordering::Greater)
            } else {
                Some(std::cmp::Ordering::Less)
            }
        }
    }
}

fn card_as_digit(card: &char, jacks_wild: bool) -> Option<u32> {
    match card {
        'A' => Some(14),
        'K' => Some(13),
        'Q' => Some(12),
        'J' => {
            if jacks_wild {
                Some(1)
            } else {
                Some(11)
            }
        }
        'T' => Some(10),
        _ => card.to_digit(10),
    }
}
