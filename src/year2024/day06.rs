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
    Ok(visited.iter().count())
}

pub fn part2(map: &ParsedInput) -> color_eyre::Result<u32> {
    // Run part 1 to get path guard follows
    // For each pos on the path
    //  Add an obstacle
    //  Re-run the path
    //  Store pos + dir for each step
    //  If at any point we hit the same pos + dir we've seen before
    //  Then we are looping

    let (start_pos, _) = map
        .indexed_iter()
        .find(|&(_, val)| *val == '^')
        .ok_or(AdventError::NotFound('^'.to_string()))?;

    let mut map = map.clone();
    let visited = find_distinct_path(&map)?;

    let mut loops = 0;

    for (row, col) in visited {
        if (row, col) == start_pos {
            continue;
        }

        let val = map.get_mut(row, col).ok_or(AdventError::LogicError(
            "Visited location not present in grid".to_string(),
        ))?;
        *val = '#';

        if does_path_loop(&map)? {
            loops += 1;
        }

        let val = map.get_mut(row, col).ok_or(AdventError::LogicError(
            "Visited location not present in grid".to_string(),
        ))?;
        *val = '.';
    }

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

fn does_path_loop(map: &ParsedInput) -> color_eyre::Result<bool> {
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
