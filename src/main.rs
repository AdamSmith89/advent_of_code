use std::{
    iter::empty,
    time::{Duration, Instant},
};

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
        let result = (puzzle.solve)(puzzle.input)?;

        println!(
            "{}::{} - Part1 = {} in {}s, Part2 = {} in {}s, Parsed in {}s, Total in {}s",
            puzzle.year,
            puzzle.day,
            result.part1.answer,
            result.part1.time_s,
            result.part2.answer,
            result.part2.time_s,
            result.parse_time_s,
            result.total_time(),
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
    solve: fn(&str) -> color_eyre::Result<PuzzleResult>,
}

struct PuzzleResult {
    parse_time_s: f32,
    part1: PartResult,
    part2: PartResult,
}

impl PuzzleResult {
    fn total_time(&self) -> f32 {
        self.parse_time_s + self.part1.time_s + self.part2.time_s
    }
}

struct PartResult {
    answer: String,
    time_s: f32,
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

                let start = Instant::now();
                let input = parse(raw)?;
                let parse_time_s = start.elapsed().as_secs_f32();

                let start = Instant::now();
                let answer = part1(&input)?.to_string();
                let time_s = start.elapsed().as_secs_f32();
                let part1 = PartResult { answer, time_s };

                let start = Instant::now();
                let answer = part2(&input)?.to_string();
                let time_s = start.elapsed().as_secs_f32();
                let part2 = PartResult { answer, time_s };

                Ok(PuzzleResult {
                    parse_time_s,
                    part1,
                    part2,
                })
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
        solution!(year2023, day06),
        solution!(year2023, day07),
        solution!(year2023, day08),
    ]
}
