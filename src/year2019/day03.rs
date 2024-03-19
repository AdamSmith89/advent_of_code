use std::collections::HashSet;

use itertools::Itertools;

use crate::error::AdventError;

type ParsedInput = (Vec<(i32, i32)>, Vec<(i32, i32)>, Vec<(i32, i32)>);

pub fn parse(input: &str) -> color_eyre::Result<ParsedInput> {
    let (first, second) = input
        .lines()
        .collect_tuple()
        .ok_or(AdventError::UnexpectedValue(
            String::from("Two lines"),
            input.to_string(),
        ))?;

    let first_path: Vec<PathStep> = first.split(',').map(PathStep::try_from).try_collect()?;
    let second_path: Vec<PathStep> = second.split(',').map(PathStep::try_from).try_collect()?;

    let first_path = gen_path(&first_path);
    let second_path = gen_path(&second_path);

    let intersections = intersection(&first_path, &second_path);

    Ok((first_path, second_path, intersections))
}

pub fn part1(input: &ParsedInput) -> color_eyre::Result<u32> {
    let nearest_intersection = input
        .2
        .iter()
        .map(|(x, y)| x.abs_diff(0) + y.abs_diff(0))
        .sorted()
        .next()
        .unwrap();

    Ok(nearest_intersection)
}

pub fn part2(input: &ParsedInput) -> color_eyre::Result<usize> {
    let first_path = &input.0;
    let second_path = &input.1;
    let intersections = &input.2;

    let shortest_path_intersection = intersections
        .iter()
        .filter_map(|intersection| {
            let first_path_steps = num_path_steps_to_point(first_path, intersection)?;
            let second_path_steps = num_path_steps_to_point(second_path, intersection)?;

            Some(first_path_steps + second_path_steps)
        })
        .sorted()
        .next()
        .ok_or(AdventError::LogicError(String::from("No intersections found")))?;

    // +2 because the paths don't contain the starting point (to avoid it being treated as an intersection)
    Ok(shortest_path_intersection + 2)
}

fn gen_path(steps: &Vec<PathStep>) -> Vec<(i32, i32)> {
    let mut path = Vec::new();
    let mut start = (0i32, 0i32);

    for step in steps {
        for offset in 1..=step.len {
            let next = match step.dir {
                Direction::Up => (start.0, start.1 + offset as i32),
                Direction::Right => (start.0 + offset as i32, start.1),
                Direction::Down => (start.0, start.1 - offset as i32),
                Direction::Left => (start.0 - offset as i32, start.1),
            };

            path.push(next);
        }

        start = match step.dir {
            Direction::Up => (start.0, start.1 + step.len as i32),
            Direction::Right => (start.0 + step.len as i32, start.1),
            Direction::Down => (start.0, start.1 - step.len as i32),
            Direction::Left => (start.0 - step.len as i32, start.1),
        };
    }

    path
}

fn intersection(vec1: &Vec<(i32, i32)>, vec2: &Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let set1 = vec1.iter().collect::<HashSet<_>>();
    let set2 = vec2.iter().collect::<HashSet<_>>();
    set1.intersection(&set2).copied().copied().collect_vec()
}

fn num_path_steps_to_point(path: &Vec<(i32, i32)>, point: &(i32, i32)) -> Option<usize> {
    path.iter().position(|path_point| path_point == point)
}

#[derive(Debug, PartialEq)]
pub struct PathStep {
    pub dir: Direction,
    pub len: u32,
}

impl TryFrom<&str> for PathStep {
    type Error = AdventError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() < 2 {
            return Err(AdventError::UnexpectedValue(
                String::from("Minimum 2-char string"),
                value.to_string(),
            )
            .into());
        }

        let (dir, len) = value.split_at(1);
        let dir = Direction::try_from(dir)?;
        let len = len.parse::<u32>()?;

        Ok(Self { dir, len })
    }
}

#[derive(Debug, PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl TryFrom<&str> for Direction {
    type Error = AdventError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "U" => Ok(Self::Up),
            "R" => Ok(Self::Right),
            "D" => Ok(Self::Down),
            "L" => Ok(Self::Left),
            _ => Err(AdventError::UnknownPattern(value.to_string()).into()),
        }
    }
}
