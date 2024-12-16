use std::{iter::empty, time::Instant};

use advent_of_code::*;
use clap::Parser;
use colored::{ColoredString, Colorize};
use simple_logger::SimpleLogger;

#[derive(Parser, Debug)]
struct Args {
    /// Run specific puzzle(s), e.g "year2023" or "year2023::day01"
    #[arg(short, long)]
    puzzle: Option<String>,

    /// Enable debug output
    #[arg(short, long)]
    debug: bool,
}

fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;
    let (year, day, debug_enabled) = parse_args();

    let puzzles = get_puzzles(year, day);
    let mut total_time = 0f32;

    if debug_enabled {
        SimpleLogger::new()
            .with_level(log::LevelFilter::Debug)
            .init()?;
    } else {
        print_header();
    }

    for puzzle in &puzzles {
        let result = (puzzle.solve)(puzzle.input)?;

        if !debug_enabled {
            total_time += result.total_time();
            print_puzzle_result(&puzzle.year, &puzzle.day, result);
        }
    }

    if !debug_enabled && puzzles.len() > 1 {
        println!();
        println!("Solved {} puzzles in {total_time}s", puzzles.len());
    }

    Ok(())
}

fn parse_args() -> (Option<String>, Option<String>, bool) {
    let args = Args::parse();

    if let Some(puzzle) = args.puzzle {
        if let Some((year, day)) = puzzle.split_once("::") {
            (Some(year.into()), Some(day.into()), args.debug)
        } else {
            (Some(puzzle), None, args.debug)
        }
    } else {
        (None, None, args.debug)
    }
}

fn get_puzzles(year: Option<String>, day: Option<String>) -> Vec<Puzzle> {
    empty()
        .chain(year2019())
        .chain(year2023())
        .chain(year2024())
        .filter(|puzzle| year.as_ref().map_or(true, |year| *year == puzzle.year))
        .filter(|puzzle| day.as_ref().map_or(true, |day| *day == puzzle.day))
        .collect::<Vec<_>>()
}

fn print_header() {
    println!(
        "{:^10}┃{:^7}┃{:^20}┃{:^20}┃{:^18}┃{:^18}┃{:^18}┃{:^15}",
        "Year".bold(),
        "Day".bold(),
        "Part 1".bold(),
        "Part 2".bold(),
        "Parse Time(s)".bold(),
        "Part 1 Time(s)".bold(),
        "Part 2 Time(s)".bold(),
        "Total Time(s)".bold()
    );
    println!(
        "{:━^10}╋{:━^7}╋{:━^20}╋{:━^20}╋{:━^18}╋{:━^18}╋{:━^18}╋{:━^15}",
        "━", "━", "━", "━", "━", "━", "━", "━"
    );
}

fn print_puzzle_result(year: &String, day: &String, result: PuzzleResult) {
    let part1_time = colorize_time(result.part1.time_s);
    let part2_time = colorize_time(result.part2.time_s);
    let parse_time = colorize_time(result.parse_time_s);
    let total_time = colorize_time(result.total_time());

    println!(
        "{:<10}┃ {:<6}┃ {:<19}┃ {:<19}┃ {:<17}┃ {:<17}┃ {:<17}┃ {:<14}",
        year,
        day,
        result.part1.answer,
        result.part2.answer,
        parse_time,
        part1_time,
        part2_time,
        total_time,
    );
}

fn colorize_time(time: f32) -> ColoredString {
    if time < 0.5 {
        format!("{}", time).green()
    } else if time < 1.0 {
        format!("{}", time).blue()
    } else {
        format!("{}", time).red()
    }
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

fn year2019() -> Vec<Puzzle> {
    vec![
        puzzle!(year2019, day01),
        puzzle!(year2019, day02),
        puzzle!(year2019, day03),
        puzzle!(year2019, day04),
        puzzle!(year2019, day05),
        puzzle!(year2019, day06),
        puzzle!(year2019, day07),
        puzzle!(year2019, day08),
        puzzle!(year2019, day09),
        puzzle!(year2019, day10),
        puzzle!(year2019, day11),
    ]
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
        puzzle!(year2023, day10),
        puzzle!(year2023, day11),
        puzzle!(year2023, day12),
        puzzle!(year2023, day13),
        puzzle!(year2023, day14),
        puzzle!(year2023, day15),
        puzzle!(year2023, day16),
        puzzle!(year2023, day17),
        puzzle!(year2023, day18),
        puzzle!(year2023, day19),
        puzzle!(year2023, day20),
        //puzzle!(year2023, day21),
        puzzle!(year2023, day22),
        //puzzle!(year2023, day23),
    ]
}

fn year2024() -> Vec<Puzzle> {
    vec![
        puzzle!(year2024, day01),
        puzzle!(year2024, day02),
        puzzle!(year2024, day03),
        puzzle!(year2024, day04),
        puzzle!(year2024, day05),
        puzzle!(year2024, day06),
        puzzle!(year2024, day07),
        puzzle!(year2024, day08),
        puzzle!(year2024, day09),
        puzzle!(year2024, day10),
        puzzle!(year2024, day11),
        puzzle!(year2024, day12),
        puzzle!(year2024, day13),
        puzzle!(year2024, day14),
        puzzle!(year2024, day15),
        // NEXT
    ]
}
