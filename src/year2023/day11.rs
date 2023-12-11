use std::fmt::{Debug, Write};

use crate::error::AdventError;
use grid::{grid, Grid};
use itertools::Itertools;

type ParsedInput = Universe;

pub fn parse(input: &str) -> color_eyre::Result<ParsedInput> {
    let mut grid = grid![];

    for line in input.lines() {
        let row = line.chars().collect_vec();
        if row.iter().all_equal_value() == Ok(&'.') {
            grid.push_row(vec!['*'; grid.cols()]);
        }

        grid.push_row(row);
    }

    let mut col_idx = 0;
    loop {
        if grid.iter_col(col_idx).all(|v| *v == '.' || *v == '*') {
            grid.insert_col(col_idx, vec!['*'; grid.rows()]);
            col_idx += 1;
        }

        col_idx += 1;

        if col_idx == grid.cols() {
            break;
        }
    }

    Ok(Universe { grid })
}

pub fn part1(universe: &ParsedInput) -> color_eyre::Result<u64> {
    let distances = universe.get_distances(2)?;

    Ok(distances.iter().sum())
}

pub fn part2(universe: &ParsedInput) -> color_eyre::Result<u64> {
    let distances = universe.get_distances(1000000)?;

    Ok(distances.iter().sum())
}

#[derive(PartialEq)]
pub struct Universe {
    pub grid: Grid<char>,
}

impl Universe {
    fn get_distances(&self, expansion: u64) -> color_eyre::Result<Vec<u64>> {
        let distances: Vec<u64> = self
            .get_galaxies()
            .iter()
            .combinations(2)
            .map(|combinations| {
                let from = combinations.first().ok_or(AdventError::EmptySlice)?;
                let to = combinations.last().ok_or(AdventError::EmptySlice)?;

                Ok(self.distance_between(from, to, expansion))
            })
            .collect::<Result<Vec<u64>, AdventError>>()?;

        Ok(distances)
    }

    fn get_galaxies(&self) -> Vec<(usize, usize)> {
        self.grid
            .indexed_iter()
            .filter_map(|((row, col), value)| {
                if *value == '#' {
                    Some((row, col))
                } else {
                    None
                }
            })
            .collect_vec()
    }

    fn distance_between(&self, from: &(usize, usize), to: &(usize, usize), expansion: u64) -> u64 {
        let min_row = from.0.min(to.0);
        let max_row = from.0.max(to.0);
        let min_col = from.1.min(to.1);
        let max_col = from.1.max(to.1);

        let extra_rows = self
            .grid
            .iter_col(0)
            .skip(min_row)
            .take(max_row - min_row)
            .filter(|&v| *v == '*')
            .count() as u64;
        let extra_cols = self
            .grid
            .iter_row(0)
            .skip(min_col)
            .take(max_col - min_col)
            .filter(|&v| *v == '*')
            .count() as u64;

        let row_diff = to.0.abs_diff(from.0) as u64;
        let col_diff = to.1.abs_diff(from.1) as u64;

        let row_diff = row_diff + (extra_rows * (expansion - 2));
        let col_diff = col_diff + (extra_cols * (expansion - 2));

        row_diff + col_diff
    }
}

impl Debug for Universe {
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
