use std::{
    borrow::BorrowMut,
    collections::{hash_map::DefaultHasher, HashMap},
    fmt::{Debug, Write},
    io::empty,
};

use grid::{Grid, Order};
use itertools::Itertools;

use crate::error::AdventError;

type ParsedInput = Platform;

pub fn parse(input: &str) -> color_eyre::Result<ParsedInput> {
    let mut grid = Grid::new_with_order(0, 0, Order::ColumnMajor);

    for line in input.lines() {
        grid.push_row(line.chars().collect());
    }

    Ok(Platform { grid })
}

pub fn part1(platform: &ParsedInput) -> color_eyre::Result<usize> {
    let max_load = platform.grid.rows();
    let mut total_load = 0;

    for col in platform.grid.iter_cols() {
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

pub fn part2(platform: &ParsedInput) -> color_eyre::Result<usize> {
    let max_cycles = 1000000000;
    let mut platform = platform.clone();
    let mut cycles = 0;

    let mut cache = HashMap::new();
    cache.insert(platform.clone(), cycles);
    println!("{platform:?}");
    while cycles < max_cycles {
        platform = platform.clone();

        // O....#....
        // O.OO#....#
        // .....##...
        // OO.#O....O
        // .O.....O#.
        // O.#..O.#.#
        // ..O..#O..O
        // .......O..
        // #....###..
        // #OO..#....

        for _ in 0..4 {
            for col in 0..platform.grid.cols() {
                let mut empty_row = platform
                    .grid
                    .iter_col(col)
                    .position(|ch| *ch == '.')
                    .unwrap();
                let mut row = empty_row + 1;

                while row < platform.grid.rows() {
                    // let entry = platform
                    //     .grid
                    //     .get(row, col)
                    //     .ok_or(AdventError::NotFound(format!("{:?}", (row, col)).into()))?;

                    // if *entry == '#' {
                    //     empty_row = platform.grid.iter_col(col).skip(empty_row + 1).position(|ch| *ch == '.').unwrap();
                    //     row = empty_row + 1;
                    // }
                    // if *entry == 'O' {
                    //     platform.swap((row, col), (empty_row, col))?;
                    //     empty_row += 1;
                    // }

                    // row += 1;

                    let entry = platform
                        .grid
                        .get(row, col)
                        .ok_or(AdventError::NotFound(format!("{:?}", (row, col)).into()))?;
                    let prev_entry =
                        platform
                            .grid
                            .get(row - 1, col)
                            .ok_or(AdventError::NotFound(
                                format!("{:?}", (row - 1, col)).into(),
                            ))?;

                    if *entry == 'O' && *prev_entry == '.' {
                        platform.swap((row, col), (row - 1, col))?;
                        if row > 1 {
                            row -= 1;
                        }
                    } else {
                        row += 1;
                    }
                }
            }

            //println!("{platform:?}");

            platform.grid.rotate_right();
        }

        //println!("{platform:?}");

        cycles += 1;

        if let Some(cycle_start) = cache.insert(platform.clone(), cycles) {
            let cycle_len = cycles - cycle_start;
            let rem = (max_cycles - 1 - cycle_start) % cycle_len;
            let final_idx = cycle_start + rem + 1;

            let (final_p, _) = cache
                .iter()
                .find(|&(_, value)| *value == final_idx)
                .unwrap();
            platform = final_p.clone();
            break;
        }
    }

    let max_load = platform.grid.rows();
    let mut total_load = 0;
    for col in platform.grid.iter_cols() {
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
        .group_by(|(_, &ch)| ch != '#')
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

#[derive(Clone, Eq, PartialEq)]
pub struct Platform {
    pub grid: Grid<char>,
}

impl Platform {
    fn swap(&mut self, x: (usize, usize), y: (usize, usize)) -> color_eyre::Result<()> {
        let t = self.grid[x];
        self.grid[x] = self.grid[y];
        self.grid[y] = t;
        Ok(())
    }
}

impl std::hash::Hash for Platform {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.grid.iter().for_each(|entry| entry.hash(state));
    }
}

impl Debug for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("\n").unwrap();
        self.grid.iter_rows().for_each(|row| {
            row.for_each(|ch| {
                f.write_char(*ch).unwrap();
            });
            f.write_str("\n").unwrap();
        });

        Ok(())
    }
}
