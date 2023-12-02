use clap::Parser;
use advent_of_code::*;

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
    let _args = Args::parse();

    let solutions = year2023();

    for Solution { input, solve } in solutions {
        let (part1, part2) = solve(input)?;

        println!("Part 1 = {part1}");
        println!("Part 2 = {part2}");
    }

    Ok(())
}

struct Solution {
    input: &'static str,
    solve: fn(&str) -> color_eyre::Result<(String, String)>,
}

macro_rules! solution {
    ($year:tt, $day:tt) => {
        Solution {
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
    ]
}
