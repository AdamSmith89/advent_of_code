use std::{iter::empty, time::Instant};

use advent_of_code::*;
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    /// Run specific puzzle(s), e.g "year2023" or "year2023::day01"
    #[arg(short, long)]
    puzzle: Option<String>,
}

fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;
    let (year, day) = parse_args();

    let puzzles = get_puzzles(year, day);
    let mut total_time = 0f32;

    for puzzle in &puzzles {
        let result = (puzzle.solve)(puzzle.input)?;
        total_time += result.total_time();

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

    if puzzles.len() > 1 {
        println!("Puzzles solved = {} in {total_time}s", puzzles.len());
    }

    Ok(())
}

fn parse_args() -> (Option<String>, Option<String>) {
    let args = Args::parse();

    if let Some(puzzle) = args.puzzle {
        if let Some((year, day)) = puzzle.split_once("::") {
            (Some(year.into()), Some(day.into()))
        } else {
            (Some(puzzle), None)
        }
    } else {
        (None, None)
    }
}

fn get_puzzles(year: Option<String>, day: Option<String>) -> Vec<Puzzle> {
    empty()
        .chain(year2023())
        .filter(|puzzle| year.as_ref().map_or(true, |year| *year == puzzle.year))
        .filter(|puzzle| day.as_ref().map_or(true, |day| *day == puzzle.day))
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

macro_rules! puzzle {
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

                let (input, parse_time_s) = timed_fn(|| parse(raw))?;

                let (answer, time_s) = timed_fn(|| part1(&input))?;
                let part1 = PartResult {
                    answer: answer.to_string(),
                    time_s,
                };

                let (answer, time_s) = timed_fn(|| part2(&input))?;
                let part2 = PartResult {
                    answer: answer.to_string(),
                    time_s,
                };

                Ok(PuzzleResult {
                    parse_time_s,
                    part1,
                    part2,
                })
            },
        }
    };
}

fn timed_fn<F, T>(f: F) -> color_eyre::Result<(T, f32)>
where
    F: Fn() -> color_eyre::Result<T>,
{
    let start = Instant::now();
    let result = f()?;
    let elapsed_s = start.elapsed().as_secs_f32();

    Ok((result, elapsed_s))
}

fn year2023() -> Vec<Puzzle> {
    vec![
        puzzle!(year2023, day01),
        puzzle!(year2023, day02),
        puzzle!(year2023, day03),
        puzzle!(year2023, day04),
        puzzle!(year2023, day05),
        puzzle!(year2023, day06),
        puzzle!(year2023, day07),
        puzzle!(year2023, day08),
        puzzle!(year2023, day09),
        //puzzle!(year2023, day10),
        puzzle!(year2023, day11),
        puzzle!(year2023, day12),
        puzzle!(year2023, day13),
        puzzle!(year2023, day14),
        puzzle!(year2023, day15),
        puzzle!(year2023, day16),
        puzzle!(year2023, day17),
        puzzle!(year2023, day18),
        // NEXT
    ]
}
