use itertools::Itertools;

use crate::error::AdventError;
use crate::util::grid::Direction;

type ParsedInput = Input;

pub fn parse(input: &str) -> color_eyre::Result<ParsedInput> {
    let parts: Vec<(DigStep, DigStep)> = input
        .lines()
        .map(|line| {
            let splits = line.split_ascii_whitespace().collect_vec();

            let dir = match splits[0] {
                "U" => Direction::North,
                "R" => Direction::East,
                "D" => Direction::South,
                "L" => Direction::West,
                _ => return Err(AdventError::UnknownPattern(splits[0].into())),
            };

            let part1 = DigStep {
                dir,
                size: splits[1].parse()?,
            };

            let colour = splits[2].trim_start_matches("(#").trim_end_matches(")");

            let (size, dir) = colour.split_at(5);
            let dir = match dir {
                "3" => Direction::North,
                "0" => Direction::East,
                "1" => Direction::South,
                "2" => Direction::West,
                _ => return Err(AdventError::UnknownPattern(dir.into())),
            };
            let size = i64::from_str_radix(size, 16)?;

            let part2 = DigStep { dir, size };

            Ok((part1, part2))
        })
        .try_collect()?;

    let (part1, part2) = parts.into_iter().unzip();

    Ok(Input { part1, part2 })
}

pub fn part1(input: &ParsedInput) -> color_eyre::Result<i64> {
    Ok(solve(&input.part1))
}

pub fn part2(input: &ParsedInput) -> color_eyre::Result<i64> {
    Ok(solve(&input.part2))
}

fn solve(steps: &Vec<DigStep>) -> i64 {
    let mut start = Point(0, 0);
    let mut end;
    let mut perimeter = 0i64;
    let mut area = 0;

    for step in steps {
        // Perimeter is the length of all the sides
        perimeter += step.size;

        end = calc_end(&start, step.size, step.dir);
        area += calc_determinant(&start, &end);

        start = end;
    }

    // Pick's Theorem
    area / 2 + perimeter / 2 + 1
}

// Shoelace Theorem
// Area = 0.5 * |(x1*y2 - x2*y1) + (x2*y3 - x3*y2) + ... + (xn*y1 - x1*yn)|
// Determinant = (xn*y1 - x1*yn)
fn calc_determinant(start: &Point, end: &Point) -> i64 {
    (start.0 * end.1) as i64 - (start.1 * end.0) as i64
}

fn calc_end(start: &Point, size: i64, dir: Direction) -> Point {
    let mut end = start.clone();
    match dir {
        Direction::North => end.1 -= size,
        Direction::East => end.0 += size,
        Direction::South => end.1 += size,
        Direction::West => end.0 -= size,
    };

    end
}

#[derive(Debug, Eq, PartialEq)]
pub struct Input {
    pub part1: Vec<DigStep>,
    pub part2: Vec<DigStep>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct DigStep {
    pub dir: Direction,
    pub size: i64,
}

impl DigStep {
    pub fn new(dir: Direction, size: i64) -> Self {
        Self { dir, size }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Point(i64, i64);

impl From<(i64, i64)> for Point {
    fn from(value: (i64, i64)) -> Self {
        Self(value.0, value.1)
    }
}

impl From<Point> for (i64, i64) {
    fn from(value: Point) -> Self {
        (value.0, value.1)
    }
}
