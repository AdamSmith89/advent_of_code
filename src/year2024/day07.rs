use itertools::Itertools;

use crate::error::AdventError;

type ParsedInput = Vec<Equation>;

pub fn parse(input: &str) -> color_eyre::Result<ParsedInput> {
    Ok(input.lines().map(Equation::try_from).try_collect()?)
}

pub fn part1(equations: &ParsedInput) -> color_eyre::Result<u128> {
    let answer = equations
        .iter()
        .filter(|eq| is_solvable(eq.target, &eq.numbers, false))
        .map(|eq| eq.target)
        .sum();

    Ok(answer)
}

pub fn part2(equations: &ParsedInput) -> color_eyre::Result<u128> {
    let answer = equations
        .iter()
        .filter(|eq| is_solvable(eq.target, &eq.numbers, true))
        .map(|eq| eq.target)
        .sum();

    Ok(answer)
}

fn is_solvable(target: u128, numbers: &[u128], test_concat: bool) -> bool {
    if numbers.len() > 2 {
        test_op(target, numbers, std::ops::Add::add, test_concat)
            || test_op(target, numbers, std::ops::Mul::mul, test_concat)
            || (test_concat && test_op(target, numbers, concat_nums, test_concat))
    } else {
        (numbers[0] + numbers[1] == target)
            || (numbers[0] * numbers[1] == target)
            || (test_concat && concat_nums(numbers[0], numbers[1]) == target)
    }
}

fn test_op<Op>(target: u128, numbers: &[u128], op: Op, test_concat: bool) -> bool
where
    Op: Fn(u128, u128) -> u128,
{
    let res = op(numbers[0], numbers[1]);
    if res <= target {
        let numbers = [&[res], &numbers[2..]].concat();
        is_solvable(target, &numbers, test_concat)
    } else {
        false
    }
}

fn concat_nums(a: u128, b: u128) -> u128 {
    a * 10u128.pow(b.ilog10() + 1) + b
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Equation {
    target: u128,
    numbers: Vec<u128>,
}

impl Equation {
    pub fn new(target: u128, numbers: Vec<u128>) -> Self {
        Self { target, numbers }
    }
}

impl TryFrom<&str> for Equation {
    type Error = AdventError;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let (target, numbers) = line
            .split_once(':')
            .ok_or(AdventError::SplitOnce(line.to_string(), ':'.to_string()))?;

        let target = target.parse::<u128>().map_err(AdventError::ParseInt)?;

        let numbers: Vec<u128> = numbers
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|num| num.parse::<u128>().map_err(AdventError::ParseInt))
            .try_collect()?;

        Ok(Equation { target, numbers })
    }
}
