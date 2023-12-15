use itertools::Itertools;

use crate::error::AdventError;

type ParsedInput = Vec<String>;

pub fn parse(input: &str) -> color_eyre::Result<ParsedInput> {
    Ok(input.split(',').map_into().collect_vec())
}

pub fn part1(steps: &ParsedInput) -> color_eyre::Result<u32> {
    // Determine the ASCII code for the current character of the string.
    // Increase the current value by the ASCII code you just determined.
    // Set the current value to itself multiplied by 17.
    // Set the current value to the remainder of dividing itself by 256.

    let mut total: u32 = 0;
    for step in steps {
        print!("Processing {step}...");

        let mut cur_value = 0;

        for ch in step.chars() {
            cur_value += ch as u32;
            cur_value *= 17;
            cur_value %= 256;
            
            //println!("  {ch} == {}", ch as u32);
        }
        
        println!("{cur_value}");
        total += cur_value;
    }

    println!("{total}");

    Ok(0)
}

pub fn part2(_: &ParsedInput) -> color_eyre::Result<u32> {
    Ok(0)
}
