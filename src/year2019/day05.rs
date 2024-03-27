use itertools::Itertools;

use crate::error::AdventError;

use super::int_code_computer::IntCodeComputer;

type ParsedInput = Vec<i32>;

pub fn parse(input: &str) -> color_eyre::Result<ParsedInput> {
    Ok(input.split(',').map(|s| s.parse::<i32>()).try_collect()?)
}

pub fn part1(input: &ParsedInput) -> color_eyre::Result<i32> {
    let mut icc = IntCodeComputer::load(input.clone());
    icc.set_input(1);
    icc.run()?;

    let output = icc.get_output();

    output
        .last()
        .ok_or(AdventError::LogicError(String::from("No diagnostic output found")).into())
        .cloned()
}

pub fn part2(input: &ParsedInput) -> color_eyre::Result<i32> {
    let mut icc = IntCodeComputer::load(input.clone());
    icc.set_input(5);
    icc.run()?;

    let output = icc.get_output();

    output
        .last()
        .ok_or(AdventError::LogicError(String::from("No diagnostic output found")).into())
        .cloned()
}
