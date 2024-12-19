use std::collections::HashMap;

use itertools::Itertools;

use crate::error::AdventError;

type ParsedInput = (Vec<String>, Vec<String>);

pub fn parse(input: &str) -> color_eyre::Result<ParsedInput> {
    let mut lines = input.lines();

    let towels = lines.next().ok_or(AdventError::EndOfIterator)?;
    let towels = towels
        .split(',')
        .map(str::trim)
        .map(String::from)
        .collect_vec();

    lines.next().ok_or(AdventError::EndOfIterator)?;

    let designs = lines.map(String::from).collect_vec();

    Ok((towels, designs))
}

pub fn part1((towels, designs): &ParsedInput) -> color_eyre::Result<u64> {
    let mut cache = HashMap::<String, u64>::new();
    let mut total_designs = 0;

    for design in designs {
        total_designs += is_design_possible(design, towels, true, &mut cache);
    }

    Ok(total_designs)
}

pub fn part2((towels, designs): &ParsedInput) -> color_eyre::Result<u64> {
    let mut cache = HashMap::<String, u64>::new();
    let mut total_designs = 0;

    for design in designs {
        total_designs += is_design_possible(design, towels, false, &mut cache);
    }

    Ok(total_designs)
}

fn is_design_possible(
    design: &str,
    towels: &Vec<String>,
    stop_on_first_match: bool,
    cache: &mut HashMap<String, u64>,
) -> u64 {
    if let Some(count) = cache.get(design) {
        return *count;
    }

    // Collect all possible sub options first so we can check for success before going all the way
    // down one route - essentially BFS vs. DFS
    let mut sub_designs = Vec::new();

    for towel in towels {
        if design.starts_with(towel) {
            let rem = &design[towel.len()..];
            sub_designs.push(rem);

            if rem.is_empty() && stop_on_first_match {
                return 1;
            }
        }
    }

    let mut count = 0;
    for design in sub_designs {
        if design.is_empty() {
            count += 1;
        } else {
            count += is_design_possible(design, towels, stop_on_first_match, cache);

            if count == 1 && stop_on_first_match {
                return 1;
            }
        }
    }

    cache.insert(design.to_string(), count);
    count
}
