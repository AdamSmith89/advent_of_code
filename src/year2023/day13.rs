use grid::{grid, Grid};
use itertools::Itertools;

type ParsedInput = Vec<Grid<char>>;

pub fn parse(input: &str) -> color_eyre::Result<ParsedInput> {
    let input = input.replace("\r\n", "\n");

    let grids = input.split("\n\n").collect_vec();
    Ok(grids
        .iter()
        .map(|grid| {
            let mut parsed_grid = grid![];
            for line in grid.lines() {
                parsed_grid.push_row(line.chars().collect_vec());
            }
            parsed_grid
        })
        .collect_vec())
}

pub fn part1(grids: &ParsedInput) -> color_eyre::Result<usize> {
    let mut grids = grids.clone();

    let mut answer = 0;
    for grid in &mut grids {
        if let Some(mirror_row) = find_mirror(grid, false) {
            let rows_above = mirror_row + 1;
            answer += rows_above * 100;
        }

        grid.transpose();

        if let Some(mirror_col) = find_mirror(grid, false) {
            let cols_left = mirror_col + 1;
            answer += cols_left;
        }
    }

    Ok(answer)
}

pub fn part2(grids: &ParsedInput) -> color_eyre::Result<usize> {
    let mut grids = grids.clone();

    let mut answer = 0;
    for grid in &mut grids {
        if let Some(mirror_row) = find_mirror(grid, true) {
            let rows_above = mirror_row + 1;
            answer += rows_above * 100;
        } else {
            grid.transpose();

            if let Some(mirror_col) = find_mirror(grid, true) {
                let cols_left = mirror_col + 1;
                answer += cols_left;
            }
        }
    }

    Ok(answer)
}

fn find_mirror(grid: &mut Grid<char>, detect_smudge: bool) -> Option<usize> {
    let mut mirror_row = None;
    'row: for row_idx in 0..(grid.rows() - 1) {
        let mut lhs_idx = row_idx;
        let mut rhs_idx = lhs_idx + 1;

        let mut lhs = grid.iter_rows().nth(lhs_idx).unwrap().collect_vec();
        let mut rhs = grid.iter_rows().nth(rhs_idx).unwrap().collect_vec();
        let mut errors = lhs
            .iter()
            .zip_eq(rhs.iter())
            .filter(|(&l, &r)| l != r)
            .count();

        while errors == 0 || (detect_smudge && errors == 1) {
            if lhs_idx == 0 || rhs_idx == (grid.rows() - 1) {
                if detect_smudge && errors == 0 {
                    break;
                } else {
                    mirror_row = Some(row_idx);
                    break 'row;
                }
            }

            lhs_idx -= 1;
            rhs_idx += 1;

            lhs = grid.iter_rows().nth(lhs_idx).unwrap().collect_vec();
            rhs = grid.iter_rows().nth(rhs_idx).unwrap().collect_vec();
            errors += lhs
                .iter()
                .zip_eq(rhs.iter())
                .filter(|(&l, &r)| l != r)
                .count();

            if detect_smudge && errors > 1 {
                break;
            }
        }
    }

    mirror_row
}

fn print_grid(grid: &Grid<char>) {
    for row in grid.iter_rows() {
        row.for_each(|ch| print!("{ch}"));
        println!();
    }
}
