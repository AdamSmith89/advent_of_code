use std::collections::{HashMap, HashSet};

use itertools::Itertools;

type ParsedInput = (i32, i32, HashMap<char, Vec<(i32, i32)>>);

pub fn parse(input: &str) -> color_eyre::Result<ParsedInput> {
    let mut rows = 0;
    let mut cols = 0;
    let mut antenna: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch != '.' {
                antenna
                    .entry(ch)
                    .or_default()
                    .push((row as i32, col as i32));
            }

            cols = cols.max(col);
        }

        rows = rows.max(row);
    }

    Ok((rows as i32 + 1, cols as i32 + 1, antenna))
}

pub fn part1((rows, cols, antenna): &ParsedInput) -> color_eyre::Result<usize> {
    let mut antinodes = HashSet::new();

    for points in antenna.values() {
        for (&from, &to) in points.iter().tuple_combinations() {
            let projection = (from.0 - to.0, from.1 - to.1);

            if let Some(antinode) =
                project_antinode(from, to, projection, rows, cols, std::ops::Sub::sub)
            {
                antinodes.insert(antinode);
            }

            if let Some(antinode) =
                project_antinode(from, to, projection, rows, cols, std::ops::Add::add)
            {
                antinodes.insert(antinode);
            }
        }
    }

    Ok(antinodes.len())
}

pub fn part2((rows, cols, antenna): &ParsedInput) -> color_eyre::Result<usize> {
    let mut antinodes = HashSet::new();

    for points in antenna.values() {
        for (&from, &to) in points.iter().tuple_combinations() {
            antinodes.insert(from);
            antinodes.insert(to);

            let projection = (from.0 - to.0, from.1 - to.1);

            let mut temp_from = from;
            while let Some(antinode) =
                project_antinode(temp_from, to, projection, rows, cols, std::ops::Sub::sub)
            {
                antinodes.insert(antinode);
                temp_from = antinode;
            }

            let mut temp_from = from;
            while let Some(antinode) =
                project_antinode(temp_from, to, projection, rows, cols, std::ops::Add::add)
            {
                antinodes.insert(antinode);
                temp_from = antinode;
            }
        }
    }

    Ok(antinodes.len())
}

fn project_antinode<Op>(
    from: (i32, i32),
    to: (i32, i32),
    projection: (i32, i32),
    rows: &i32,
    cols: &i32,
    op: Op,
) -> Option<(i32, i32)>
where
    Op: Fn(i32, i32) -> i32,
{
    let mut antinode = (op(from.0, projection.0), op(from.1, projection.1));

    if antinode == to {
        antinode = (op(to.0, projection.0), op(to.1, projection.1));
    }

    if is_in_bounds(antinode, *rows, *cols) {
        Some(antinode)
    } else {
        None
    }
}

fn is_in_bounds(point: (i32, i32), rows: i32, cols: i32) -> bool {
    point.0 >= 0 && point.0 < rows && point.1 >= 0 && point.1 < cols
}
