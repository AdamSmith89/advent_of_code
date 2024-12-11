use std::collections::HashMap;

use itertools::Itertools;

type ParsedInput = HashMap<u128, u128>;

pub fn parse(input: &str) -> color_eyre::Result<ParsedInput> {
    Ok(input
        .split_whitespace()
        .map(|s| s.parse::<u128>())
        .map(|r| match r {
            Ok(stone) => Ok((stone, 1)),
            Err(e) => Err(e),
        })
        .try_collect()?)
}

pub fn part1(stones: &ParsedInput) -> color_eyre::Result<u128> {
    Ok(blink(stones, 25))
}

pub fn part2(stones: &ParsedInput) -> color_eyre::Result<u128> {
    Ok(blink(stones, 75))
}

fn blink(stones: &ParsedInput, num_blinks: u32) -> u128 {
    let mut stones = stones.clone();

    for _ in 0..num_blinks {
        let mut temp_stones = HashMap::new();

        for (&stone, &count) in &stones {
            let stone_replacements = change_stone(stone);

            for new_stone in stone_replacements {
                temp_stones
                    .entry(new_stone)
                    .and_modify(|v| *v += count)
                    .or_insert(count);
            }
        }

        stones = temp_stones;
    }

    stones.values().sum::<u128>()
}

fn change_stone(stone: u128) -> Vec<u128> {
    if stone == 0 {
        vec![1]
    } else {
        let num_digits = get_num_digits(stone);
        if num_digits % 2 == 0 {
            let split = split_num_at(stone, num_digits / 2);
            vec![split.0, split.1]
        } else {
            vec![stone * 2024]
        }
    }
}

fn get_num_digits(num: u128) -> u32 {
    let mut num = num;

    // Count the number of digits using division
    let mut count = 0;
    while num > 0 {
        num /= 10;
        count += 1;
    }

    count
}

fn split_num_at(num: u128, mid: u32) -> (u128, u128) {
    let divisor = 10_u128.pow(mid);

    let lhs = num / divisor; // Left part
    let rhs = num % divisor; // Right part

    (lhs, rhs)
}
