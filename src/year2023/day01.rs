use std::collections::HashMap;

pub fn parse(input: &str) -> color_eyre::Result<Vec<&str>> {
    Ok(input.lines().collect())
}

pub fn part1(input: &Vec<&str>) -> color_eyre::Result<u32> {
    // On each line, the calibration value can be found by combining
    // the first digit and the last digit (in that order) to form a single two-digit number.
    // What is the sum of all of the calibration values?

    let mut sum = 0;

    for line in input {
        let first = line
            .chars()
            .find(|ch| ch.is_ascii_digit())
            .expect("Failed to find first digit")
            .to_digit(10)
            .unwrap();
        let last = line
            .chars()
            .rfind(|ch| ch.is_ascii_digit())
            .expect("Failed to find last digit")
            .to_digit(10)
            .unwrap();

        sum += 10 * first + last;
    }

    Ok(sum)
}

pub fn part2(input: &Vec<&str>) -> color_eyre::Result<u32> {
    // It looks like some of the digits are actually spelled out with letters:
    // one, two, three, four, five, six, seven, eight, and nine also count as valid "digits".

    let numbers = vec![
        "0".to_string(),
        "1".to_string(),
        "2".to_string(),
        "3".to_string(),
        "4".to_string(),
        "5".to_string(),
        "6".to_string(),
        "7".to_string(),
        "8".to_string(),
        "9".to_string(),
        "one".to_string(),
        "two".to_string(),
        "three".to_string(),
        "four".to_string(),
        "five".to_string(),
        "six".to_string(),
        "seven".to_string(),
        "eight".to_string(),
        "nine".to_string(),
    ];

    let mut sum = 0;

    for line in input {
        let first = find_first_of(&line, &numbers)
            .expect(format!("Failed to find first number in \"{line}\"").as_str());
        let last = find_last_of(&line, &numbers)
            .expect(format!("Failed to find last number in \"{line}\"").as_str());

        sum += 10 * to_digit(&first) + to_digit(&last);
    }

    Ok(sum)
}

fn to_digit(s: &String) -> u32 {
    let map = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);

    let ch = if s.chars().next().unwrap().is_ascii_digit() {
        s.chars().next().unwrap()
    } else {
        map.get(s.as_str()).expect("Failed to find digit").clone()
    };

    ch.to_digit(10).unwrap()
}

fn find_first_of(s: &str, terms: &[String]) -> Option<String> {
    let mut possibles = terms.to_vec();
    let mut cur_search = String::new();
    let mut iter = s.chars().peekable();

    while let Some(ch) = iter.peek() {
        cur_search.push(*ch);
        possibles.retain(|s| s.starts_with(&cur_search));

        if possibles.is_empty() {
            possibles = terms.to_vec();

            while !cur_search.is_empty() {
                cur_search.remove(0);
                possibles.retain(|s| s.starts_with(&cur_search));

                if possibles.is_empty() {
                    possibles = terms.to_vec();
                } else {
                    break;
                }
            }
        }

        if possibles.len() == 1 && cur_search == possibles[0] {
            return Some(cur_search);
        }

        iter.next();
    }

    None
}

fn find_last_of(s: &str, terms: &[String]) -> Option<String> {
    let s: String = s.chars().rev().collect();
    let terms: Vec<String> = terms.iter().map(|s| s.chars().rev().collect()).collect();

    if let Some(result) = find_first_of(s.as_str(), terms.as_slice()) {
        Some(result.chars().rev().collect())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PART1_EXAMPLE: &str = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    #[test]
    fn part1_test() {
        let input = parse(PART1_EXAMPLE).expect("Failed to parse test input");

        let answer = part1(&input).expect("Failed to calculate answer");
        assert_eq!(answer, 142);
    }

    const PART2_EXAMPLE: &str = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn part2_test() {
        let input = parse(PART2_EXAMPLE).expect("Failed to parse test input");

        let answer = part2(&input).expect("Failed to calculate answer");
        assert_eq!(answer, 281);
    }

    const TROUBLESOME: &str = "\
1hggcqcstgpmg26lzxtltcgg
vfzmncfonexxkzlcstqhxvtwoplsglsix1kpkssfz
254zhtrlvpfs";

    #[test]
    fn troublesome_test() {
        let input = parse(TROUBLESOME).expect("Failed to parse test input");

        let answer = part2(&input).expect("Failed to calculate answer");
        assert_eq!(answer, 16 + 11 + 24);
    }
}
