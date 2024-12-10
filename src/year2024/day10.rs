use std::collections::{vec_deque, HashSet, VecDeque};

use itertools::Itertools;
use strum::IntoEnumIterator;

use crate::{
    error::AdventError,
    util::{direction::Direction, grid::Grid},
};

type ParsedInput = Grid<u32>;

pub fn parse(input: &str) -> color_eyre::Result<ParsedInput> {
    // TODO: Fix Grid try_from to support any type...
    // let g = Grid::<u32>::try_from(input)?;

    let mut inner = grid::Grid::new(0, 0);

    for line in input.lines() {
        inner.push_row(
            line.chars()
                .map(|ch| {
                    ch.to_digit(10)
                        .ok_or(AdventError::UnexpectedValue("number".into(), ch.into()))
                })
                .try_collect()?,
        );
    }

    Ok(Grid::from(inner))
}

pub fn part1(map: &ParsedInput) -> color_eyre::Result<usize> {
    let mut sum_trailheads = 0;
    let mut map_iter = map.indexed_iter();

    while let Some(start_step) = map_iter.find(|&(_, height)| *height == 0) {
        let mut steps = VecDeque::from(vec![start_step]);

        let mut reachable_ends = HashSet::new();

        while let Some((cur_loc, cur_height)) = steps.pop_back() {
            if *cur_height == 9 {
                reachable_ends.insert(cur_loc);
                continue;
            }

            let mut next_height = cur_height + 1;

            for dir in Direction::iter() {
                if let Some(next_step) = map
                    .get_in_direction_indexed(cur_loc, dir)
                    .filter(|&(_, height)| *height == next_height)
                {
                    steps.push_back(next_step);
                }
            }
        }

        sum_trailheads += reachable_ends.len();
    }

    Ok(sum_trailheads)
}

pub fn part2(map: &ParsedInput) -> color_eyre::Result<u32> {
    let mut sum_trailheads = 0;
    let mut map_iter = map.indexed_iter();

    while let Some(start_step) = map_iter.find(|&(_, height)| *height == 0) {
        let mut steps = VecDeque::from(vec![start_step]);

        let mut reachable_ends = 0;

        while let Some((cur_loc, cur_height)) = steps.pop_back() {
            if *cur_height == 9 {
                reachable_ends += 1;
                continue;
            }

            let mut next_height = cur_height + 1;

            for dir in Direction::iter() {
                if let Some(next_step) = map
                    .get_in_direction_indexed(cur_loc, dir)
                    .filter(|&(_, height)| *height == next_height)
                {
                    steps.push_back(next_step);
                }
            }
        }

        sum_trailheads += reachable_ends;
    }

    Ok(sum_trailheads)
}
