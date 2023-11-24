use advent_of_code::year2022;
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

use year2022::day01::*;
use advent_of_code::solver::*;

fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;

    let args = Args::parse();
    println!("{args:?}");

    let mut s = Year2022Day01{};
    solve_puzzle(&mut s, "test")?;

    Ok(())
}

fn solve_puzzle<S: Solver>(solver: &mut S, input: &str) -> color_eyre::eyre::Result<()>
where
    <S as Solver>::Output: std::fmt::Display,
{
    let parsed_input = S::parse(input)?;

    let part1 = solver.part1(&parsed_input)?;
    println!("  Part1 = {part1}");
    
    let part2 = solver.part2(&parsed_input)?;
    println!("  Part2 = {part2}");

    Ok(())
}
