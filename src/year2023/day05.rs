use itertools::Itertools;
use std::{num::ParseIntError, ops::Range, str::FromStr};

use crate::error::AdventError;

type ParsedInput = Almanac;

pub fn parse(input: &str) -> color_eyre::Result<ParsedInput> {
    let input = input.replace("\r\n", "\n");

    let (seeds, maps) = input
        .split_once("\n\n")
        .ok_or(AdventError::SplitOnce(input.to_string(), "\n\n".into()))?;

    // seeds: 79 14 55 13
    let (_, seeds) = seeds
        .split_once(':')
        .ok_or(AdventError::SplitOnce(seeds.into(), ':'.into()))?;

    let seeds_pt1 = seeds
        .split_ascii_whitespace()
        .map(|s| s.trim().parse::<u64>())
        .collect::<Result<Vec<_>, ParseIntError>>()?;

    let seeds_pt2 = seeds
        .split_ascii_whitespace()
        .tuples()
        .map(|(start, len)| Ok((start.trim().parse::<u64>()?, len.trim().parse::<u64>()?)))
        .collect::<Result<Vec<_>, ParseIntError>>()?;

    let seeds_pt2 = seeds_pt2
        .iter()
        .map(|(start, len)| (*start..(*start + *len)))
        .collect::<Vec<_>>();

    let maps = maps
        .split("\n\n")
        .map(Map::from_str)
        .collect::<Result<Vec<_>, AdventError>>()?;

    Ok(Almanac {
        seeds_pt1,
        seeds_pt2,
        maps,
    })
}

pub fn part1(almanac: &ParsedInput) -> color_eyre::Result<u64> {
    // What is the lowest location number that corresponds to any of the initial seed numberss

    let mut lowest_location = None;

    for seed in &almanac.seeds_pt1 {
        let mut seed = *seed;
        for map in &almanac.maps {
            seed = map.map_value(seed);
        }

        if lowest_location.is_none() {
            lowest_location = Some(seed);
        } else if let Some(cur_lowest) = lowest_location {
            lowest_location = Some(std::cmp::min(cur_lowest, seed));
        }
    }

    Ok(lowest_location.unwrap())
}

pub fn part2(almanac: &ParsedInput) -> color_eyre::Result<u64> {
    // seeds: line actually describes ranges of seed numbers.
    // What is the lowest location number that corresponds to any of the initial seed numberss

    let mut seed_ranges = almanac.seeds_pt2.clone();

    for map in &almanac.maps {
        seed_ranges = seed_ranges
            .iter()
            .flat_map(|range| map.map_range(range))
            .collect();
    }

    Ok(seed_ranges.iter().map(|range| range.start).min().unwrap())
}

pub struct Almanac {
    pub seeds_pt1: Vec<u64>,
    pub seeds_pt2: Vec<Range<u64>>,
    pub maps: Vec<Map>,
}

#[derive(Debug)]
pub struct Map {
    pub mappings: Vec<Mapping>,
}

impl FromStr for Map {
    type Err = AdventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // seed-to-soil map:
        // 50 98 2
        // 52 50 48
        // ...

        Ok(Self {
            mappings: s
                .lines()
                .skip(1)
                .map(Mapping::from_str)
                .collect::<Result<Vec<_>, Self::Err>>()?,
        })
    }
}

impl Map {
    pub fn map_value(&self, value: u64) -> u64 {
        let mapping = self
            .mappings
            .iter()
            .find(|mapping| mapping.source_range.contains(&value));

        if let Some(mapping) = mapping {
            let diff = value - mapping.source_range.start;
            mapping.dest_range.start + diff
        } else {
            value
        }
    }

    pub fn map_range(&self, range: &Range<u64>) -> Vec<Range<u64>> {
        let mut search = vec![range.clone()];
        let mut output = Vec::new();

        'outer: while let Some(cur_range) = search.pop() {
            for mapping in &self.mappings {
                let src_range = &mapping.source_range;
                let dst_range = &mapping.dest_range;

                // Range entirely enclosed in source mapping
                if cur_range.start >= src_range.start && cur_range.end <= src_range.end {
                    // source_start...cur_start...cur_end...source_end

                    let new_start = cur_range.start - src_range.start;
                    let new_end = src_range.end - cur_range.end;
                    output.push((dst_range.start + new_start)..(dst_range.end - new_end));
                    break 'outer;
                }
                // Range starts within source mapping
                else if src_range.contains(&cur_range.start) {
                    // source_start...cur_start...source_end...cur_end
                    let new_start = cur_range.start - src_range.start;
                    output.push((dst_range.start + new_start)..dst_range.end);
                    search.push(src_range.end..cur_range.end);
                    continue 'outer;
                }
                // Range ends within source mapping
                else if src_range.contains(&(cur_range.end - 1)) {
                    // cur_start...source_start...cur_end...source_end
                    let new_end = cur_range.end - src_range.start;
                    output.push(dst_range.start..(dst_range.start + new_end));
                    search.push(cur_range.start..src_range.start);
                    continue 'outer;
                }
                // Range encloses source mapping
                else if cur_range.start < src_range.start && cur_range.end > src_range.end {
                    // cur_start...source_start...source_end...cur_end
                    search.push(cur_range.start..src_range.start);
                    search.push(src_range.end..cur_range.end);
                    output.push(dst_range.clone());
                    continue 'outer;
                }
                // Range outside of this mapping
                else {
                    continue;
                }
            }

            // Must be outside all mappings to get here
            output.push(cur_range);
        }

        output
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Mapping {
    pub source_range: Range<u64>,
    pub dest_range: Range<u64>,
}

impl FromStr for Mapping {
    type Err = AdventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 50 98 2
        // dest_min source_min range_len
        let splits = s.split_ascii_whitespace().collect::<Vec<_>>();
        if splits.len() != 3 {
            return Err(AdventError::UnexpectedValue(3.to_string(), splits.len().to_string()));
        }

        let parse_u64 = |s: &str| -> Result<u64, AdventError> {
            s.parse().map_err(|err: ParseIntError| err.into())
        };

        let dest_min = parse_u64(splits[0])?;
        let source_min = parse_u64(splits[1])?;
        let range = parse_u64(splits[2])?;

        Ok(Self {
            source_range: source_min..(source_min + range),
            dest_range: dest_min..(dest_min + range),
        })
    }
}
