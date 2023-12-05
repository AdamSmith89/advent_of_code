use std::iter::empty;

use advent_of_code::*;
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    /// Run specific puzzle, e.g "year2022" or "year2022::day01"
    #[arg(short, long, verbatim_doc_comment)]
    puzzle: Option<String>,
}

fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;

    let args = Args::parse();
    let (year, day) = parse_args(&args);

    for puzzle in get_puzzles(year, day) {
        let (part1, part2) = (puzzle.solve)(puzzle.input)?;

        println!(
            "{}::{} - Part1 = {part1}, Part2 = {part2}",
            puzzle.year, puzzle.day
        );
    }

    Ok(())
}

fn parse_args<'a>(args: &'a Args) -> (Option<&'a str>, Option<&'a str>) {
    if let Some(puzzle) = &args.puzzle {
        if let Some((year, day)) = puzzle.split_once("::") {
            (Some(year), Some(day))
        } else {
            (Some(puzzle.as_str()), None)
        }
    } else {
        (None, None)
    }
}

fn get_puzzles(year: Option<&str>, day: Option<&str>) -> Vec<Puzzle> {
    empty()
        .chain(year2023())
        .filter(|puzzle| year.map_or(true, |year| year == puzzle.year))
        .filter(|puzzle| day.map_or(true, |day| day == puzzle.day))
        .collect::<Vec<_>>()
}

struct Puzzle {
    year: String,
    day: String,
    input: &'static str,
    solve: fn(&str) -> color_eyre::Result<(String, String)>,
}

macro_rules! solution {
    ($year:tt, $day:tt) => {
        Puzzle {
            year: stringify!($year).to_string(),
            day: stringify!($day).to_string(),
            input: include_str!(concat![
                "../input/",
                stringify!($year),
                "/",
                stringify!($day),
                ".txt"
            ]),
            solve: |raw: &str| {
                use $year::$day::*;
                let input = parse(raw)?;
                let part1 = part1(&input)?.to_string();
                let part2 = part2(&input)?.to_string();
                Ok((part1, part2))
            },
        }
    };
}

fn year2023() -> Vec<Puzzle> {
    vec![
        solution!(year2023, day01),
        solution!(year2023, day02),
        solution!(year2023, day03),
        solution!(year2023, day04),
        solution!(year2023, day05),
    ]
}
