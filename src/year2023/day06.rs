use std::num::ParseIntError;

use crate::error::AdventError;

type ParsedInput = Input;

pub fn parse(input: &str) -> color_eyre::Result<ParsedInput> {
    let mut lines = input.lines();
    let times = lines.next().ok_or(AdventError::EndOfIterator)?;
    let distances = lines.next().ok_or(AdventError::EndOfIterator)?;

    let (_, times) = times
        .split_once(':')
        .ok_or(AdventError::SplitOnce(times.into(), ':'.into()))?;
    let times_pt1 = times
        .split_ascii_whitespace()
        .map(|time| time.parse::<u64>())
        .collect::<Result<Vec<u64>, ParseIntError>>()?;
    let time_pt2 = times
        .split_ascii_whitespace()
        .collect::<String>()
        .parse::<u64>()?;

    let (_, distances) = distances
        .split_once(':')
        .ok_or(AdventError::SplitOnce(distances.into(), ':'.into()))?;
    let distances_pt1 = distances
        .split_ascii_whitespace()
        .map(|dist| dist.parse::<u64>())
        .collect::<Result<Vec<u64>, ParseIntError>>()?;
    let distance_pt2 = distances
        .split_ascii_whitespace()
        .collect::<String>()
        .parse::<u64>()?;

    let part1 = times_pt1
        .iter()
        .zip(distances_pt1.iter())
        .map(|(&time, &dist)| Race { time, dist })
        .collect::<Vec<_>>();

    Ok(Input {
        part1,
        part2: Race {
            time: time_pt2,
            dist: distance_pt2,
        },
    })
}

pub fn part1(input: &ParsedInput) -> color_eyre::Result<u64> {
    // Determine the number of ways you could beat the record in each race.
    // What do you get if you multiply these numbers together?

    let mut race_wins = Vec::new();
    for race in &input.part1 {
        let first_loss_index = (0..=race.time)
            .find(|hold_time| hold_time * (race.time - hold_time) > race.dist)
            .unwrap();

        race_wins.push(race.time - (2 * first_loss_index) + 1);
    }

    Ok(race_wins.iter().product())
}

pub fn part2(input: &ParsedInput) -> color_eyre::Result<u64> {
    let race = &input.part2;

    let first_loss_index = (0..=race.time)
        .find(|hold_time| hold_time * (race.time - hold_time) > race.dist)
        .unwrap();

    Ok(race.time - (2 * first_loss_index) + 1)
}

#[derive(Debug, Eq, PartialEq)]
pub struct Input {
    pub part1: Vec<Race>,
    pub part2: Race,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Race {
    pub time: u64,
    pub dist: u64,
}
