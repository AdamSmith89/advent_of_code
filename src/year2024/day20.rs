use strum::IntoEnumIterator;

use crate::{
    error::AdventError,
    util::{direction::Direction, grid::Grid},
};

type ParsedInput = Grid<char>;

pub fn parse(input: &str) -> color_eyre::Result<ParsedInput> {
    Ok(input.try_into()?)
}

pub fn part1(grid: &ParsedInput) -> color_eyre::Result<u32> {
    solve(grid, 2)
}

pub fn part2(grid: &ParsedInput) -> color_eyre::Result<u32> {
    solve(grid, 20)
}

fn solve(grid: &ParsedInput, max_cheat_dist: usize) -> color_eyre::Result<u32> {
    let path = get_full_path(grid)?;
    let limit = path.len() - 100; // We only want shortcuts that save more than 100 steps

    let mut cheat_count = 0;

    for (start_idx, (start_cheat, start_time)) in path[..limit].iter().enumerate() {
        let cheat_end_min = start_idx + 100 + 1; // Want to save at least 100 steps

        for (end_cheat, end_time) in path[cheat_end_min..].iter() {
            let cheat_dist = calc_manhattan_distance(&start_cheat, &end_cheat);
            let saved = end_time - start_time - cheat_dist;
            
            if cheat_dist <= max_cheat_dist && saved >= 100 {
                cheat_count += 1;
            }
        }
    }

    Ok(cheat_count)
}

fn get_full_path(grid: &ParsedInput) -> color_eyre::Result<Vec<((usize, usize), usize)>> {
    let start = grid.position(&'S').ok_or(AdventError::LogicError(
        "Failed to find start location".into(),
    ))?;
    let end = grid.position(&'E').ok_or(AdventError::LogicError(
        "Failed to find start location".into(),
    ))?;

    let mut prev_pos = start;
    let mut cur_pos = start;
    let mut time = 0;
    let mut path = Vec::new();
    while cur_pos != end {
        path.push((cur_pos, time));

        for dir in Direction::iter() {
            if let Some((next, val)) = grid.get_in_direction_indexed(cur_pos, dir) {
                if *val == '#' || next == prev_pos {
                    continue;
                }

                prev_pos = cur_pos;
                cur_pos = next;
                time += 1;
                break;
            }
        }
    }
    path.push((cur_pos, time));

    Ok(path)
}

fn calc_manhattan_distance(from: &(usize, usize), to: &(usize, usize)) -> usize {
    let x_diff = to.0.abs_diff(from.0);
    let y_diff = to.1.abs_diff(from.1);

    x_diff + y_diff
}
