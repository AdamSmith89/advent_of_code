use advent_of_code::*;
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    /// Specific puzzle to run. Options are;
    /// - "latest" to run the latest completed test
    /// - "all" to run all completed puzzles
    /// - "year2022" or "year2022::day01" for a specific puzzle
    #[arg(short, long, default_value = "latest", verbatim_doc_comment)]
    puzzle: String,
}

fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;
    let args = Args::parse();

    let all_solutions = year2023();
    let mut run_solutions = Vec::new();

    if args.puzzle == "latest" {
        run_solutions.push(all_solutions.last().unwrap());
    } else if args.puzzle == "all" {
        run_solutions = all_solutions.iter().collect();
    } else {
        run_solutions = all_solutions
            .iter()
            .filter(|&solution| {
                if let Some((year, day)) = args.puzzle.split_once("::") {
                    solution.year == year && solution.day == day
                } else {
                    solution.year == args.puzzle
                }
            })
            .collect::<Vec<_>>();
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
    ]
}
