use std::collections::HashMap;

use itertools::Itertools;

use crate::error::AdventError;

type ParsedInput = Input;

pub fn parse(input: &str) -> color_eyre::Result<ParsedInput> {
    let part1 = input
        .lines()
        .map(|line| {
            let (springs, groups) = line
                .split_once(' ')
                .ok_or(AdventError::SplitOnce(line.into(), ' '.into()))?;

            let springs = springs.chars().collect_vec();
            let groups: Vec<usize> = groups
                .split(',')
                .map(|group| group.parse::<usize>())
                .try_collect()?;

            Ok(Row { springs, groups })
        })
        .collect::<Result<Vec<Row>, AdventError>>()?;

    let part2 = part1
        .iter()
        .map(|row| {
            let mut new_row = row.clone();
            (0..4).for_each(|_| {
                new_row.springs.push('?');
                new_row.springs.extend(row.springs.iter());
                new_row.groups.extend(row.groups.iter());
            });
            new_row
        })
        .collect_vec();

    Ok(Input { part1, part2 })
}

pub fn part1(input: &ParsedInput) -> color_eyre::Result<u64> {
    let mut total = 0;
    for row in &input.part1 {
        let springs = row.springs.clone();
        let groups = row.groups.clone();

        let mut cache = HashMap::new();
        let answer = cached_solve(&springs, &groups, &mut cache)?;
        total += answer;
    }
    Ok(total)
}

pub fn part2(input: &ParsedInput) -> color_eyre::Result<u64> {
    let mut total = 0;
    for row in &input.part2 {
        let springs = row.springs.clone();
        let groups = row.groups.clone();

        let mut cache = HashMap::new();
        let answer = cached_solve(&springs, &groups, &mut cache)?;
        total += answer;
    }
    Ok(total)
}

fn cached_solve<'a>(
    springs: &'a [char],
    groups: &'a [usize],
    cache: &mut HashMap<(&'a [char], &'a [usize]), u64>,
) -> color_eyre::Result<u64> {
    if let Some(result) = cache.get(&(springs, groups)) {
        Ok(*result)
    } else {
        let result = solve(springs, groups, cache)?;
        cache.insert((springs, groups), result);
        Ok(result)
    }
}

fn solve<'a>(
    springs: &'a [char],
    groups: &'a [usize],
    cache: &mut HashMap<(&'a [char], &'a [usize]), u64>,
) -> color_eyre::Result<u64> {
    if groups.is_empty() {
        // No groups and no remaining #'s means we've finished and matched
        if !springs.contains(&'#') {
            return Ok(1);
        }
        // No groups but some #'s remaining means this doesn't match
        else {
            return Ok(0);
        }
    }

    // No springs but some groups left
    if springs.is_empty() {
        return Ok(0);
    }

    // Both springs and groups have something in them at this point
    let next_spring = springs[0];
    let next_group = groups[0];

    let out = match next_spring {
        '.' => cached_solve(&springs[1..], groups, cache)?,
        '#' => resolve_pound(springs, groups, cache, next_group)?,
        '?' => {
            cached_solve(&springs[1..], groups, cache)?
                + resolve_pound(springs, groups, cache, next_group)?
        }
        _ => return Err(AdventError::UnknownPattern(next_spring.into()).into()),
    };

    Ok(out)
}

fn resolve_pound<'a>(
    springs: &'a [char],
    groups: &'a [usize],
    cache: &mut HashMap<(&'a [char], &'a [usize]), u64>,
    next_group: usize,
) -> color_eyre::Result<u64> {
    if next_group > springs.len() {
        return Ok(0);
    }

    let this_group = &springs[..next_group];

    // Some of the springs in the proposed group are '.' which is invalid
    if this_group.iter().any(|ch| *ch == '.') {
        return Ok(0);
    }

    if springs.len() == next_group {
        // The remaining springs match the group length, so this is a match
        // if this is also the last group
        if groups.len() == 1 {
            return Ok(1);
        } else {
            return Ok(0);
        }
    }

    // There's more springs to process
    if matches!(springs[next_group], '?' | '.') {
        return cached_solve(&springs[next_group + 1..], &groups[1..], cache);
    }

    // The next character is '#' which wouldn't be valid for this group
    Ok(0)
}

#[derive(Debug, Eq, PartialEq)]
pub struct Input {
    pub part1: Vec<Row>,
    pub part2: Vec<Row>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Row {
    pub springs: Vec<char>,
    pub groups: Vec<usize>,
}
