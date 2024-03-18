use std::string;

use itertools::Itertools;

use crate::error::AdventError;
use crate::year2019::int_code_computer::*;

type ParsedInput = Vec<u32>;

pub fn parse(input: &str) -> color_eyre::Result<ParsedInput> {
    Ok(input.split(',').map(|s| s.parse::<u32>()).try_collect()?)
}

// To do this, before running the program,
// replace position 1 with the value 12 and
// replace position 2 with the value 2.
// What value is left at position 0 after the program halts?
pub fn part1(code: &ParsedInput) -> color_eyre::Result<u32> {
    let mut code = code.clone();
    code[1] = 12;
    code[2] = 2;

    let mut icc = IntCodeComputer::new(code.clone());
    icc.run()?;
    Ok(icc.read(0)?)
}

pub fn part2(code: &ParsedInput) -> color_eyre::Result<u32> {
    let mut code = code.clone();

    for noun in 0..100 {
        for verb in 0..100 {
            code[1] = noun;
            code[2] = verb;

            let mut icc = IntCodeComputer::new(code.clone());
            icc.run()?;
            if icc.read(0)? == 19690720 {
                return Ok(100 * noun + verb);
            }
        }
    }

    Err(AdventError::LogicError(String::from(
        "Expected answer not found after iterating all noun and verb permutations",
    ))
    .into())
}
