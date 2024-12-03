use regex::Regex;

use crate::error::AdventError;

type ParsedInput = String;

pub fn parse(input: &str) -> color_eyre::Result<ParsedInput> {
    Ok(input.to_string())
}

pub fn part1(memory: &ParsedInput) -> color_eyre::Result<u32> {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)")?;

    let mut result = 0;
    for (_, [first, second]) in re.captures_iter(memory).map(|c| c.extract()) {
        let first = first.parse::<u32>().map_err(AdventError::ParseInt)?;
        let second = second.parse::<u32>().map_err(AdventError::ParseInt)?;

        result += first * second;
    }

    Ok(result)
}

pub fn part2(memory: &ParsedInput) -> color_eyre::Result<u32> {
    let mut instructions = Vec::new();

    let re = Regex::new(r"do\(\)")?;
    for m in re.find_iter(memory) {
        instructions.push((m.start(), "do"));
    }

    let re = Regex::new(r"don't\(\)")?;
    for m in re.find_iter(memory) {
        instructions.push((m.start(), "don't"));
    }

    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)")?;
    for m in re.find_iter(memory) {
        instructions.push((m.start(), m.as_str()));
    }

    instructions.sort_by(|(lhs, _), (rhs, _)| lhs.cmp(&rhs));

    let mut result = 0;
    let mut enabled = true;
    for (_, instruction) in instructions {
        match instruction {
            "do" => enabled = true,
            "don't" => enabled = false,
            mul => {
                if enabled {
                    let mut caps = re.captures_iter(mul);

                    let cap = caps
                        .next()
                        .ok_or(AdventError::NotFound("Digit capture group".to_string()))?;

                    let (_, [first, second]) = cap.extract();
                    let first = first.parse::<u32>().map_err(AdventError::ParseInt)?;
                    let second = second.parse::<u32>().map_err(AdventError::ParseInt)?;

                    result += first * second;
                }
            }
        }
    }

    Ok(result)
}
