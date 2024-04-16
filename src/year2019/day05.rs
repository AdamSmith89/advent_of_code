use crate::error::AdventError;

use super::int_code_computer::IntCodeComputer;

type ParsedInput = Vec<i64>;

pub fn parse(input: &str) -> color_eyre::Result<ParsedInput> {
    IntCodeComputer::parse_program(input)
}

pub fn part1(input: &ParsedInput) -> color_eyre::Result<i64> {
    let mut icc = IntCodeComputer::load(input.clone());
    icc.push_input(1);
    icc.run()?;

    let output = icc.get_output();

    output
        .last()
        .ok_or(AdventError::LogicError(String::from("No diagnostic output found")).into())
        .cloned()
}

pub fn part2(input: &ParsedInput) -> color_eyre::Result<i64> {
    let mut icc = IntCodeComputer::load(input.clone());
    icc.push_input(5);
    icc.run()?;

    let output = icc.get_output();

    output
        .last()
        .ok_or(AdventError::LogicError(String::from("No diagnostic output found")).into())
        .cloned()
}
