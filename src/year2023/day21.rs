use crate::error::AdventError;

type ParsedInput = Map;

pub fn parse(input: &str) -> color_eyre::Result<ParsedInput> {
    Ok(Map::new())
}

pub fn part1(_: &ParsedInput) -> color_eyre::Result<u32> {
    Ok(0)
}

pub fn part2(_: &ParsedInput) -> color_eyre::Result<u32> {
    Ok(0)
}

#[derive(Debug, PartialEq, Eq)]
pub struct Map {
    pub steps_override: Option<u32>,
}

impl Map {
    fn new() -> Self {
        Self {
            steps_override: None,
        }
    }
}
