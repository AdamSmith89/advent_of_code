use itertools::Itertools;

use crate::error::AdventError;

type ParsedInput = ([u32; 6], [u32; 6]);

pub fn parse(input: &str) -> color_eyre::Result<ParsedInput> {
    let (start, end) = input
        .split_once('-')
        .ok_or(AdventError::SplitOnce(input.to_string(), '-'.to_string()))?;

    Ok((
        explode_number(&start.parse::<u32>()?),
        explode_number(&end.parse::<u32>()?),
    ))
}

// It is a six-digit number.
// The value is within the range given in your puzzle input.
// Two adjacent digits are the same (like 22 in 122345).
// Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679).
// How many different passwords within the range given in your puzzle input meet these criteria?
pub fn part1((start, end): &ParsedInput) -> color_eyre::Result<u32> {
    let mut valid_pwds = 0;

    let mut next_num = next_non_dec_number(start);
    while next_num <= *end {
        if has_adjacent_digits(&next_num) {
            valid_pwds += 1;
        }

        next_num = next_non_dec_number(&next_num);
    }

    Ok(valid_pwds)
}

// the two adjacent matching digits are not part of a larger group of matching digits.
// 111122 meets the criteria (even though 1 is repeated more than twice, it still contains a double 22).
pub fn part2((start, end): &ParsedInput) -> color_eyre::Result<u32> {
    let mut valid_pwds = 0;

    let mut next_num = next_non_dec_number(start);
    while next_num <= *end {
        if has_two_adjacent_digits(&next_num) {
            valid_pwds += 1;
        }

        next_num = next_non_dec_number(&next_num);
    }

    Ok(valid_pwds)
}

fn explode_number(num: &u32) -> [u32; 6] {
    [
        num / 100000,
        (num / 10000) % 10,
        (num / 1000) % 10,
        (num / 100) % 10,
        (num / 10) % 10,
        num % 10,
    ]
}

fn next_non_dec_number(digits: &[u32; 6]) -> [u32; 6] {
    let mut next_num = *digits;

    for (idx, digit) in digits.iter().enumerate() {
        if idx == digits.len() - 1 {
            if next_num[idx] == 9 {
                if let Some((non_nine_idx, _)) = digits
                    .iter()
                    .enumerate()
                    .rev()
                    .find(|&(_, digit)| *digit != 9)
                {
                    let fill = next_num[non_nine_idx] + 1;
                    next_num[non_nine_idx..].fill(fill);
                } else {
                    panic!("All digits are unexpectedly 9!")
                }
            } else {
                next_num[idx] += 1;
            }

            break;
        }

        if *digit > digits[idx + 1] {
            next_num[idx + 1..].fill(*digit);
            break;
        }
    }

    next_num
}

fn has_adjacent_digits(digits: &[u32; 6]) -> bool {
    digits.iter().dedup().collect_vec().len() < digits.len()
}

fn has_two_adjacent_digits(digits: &[u32; 6]) -> bool {
    let adj_digits = digits.iter().dedup_with_count().collect_vec();
    adj_digits.iter().any(|(count, _)| *count == 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_non_dec_number_last_digit() {
        assert_eq!([1, 2, 3, 4, 5, 7], next_non_dec_number(&[1, 2, 3, 4, 5, 6]));
    }

    #[test]
    fn next_non_dec_number_last_digit_9() {
        assert_eq!([1, 2, 3, 4, 6, 6], next_non_dec_number(&[1, 2, 3, 4, 5, 9]));
    }

    #[test]
    fn next_non_dec_number_many_9s() {
        assert_eq!([1, 3, 3, 3, 3, 3], next_non_dec_number(&[1, 2, 9, 9, 9, 9]));
    }

    #[test]
    fn next_non_dec_number_penultimate_digit() {
        assert_eq!([1, 2, 3, 4, 4, 4], next_non_dec_number(&[1, 2, 3, 4, 3, 6]));
    }

    #[test]
    fn next_non_dec_number_first_digit() {
        assert_eq!([7, 7, 7, 7, 7, 7], next_non_dec_number(&[7, 2, 3, 4, 3, 6]));
    }

    #[test]
    fn has_adjacent_digits_none() {
        assert!(!has_adjacent_digits(&[1, 2, 3, 4, 5, 6]));
    }

    #[test]
    fn has_adjacent_digits_single() {
        assert!(has_adjacent_digits(&[1, 2, 2, 4, 5, 6]));
    }

    #[test]
    fn has_adjacent_digits_triplet() {
        assert!(has_adjacent_digits(&[1, 2, 2, 2, 5, 6]));
    }

    #[test]
    fn has_adjacent_digits_multiple() {
        assert!(has_adjacent_digits(&[1, 2, 2, 4, 5, 5]));
    }

    #[test]
    fn has_two_adjacent_digits_none() {
        assert!(!has_two_adjacent_digits(&[1, 2, 3, 4, 5, 6]));
    }

    #[test]
    fn has_two_adjacent_digits_too_many() {
        assert!(!has_two_adjacent_digits(&[1, 1, 1, 1, 5, 6]));
    }

    #[test]
    fn has_two_adjacent_digits_single() {
        assert!(has_two_adjacent_digits(&[1, 1, 3, 4, 5, 6]));
    }

    #[test]
    fn has_two_adjacent_digits_multiple() {
        assert!(has_two_adjacent_digits(&[1, 1, 2, 2, 5, 6]));
    }

    #[test]
    fn has_two_adjacent_digits_too_many_and_single() {
        assert!(has_two_adjacent_digits(&[1, 1, 1, 1, 2, 2]));
    }
}
