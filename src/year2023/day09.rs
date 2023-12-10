use itertools::Itertools;

use crate::error::AdventError;

type ParsedInput = Vec<Reading>;

pub fn parse(input: &str) -> color_eyre::Result<ParsedInput> {
    Ok(input
        .lines()
        .map(|line| line.split_ascii_whitespace())
        .map(|history| history.map(|value| value.parse::<i32>()).try_collect())
        .try_collect()?)
}

pub fn part1(readings: &ParsedInput) -> color_eyre::Result<i32> {
    let next_values: Vec<i32> = readings.iter().map(next_in_sequence).try_collect()?;

    Ok(next_values.iter().sum())
}

pub fn part2(readings: &ParsedInput) -> color_eyre::Result<i32> {
    let prev_values: Vec<i32> = readings.iter().map(prev_in_sequence).try_collect()?;

    Ok(prev_values.iter().sum())
}

pub fn next_in_sequence(reading: &Reading) -> color_eyre::Result<i32> {
    let mut lasts = Vec::new();
    let mut cur = reading.history.clone();

    while cur.iter().any(|v| *v != 0) {
        lasts.push(cur.last().ok_or(AdventError::EmptySlice)?.clone());

        cur = cur.iter().tuple_windows().map(|(l, r)| r - l).collect_vec();
    }

    Ok(lasts.iter().sum())
}

pub fn prev_in_sequence(reading: &Reading) -> color_eyre::Result<i32> {
    let mut firsts = Vec::new();
    let mut cur = reading.history.clone();

    while cur.iter().any(|v| *v != 0) {
        firsts.push(cur.first().ok_or(AdventError::EmptySlice)?.clone());

        cur = cur.iter().tuple_windows().map(|(l, r)| r - l).collect_vec();
    }

    Ok(firsts.iter().rev().fold(0, |acc, x| x - acc))
}

#[derive(Debug, Eq, PartialEq)]
pub struct Reading {
    pub history: Vec<i32>,
}

impl FromIterator<i32> for Reading {
    fn from_iter<T: IntoIterator<Item = i32>>(iter: T) -> Self {
        Self {
            history: iter.into_iter().collect(),
        }
    }
}
