use itertools::Itertools;

use crate::error::AdventError;

type ParsedInput = Vec<u32>;

pub fn parse(input: &str) -> color_eyre::Result<ParsedInput> {
    Ok(input
        .lines()
        .map(|line| line.parse::<u32>())
        .try_collect()?)
}

// Fuel required to launch a given module is based on its mass.
// Specifically, to find the fuel required for a module,
// take its mass, divide by three, round down, and subtract 2.
pub fn part1(mass_values: &ParsedInput) -> color_eyre::Result<u32> {
    Ok(mass_values
        .iter()
        .map(|mass| f32::floor(*mass as f32 / 3f32) as u32 - 2)
        .sum())
}

// Fuel itself requires fuel just like a module
// Any mass that would require negative fuel should instead be treated as if it requires zero fuel
pub fn part2(mass_values: &ParsedInput) -> color_eyre::Result<u32> {
    Ok(mass_values
        .iter()
        .map(|mass| {
            let mut total_mod_fuel = 0;
            let mut mod_fuel = f32::floor(*mass as f32 / 3f32) as i32 - 2;

            while mod_fuel > 0 {
                total_mod_fuel += mod_fuel as u32;
                mod_fuel = f32::floor(mod_fuel as f32 / 3f32) as i32 - 2;
            }

            total_mod_fuel
        })
        .sum())
}
