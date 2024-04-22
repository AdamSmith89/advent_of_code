use crate::error::AdventError;

use super::int_code_computer::{IcProgram, IntCodeComputer};

type ParsedInput = IcProgram;

pub fn parse(input: &str) -> color_eyre::Result<ParsedInput> {
    IntCodeComputer::parse_program(input)
}

pub fn part1(input: &ParsedInput) -> color_eyre::Result<i64> {
    let mut icc = IntCodeComputer::load(input.clone());
    icc.push_input(1);
    icc.run()?;

    icc.last_output()
        .ok_or(AdventError::LogicError(String::from("No diagnostic output found")).into())
        .cloned()
}

pub fn part2(input: &ParsedInput) -> color_eyre::Result<i64> {
    let mut icc = IntCodeComputer::load(input.clone());
    icc.push_input(5);
    icc.run()?;

    icc.last_output()
        .ok_or(AdventError::LogicError(String::from("No diagnostic output found")).into())
        .cloned()
}
