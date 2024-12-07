use std::collections::HashMap;

use grid::Order;
use itertools::Itertools;

use crate::error::AdventError;
use crate::util::grid::Grid;

type ParsedInput = Grid<char>;

pub fn parse(input: &str) -> color_eyre::Result<ParsedInput> {
    Ok(Grid::from_str_with_order(input, Order::ColumnMajor)?)
}

pub fn part1(grid: &ParsedInput) -> color_eyre::Result<usize> {
    let max_load = grid.rows();
    let mut total_load = 0;

    for col in grid.iter_cols() {
        let groups = get_groups_with_rocks(col);

        // Don't need to actually move the rocks, can just calculate based on number of rocks
        // and offset of each group
        for group in groups {
            let num_rocks = group.iter().filter(|(_, &value)| value == 'O').count();

            let start_idx = group[0].0;
            let end_idx = start_idx + num_rocks;

            let loads = (start_idx..end_idx).map(|v| max_load - v).collect_vec();

            total_load += loads.iter().sum::<usize>();
        }
    }

    Ok(total_load)
}

pub fn part2(grid: &ParsedInput) -> color_eyre::Result<usize> {
    let max_cycles = 1000000000;
    let mut cycles = 0;

    let mut cache = HashMap::new();
    cache.insert(grid.clone(), cycles);

    let mut grid = grid.clone();
    while cycles < max_cycles {
        for _ in 0..4 {
            for col in 0..grid.cols() {
                let mut next_empty = next_empty_row(&grid, col, 0);

                while let Some(empty_idx) = next_empty {
                    if let Some((idx, next)) = grid
                        .iter_col_mut(col)
                        .enumerate()
                        .skip(empty_idx)
                        .find(|(_, &mut ch)| ch != '.')
                    {
                        if *next == '#' {
                            next_empty = next_empty_row(&grid, col, idx);
                        } else if *next == 'O' {
                            grid.swap((idx, col), (empty_idx, col))?;
                            next_empty = next_empty_row(&grid, col, empty_idx);
                        }
                    } else {
                        next_empty = None;
                    }
                }
            }

            grid.rotate_right();
        }

        cycles += 1;

        if let Some(cycle_start) = cache.insert(grid.clone(), cycles) {
            let cycle_len = cycles - cycle_start;
            let rem = (max_cycles - 1 - cycle_start) % cycle_len;
            let final_idx = cycle_start + rem + 1;

            let (final_p, _) = cache
                .iter()
                .find(|&(_, value)| *value == final_idx)
                .ok_or(AdventError::NotFound(format!("{final_idx}")))?;
            grid = final_p.clone();
            break;
        }
    }

    let max_load = grid.rows();
    let mut total_load = 0;
    for col in grid.iter_cols() {
        for (idx, entry) in col.enumerate() {
            if *entry == 'O' {
                total_load += max_load - idx;
            }
        }
    }

    Ok(total_load)
}

fn get_groups_with_rocks<'a>(iter: impl Iterator<Item = &'a char>) -> Vec<Vec<(usize, &'a char)>> {
    iter.enumerate()
        .chunk_by(|(_, &ch)| ch != '#')
        .into_iter()
        .filter_map(|(key, group)| {
            if key {
                Some((key, group.collect_vec()))
            } else {
                None
            }
        })
        .filter_map(|(_, group)| {
            if !group.is_empty() && group.iter().any(|(_, &ch)| ch == 'O') {
                Some(group)
            } else {
                None
            }
        })
        .collect_vec()
}

fn next_empty_row(grid: &Grid<char>, col: usize, offset: usize) -> Option<usize> {
    grid.iter_col(col)
        .skip(offset)
        .position(|ch| *ch == '.')
        .map(|idx| idx + offset)
}
