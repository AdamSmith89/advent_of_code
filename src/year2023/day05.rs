use itertools::Itertools;
use std::{num::ParseIntError, ops::Range, str::FromStr};

type ParsedInput = Almanac;

pub fn parse(input: &str) -> color_eyre::Result<ParsedInput> {
    use Year2023Day05Error::*;

    let input = input.replace("\r\n", "\n");

    let (seeds, maps) = input
        .split_once("\n\n")
        .ok_or(ParseAlmanac("Failed to split seeds from maps".to_string()))?;

    // seeds: 79 14 55 13
    let (_, seeds) = seeds.split_once(':').ok_or(ParseAlmanac(
        format!("Failed to split seeds line: {seeds}").to_string(),
    ))?;

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
        .collect::<Result<Vec<_>, Year2023Day05Error>>()?;

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
    seeds_pt1: Vec<u64>,
    seeds_pt2: Vec<Range<u64>>,
    maps: Vec<Map>,
}

#[derive(Debug)]
struct Map {
    mappings: Vec<Mapping>,
}

impl FromStr for Map {
    type Err = Year2023Day05Error;

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
    fn map_value(&self, value: u64) -> u64 {
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

    fn map_range(&self, range: &Range<u64>) -> Vec<Range<u64>> {
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
struct Mapping {
    source_range: Range<u64>,
    dest_range: Range<u64>,
}

impl FromStr for Mapping {
    type Err = Year2023Day05Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 50 98 2
        // dest_min source_min range_len
        use Year2023Day05Error::*;

        let splits = s.split_ascii_whitespace().collect::<Vec<_>>();
        if splits.len() != 3 {
            return Err(ParseMapping(format!(
                "Found {} items in mapping, expected 3: {}",
                splits.len(),
                s
            )));
        }

        let parse_u64 = |s: &str| -> Result<u64, Year2023Day05Error> {
            s.parse().map_err(|err: std::num::ParseIntError| {
                ParseInt(format!("Parse error for '{s}': {}", err))
            })
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

#[derive(Debug, thiserror::Error, PartialEq)]
enum Year2023Day05Error {
    #[error("Failed to parse almanac: {0}")]
    ParseAlmanac(String),
    #[error("Failed to parse mapping: {0}")]
    ParseMapping(String),
    #[error("Failed to parse int: {0}")]
    ParseInt(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn parse_test() {
        let parsed = parse(EXAMPLE).expect("Error parsing example input");

        assert_eq!(parsed.seeds_pt1, vec![79, 14, 55, 13]);
        assert_eq!(parsed.seeds_pt2, vec![79..93, 55..68]);
        assert_eq!(
            parsed.maps[0].mappings[0],
            Mapping {
                source_range: 98..100,
                dest_range: 50..52
            }
        );
        //81 45 19
        assert_eq!(
            parsed.maps[4].mappings[1],
            Mapping {
                source_range: 45..64,
                dest_range: 81..100
            }
        );
    }

    #[test]
    fn part1_test() {
        let input = parse(EXAMPLE).expect("Error parsing example input");
        let answer = part1(&input).expect("Error solving part 1");

        assert_eq!(answer, 35);
    }

    #[test]
    fn part2_test() {
        let input = parse(EXAMPLE).expect("Error parsing example input");
        let answer = part2(&input).expect("Error solving part 2");

        assert_eq!(answer, 46);
    }

    #[test]
    fn map_map_seed_out_source_range() {
        let map = Map {
            mappings: vec![Mapping {
                source_range: 15..21,
                dest_range: 50..56,
            }],
        };

        assert_eq!(map.map_value(5), 5);
    }

    #[test]
    fn map_map_seed_in_source_range() {
        let map = Map {
            mappings: vec![Mapping {
                source_range: 15..21,
                dest_range: 50..56,
            }],
        };

        assert_eq!(map.map_value(15), 50);
        assert_eq!(map.map_value(17), 52);
        assert_eq!(map.map_value(20), 55);
    }

    #[test]
    fn map_map_seed_multiple() {
        let map = Map {
            mappings: vec![
                Mapping {
                    source_range: 15..21,
                    dest_range: 50..56,
                },
                Mapping {
                    source_range: 100..151,
                    dest_range: 0..51,
                },
            ],
        };

        assert_eq!(map.map_value(60), 60);
        assert_eq!(map.map_value(17), 52);
        assert_eq!(map.map_value(125), 25);
    }

    #[test]
    fn map_map_value_encloses_map() {
        let map = Map {
            mappings: vec![Mapping {
                source_range: 5..10,
                dest_range: 15..20,
            }],
        };

        // 3 4  5  6  7  8  9 10 11 12
        // 3 4 15 16 17 18 19 10 11 12

        let result = map.map_range(&(3..13));
        assert!(result.contains(&(3..5)));
        assert!(result.contains(&(15..20)));
        assert!(result.contains(&(10..13)));
    }

    #[test]
    fn map_map_value_within_map() {
        let map = Map {
            mappings: vec![Mapping {
                source_range: 5..10,
                dest_range: 15..20,
            }],
        };

        let result = map.map_range(&(7..10));
        assert!(result.contains(&(17..20)));
    }

    #[test]
    fn map_map_value_starts_in_map() {
        let map = Map {
            mappings: vec![Mapping {
                source_range: 5..10,
                dest_range: 15..20,
            }],
        };

        let result = map.map_range(&(7..16));
        assert!(result.contains(&(17..20)));
        assert!(result.contains(&(10..16)));
    }

    #[test]
    fn map_map_value_ends_in_map() {
        let map = Map {
            mappings: vec![Mapping {
                source_range: 5..10,
                dest_range: 15..20,
            }],
        };

        // 3 4  5  6  7  8
        // 3 4 15 16 17 18

        let result = map.map_range(&(3..9));
        assert!(result.contains(&(3..5)));
        assert!(result.contains(&(15..19)));
    }
}
