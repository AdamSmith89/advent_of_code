use std::collections::HashSet;

use crate::util::grid::Grid;

type ParsedInput = Grid<char>;

pub fn parse(input: &str) -> color_eyre::Result<ParsedInput> {
    Ok(input.into())
}

pub fn part1(grid: &ParsedInput) -> color_eyre::Result<u32> {
    // any number adjacent to a symbol, even diagonally, is a "part number"
    // add up all the part numbers in the engine schematic

    let mut h = HashSet::new();
    let mut first_visit = |(num, points): (u32, Vec<(usize, usize)>)| {
        if points.iter().all(|&point| h.insert(point)) {
            return Some(num);
        }
        None
    };

    let sum: u32 = grid
        .indexed_iter()
        .filter(|((_, _), &value)| !value.is_ascii_digit() && value != '.')
        .map(|((row, col), _)| adjacent_digit_points(grid, row, col))
        .flat_map(reduce_adjacent_digit_points)
        .map(|(row, col)| build_number_at_point(grid, row, col))
        .filter_map(|(num, points)| first_visit((num, points)))
        .sum();

    Ok(sum)
}

pub fn part2(grid: &ParsedInput) -> color_eyre::Result<u32> {
    let sum = grid
        .indexed_iter()
        // Filter to '*' locations - gears
        .filter(|((_, _), &value)| value == '*')
        // for each gear, map to the adjacent points which are digits
        .map(|((row, col), _)| adjacent_digit_points(grid, row, col))
        // for each set of adjacent digit points, remove the ones which are in the same number
        .map(reduce_adjacent_digit_points)
        // for each set of adjacent points, map to the full number
        .map(|points| {
            points
                .iter()
                .map(|(row, col)| build_number_at_point(grid, *row, *col).0)
                .collect::<Vec<_>>()
        })
        // for each set of adjacent numbers, filter to the sets with just 2 (gears only have 2 parts)
        .filter(|gear_parts| gear_parts.len() == 2)
        // for each set of adjacent numbers, multiply them
        .map(|gear_parts| gear_parts.iter().product::<u32>())
        // Sum all the products
        .sum();

    Ok(sum)
}

fn adjacent_digit_points(grid: &Grid<char>, row: usize, col: usize) -> Vec<(usize, usize)> {
    let offsets: Vec<(i32, i32)> = vec![
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
    ];

    offsets
        .iter()
        .filter_map(|(row_offset, col_offset)| {
            let row_n = usize::try_from(row as i32 + row_offset);
            let col_n = usize::try_from(col as i32 + col_offset);

            if row_n.is_ok() && col_n.is_ok() {
                let row_n = row_n.unwrap();
                let col_n = col_n.unwrap();
                if let Some(v) = grid.get(row_n, col_n) {
                    if v.is_ascii_digit() {
                        return Some((row_n, col_n));
                    }
                }
            }

            None
        })
        .collect::<Vec<_>>()
}

fn reduce_adjacent_digit_points(points: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut reduced_pts = Vec::new();
    let mut thrown_away = Vec::new();
    for (row, col) in points {
        if !reduced_pts
            .iter()
            .any(|(row_inner, col_inner)| row == *row_inner && col.abs_diff(*col_inner) == 1)
            && !thrown_away
                .iter()
                .any(|(row_inner, col_inner)| row == *row_inner && col.abs_diff(*col_inner) == 1)
        {
            reduced_pts.push((row, col));
        } else {
            thrown_away.push((row, col));
        }
    }
    reduced_pts
}

fn build_number_at_point(grid: &Grid<char>, row: usize, col: usize) -> (u32, Vec<(usize, usize)>) {
    let mut number = String::from(unsafe { *grid.get_unchecked(row, col) });
    let mut points = vec![(row, col)];

    grid.enum_row(row)
        .skip(col + 1)
        .take_while(|(_, ch)| ch.is_ascii_digit())
        .for_each(|(col, &ch)| {
            number.push(ch);
            points.push((row, col));
        });

    grid.enum_row(row)
        .rev()
        .skip(grid.cols() - col)
        .take_while(|(_, ch)| ch.is_ascii_digit())
        .for_each(|(col, &ch)| {
            number.insert(0, ch);
            points.push((row, col));
        });

    let number = number
        .parse()
        .unwrap_or_else(|_| panic!("Failed to parse {number} into u32"));

    (number, points)
}
