use crate::error::AdventError;

use super::int_code_computer::{IcProgram, IntCodeComputer};

type ParsedInput = IcProgram;

pub fn parse(input: &str) -> color_eyre::Result<ParsedInput> {
    IntCodeComputer::parse_program(input)
}

pub fn part1(code: &ParsedInput) -> color_eyre::Result<i64> {
    let mut icc = IntCodeComputer::load(code.clone());
    icc.push_input(1);
    icc.run()?;
    Ok(icc
        .last_output()
        .ok_or(AdventError::LogicError(String::from("No output from icc")))
        .cloned()?)
}

pub fn part2(code: &ParsedInput) -> color_eyre::Result<i64> {
    let mut icc = IntCodeComputer::load(code.clone());
    icc.push_input(2);
    icc.run()?;
    Ok(icc
        .last_output()
        .ok_or(AdventError::LogicError(String::from("No output from icc")))
        .cloned()?)
}
