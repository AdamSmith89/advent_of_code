use advent_of_code::year2023::day05::*;

const PUZZLE_INPUT: &str = include_str!("../../input/year2023/day05.txt");
const EXAMPLE_INPUT: &str = "\
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
fn parse_example() {
    let parsed = parse(EXAMPLE_INPUT).expect("Error parsing example input");

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
fn part1_example() {
    let input = parse(EXAMPLE_INPUT).expect("Error parsing example input");
    let answer = part1(&input).expect("Error solving part 1");

    assert_eq!(answer, 35);
}

#[test]
fn part1_real() {
    let input = parse(PUZZLE_INPUT).expect("Error parsing example input");
    let answer = part1(&input).expect("Error solving part 1");

    assert_eq!(answer, 806029445);
}

#[test]
fn part2_example() {
    let input = parse(EXAMPLE_INPUT).expect("Error parsing example input");
    let answer = part2(&input).expect("Error solving part 2");

    assert_eq!(answer, 46);
}

#[test]
fn part2_real() {
    let input = parse(PUZZLE_INPUT).expect("Error parsing example input");
    let answer = part2(&input).expect("Error solving part 2");

    assert_eq!(answer, 59370572);
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
