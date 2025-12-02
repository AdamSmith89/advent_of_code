type ParsedInput = Vec<i32>;

pub fn parse(input: &str) -> color_eyre::Result<ParsedInput> {
    input
        .lines()
        .map(|line| {
            let (dir, num) = line.split_at(1);

            if dir == "L" {
                Ok(-num.parse::<i32>()?)
            } else if dir == "R" {
                Ok(num.parse::<i32>()?)
            } else {
                Err(color_eyre::eyre::eyre!("Invalid direction"))
            }
        })
        .collect::<Result<Vec<_>, _>>()
}

pub fn part1(rotations: &ParsedInput) -> color_eyre::Result<u32> {
    let mut dial = 50;
    let max = 100;

    Ok(rotations
        .iter()
        .filter(|&&rotation| {
            if rotation > 0 {
                dial = (dial + rotation) % max;
            } else {
                dial = (dial + max + rotation) % max;
            }

            dial == 0
        })
        .count() as u32)
}

pub fn part2(rotations: &ParsedInput) -> color_eyre::Result<i32> {
    let mut dial = 50;
    let max = 100;

    Ok(rotations
        .iter()
        .map(|&rotation| {
            let mut zeroes = rotation.abs() / max;
            let rotation = rotation % max;

            if rotation > 0 {
                if dial != 0 && max - rotation <= dial {
                    zeroes += 1;
                }
                
                dial = (dial + rotation) % max;
            } else {
                if dial != 0 && rotation.abs() >= dial {
                    zeroes += 1;
                }

                dial = (dial + max + rotation) % max;
            }

            zeroes
        })
        .sum())
}
