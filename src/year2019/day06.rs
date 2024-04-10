use std::{
    collections::{HashMap},
};

use itertools::Itertools;

use crate::error::AdventError;

type ParsedInput = Vec<(String, String)>;

pub fn parse(input: &str) -> color_eyre::Result<ParsedInput> {
    Ok(input
        .lines()
        .filter_map(|line| line.split_once(')'))
        .map(|(s1, s2)| (s1.to_string(), s2.to_string()))
        .collect_vec())
}

pub fn part1(orbit_map: &ParsedInput) -> color_eyre::Result<u32> {
    let mut orbit_counts = HashMap::new();

    Ok(orbit_map
        .iter()
        .map(|orbit| calc_num_orbits(orbit, orbit_map, &mut orbit_counts, None))
        .sum())
}

fn calc_num_orbits(
    (parent, child): &(String, String),
    orbit_map: &ParsedInput,
    orbit_counts: &mut HashMap<String, u32>,
    stop_at: Option<String>,
) -> u32 {
    // Have we already calculated the orbits for the child body?
    if let Some(child_orbits) = orbit_counts.get(child) {
        return *child_orbits;
    }

    // Stop traversing orbits if we've reached a pre-determined object
    if let Some(stop_at) = &stop_at {
        if stop_at == parent {
            return 1;
        }
    }

    // Does the parent appear as a child in the orbit_map?
    let num_orbits = if let Some(parent_orbit) = orbit_map.iter().find(|(_, c)| c == parent) {
        1 + calc_num_orbits(parent_orbit, orbit_map, orbit_counts, stop_at)
    } else {
        // Parent of current orbit isn't orbiting anything (so COM)
        1
    };

    *orbit_counts
        .entry(child.clone())
        .and_modify(|e| *e = num_orbits)
        .or_insert(num_orbits)
}

pub fn part2(orbit_map: &ParsedInput) -> color_eyre::Result<usize> {
    let you_orbit_chain = get_orbit_chain("YOU", orbit_map);
    let san_orbit_chain = get_orbit_chain("SAN", orbit_map);

    for (you_idx, name) in you_orbit_chain.iter().enumerate() {
        if let Some(san_idx) = san_orbit_chain.iter().position(|e| e == name) {
            return Ok(you_idx + san_idx);
        }
    }

    Err(AdventError::LogicError(String::from("No intersection found between YOU and SAN")).into())
}

fn get_orbit_chain(from: &str, orbit_map: &ParsedInput) -> Vec<String> {
    let mut orbit_chain = Vec::new();

    let mut from = from;

    while let Some((parent, _)) = orbit_map.iter().find(|(_, child)| child == from) {
        orbit_chain.push(parent.clone());
        from = parent;
    }

    orbit_chain
}
