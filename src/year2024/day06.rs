use std::collections::HashSet;

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
    let mut cur_dir = Direction::North;
    let mut cur_pos = get_start_pos(map)?;

    let mut visited = HashSet::new();
    visited.insert((cur_pos, cur_dir));

    let mut obstacles = HashSet::new();
    let mut loops = 0;

    while let Some((next_pos, next_val)) = map.get_in_direction_indexed(cur_pos, cur_dir) {
        if *next_val == '#' {
            cur_dir = cur_dir.rotate_90_cwise();
        } else {
            // Only test obstacles once as hitting it later in the path a 2nd time shouldn't count
            if obstacles.insert(next_pos)
                && does_path_loop(&cur_pos, &cur_dir, &visited, map, next_pos)?
            {
                loops += 1;
            }

            cur_pos = next_pos;
            visited.insert((cur_pos, cur_dir));
        }
    }

    Ok(loops)
}

fn get_start_pos(map: &ParsedInput) -> color_eyre::Result<(usize, usize)> {
    Ok(map
        .indexed_iter()
        .find(|&(_, val)| *val == '^')
        .ok_or(AdventError::NotFound('^'.to_string()))?
        .0)
}

fn find_distinct_path(map: &ParsedInput) -> color_eyre::Result<HashSet<(usize, usize)>> {
    let mut cur_dir = Direction::North;
    let mut cur_pos = get_start_pos(map)?;

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

fn does_path_loop(
    start_pos: &(usize, usize),
    start_dir: &Direction,
    visited: &HashSet<((usize, usize), Direction)>,
    map: &ParsedInput,
    temp_obstacle: (usize, usize),
) -> color_eyre::Result<bool> {
    let mut cur_dir = *start_dir;
    let mut cur_pos = *start_pos;

    // Use a 2nd HashSet for points visited from now to save cloning
    let mut visited2 = HashSet::new();

    while let Some((next_pos, next_val)) = map.get_in_direction_indexed(cur_pos, cur_dir) {
        if *next_val == '#' || next_pos == temp_obstacle {
            cur_dir = cur_dir.rotate_90_cwise();
        } else {
            cur_pos = next_pos;

            if visited.contains(&(cur_pos, cur_dir)) || !visited2.insert((cur_pos, cur_dir)) {
                // We've hit the same position going in the same direction as we have before
                // So we must be in a loop
                return Ok(true);
            }
        }
    }

    Ok(false)
}
