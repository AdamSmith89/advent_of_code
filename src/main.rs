use advent_of_code::*;
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    /// Run all solved puzzles
    #[arg(short, long, action, conflicts_with("puzzle"))]
    all: bool,

    /// Run specific puzzle, e.g "year2022" or "year2022::day01"
    #[arg(short, long, verbatim_doc_comment, conflicts_with("all"))]
    puzzle: Option<String>,
}

fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;
    let args = Args::parse();

    let all_solutions = year2023();
    let mut run_solutions = Vec::new();

    if args.all {
        run_solutions = all_solutions.iter().collect();
    } else if let Some(puzzle) = args.puzzle {
        run_solutions = all_solutions
            .iter()
            .filter(|&solution| {
                if let Some((year, day)) = puzzle.split_once("::") {
                    solution.year == year && solution.day == day
                } else {
                    solution.year == puzzle
                }
            })
            .collect::<Vec<_>>();
    } else {
        run_solutions.push(all_solutions.last().unwrap());
    }

    for solution in run_solutions {
        let (part1, part2) = (solution.solve)(solution.input)?;

        println!(
            "{}::{} - Part1 = {part1}, Part2 = {part2}",
            solution.year, solution.day
        );
    }

    Ok(())
}

struct Solution {
    year: String,
    day: String,
    input: &'static str,
    solve: fn(&str) -> color_eyre::Result<(String, String)>,
}

macro_rules! solution {
    ($year:tt, $day:tt) => {
        Solution {
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

fn year2023() -> Vec<Solution> {
    vec![
        solution!(year2023, day01),
        solution!(year2023, day02),
        solution!(year2023, day03),
        solution!(year2023, day04),
        solution!(year2023, day05),
    ]
}
