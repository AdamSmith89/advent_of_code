use std::collections::HashSet;

use log::debug;

use crate::{
    error::AdventError,
    util::grid::{Direction, Grid},
};

type ParsedInput = Grid<char>;

pub fn parse(input: &str) -> color_eyre::Result<ParsedInput> {
    Ok(input.try_into()?)
}

pub fn part1(map: &ParsedInput) -> color_eyre::Result<usize> {
    let visited = find_distinct_path(map)?;
    Ok(visited.len())
}

pub fn part2(map: &ParsedInput) -> color_eyre::Result<u32> {
    let mut map = map.clone();

    let mut cur_dir = Direction::North;
    let (mut cur_pos, _) = map
        .indexed_iter()
        .find(|&(_, val)| *val == '^')
        .ok_or(AdventError::NotFound('^'.to_string()))?;

    let mut visited = HashSet::new();
    visited.insert((cur_pos, cur_dir));

    let mut obstacles = HashSet::new();
    let mut loops = 0;

    while let Some((next_pos, next_val)) = map.get_in_direction_indexed(cur_pos, cur_dir) {
        debug!("cur_pos = {cur_pos:?}, cur_dir = {cur_dir}");
        debug!("next_pos = {next_pos:?}, next_val = {next_val}");
        debug!("visited = {visited:?}");
        debug!("loops = {loops}, obstacles = {obstacles:?}");

        if *next_val == '#' {
            cur_dir = cur_dir.rotate_90_cwise();
        } else {
            let val = map
                .get_mut(next_pos.0, next_pos.1)
                .ok_or(AdventError::LogicError(
                    "Visited location not present in grid".to_string(),
                ))?;
            *val = '#';
            debug!("Obstacle placed at {next_pos:?}");

            // does_path_loop_2 is quicker but doesn't work
            // does_path_loop_1 does work but is slower

            //if does_path_loop_2(&cur_pos, &cur_dir, &visited, &map)? {
            if does_path_loop_1(&map)? {
                if obstacles.insert(next_pos) {
                    loops += 1;
                }
            }

            let val = map
                .get_mut(next_pos.0, next_pos.1)
                .ok_or(AdventError::LogicError(
                    "Visited location not present in grid".to_string(),
                ))?;
            *val = '.';

            cur_pos = next_pos;
            visited.insert((cur_pos, cur_dir));
        }
    }

    debug!("Obstacles = {}", obstacles.len());
    Ok(loops)
}

fn find_distinct_path(map: &ParsedInput) -> color_eyre::Result<HashSet<(usize, usize)>> {
    let mut cur_dir = Direction::North;
    let (mut cur_pos, _) = map
        .indexed_iter()
        .find(|&(_, val)| *val == '^')
        .ok_or(AdventError::NotFound('^'.to_string()))?;

    let mut visited = HashSet::new();
    visited.insert(cur_pos);

    while let Some((next_pos, next_val)) = map.get_in_direction_indexed(cur_pos, cur_dir) {
        if *next_val == '#' {
            cur_dir = cur_dir.rotate_90_cwise();
        } else {
            cur_pos = next_pos;
            visited.insert(cur_pos);
        }
    }

    Ok(visited)
}

fn does_path_loop_2(
    start_pos: &(usize, usize),
    start_dir: &Direction,
    visited: &HashSet<((usize, usize), Direction)>,
    map: &ParsedInput,
) -> color_eyre::Result<bool> {
    let mut cur_dir = start_dir.clone();
    let mut cur_pos = start_pos.clone();
    let mut visited = visited.clone();

    while let Some((next_pos, next_val)) = map.get_in_direction_indexed(cur_pos, cur_dir) {
        if *next_val == '#' {
            cur_dir = cur_dir.rotate_90_cwise();
        } else {
            cur_pos = next_pos;

            if !visited.insert((cur_pos, cur_dir)) {
                // We've hit the same position going in the same direction as we have before
                // So we must be in a loop
                return Ok(true);
            }
        }
    }

    Ok(false)
}

fn does_path_loop_1(map: &ParsedInput) -> color_eyre::Result<bool> {
    let mut cur_dir = Direction::North;
    let (mut cur_pos, _) = map
        .indexed_iter()
        .find(|&(_, val)| *val == '^')
        .ok_or(AdventError::NotFound('^'.to_string()))?;

    let mut visited = HashSet::new();
    visited.insert((cur_pos, cur_dir));

    while let Some((next_pos, next_val)) = map.get_in_direction_indexed(cur_pos, cur_dir) {
        if *next_val == '#' {
            cur_dir = cur_dir.rotate_90_cwise();
        } else {
            cur_pos = next_pos;

            if !visited.insert((cur_pos, cur_dir)) {
                // We've hit the same position going in the same direction as we have before
                // So we must be in a loop
                return Ok(true);
            }
        }
    }

    Ok(false)
}
