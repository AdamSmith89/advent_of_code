use std::collections::HashMap;

use grid::grid;
use itertools::Itertools;
use pathfinding::directed::dijkstra::dijkstra;
use strum::IntoEnumIterator;

use crate::{
    error::AdventError,
    util::{direction::Direction, grid::Grid, point::Point},
};

type ParsedInput = Grid<Tile>;

pub fn parse(input: &str) -> color_eyre::Result<ParsedInput> {
    let mut grid = grid![];

    for line in input.lines() {
        grid.push_row(line.chars().map(Tile::try_from).try_collect()?);
    }

    Ok(Grid::from(grid))
}

pub fn part1(grid: &ParsedInput) -> color_eyre::Result<usize> {
    let start = grid
        .iter_row(0)
        .enumerate()
        .find(|(_, tile)| Tile::is_path(tile))
        .map(|(col, _)| (0usize, col))
        .ok_or(AdventError::NotFound("Path".to_string()))?;

    let last_row = grid.rows() - 1;
    let end = grid
        .iter_row(last_row)
        .enumerate()
        .find(|(_, tile)| Tile::is_path(tile))
        .map(|(col, _)| (last_row, col))
        .ok_or(AdventError::NotFound("Path".to_string()))?;

    if let Some(paths) = find_paths(
        grid,
        Point::from(start),
        Direction::South,
        Point::from(end),
        true,
    ) {
        // if paths.is_empty() {
        //     println!("Failed to find any path to the end");
        // }

        // for path in &paths {
        //     println!("Len = {}", path.len() - 1);
        //     //println!("{path:?}");
        // }

        let longest_path = paths
            .iter()
            .max_by(|p1, p2| p1.len().cmp(&p2.len()))
            .unwrap();
        //println!("Longest path = {}", longest_path.len() - 1);

        Ok(longest_path.len() - 1)
    } else {
        Err(AdventError::NotFound("Path".into()).into())
    }
}

pub fn part2(grid: &ParsedInput) -> color_eyre::Result<usize> {
    // Running part 1 code doesn't work, it blows the stack.
    // Need to first create a graph of junction nodes so can search that instead

    let start: Point = grid
        .iter_row(0)
        .enumerate()
        .find(|(_, tile)| Tile::is_path(tile))
        .map(|(col, _)| (0usize, col))
        .ok_or(AdventError::NotFound("Path".to_string()))?
        .into();

    let last_row = grid.rows() - 1;
    let end: Point = grid
        .iter_row(last_row)
        .enumerate()
        .find(|(_, tile)| Tile::is_path(tile))
        .map(|(col, _)| (last_row, col))
        .ok_or(AdventError::NotFound("Path".to_string()))?
        .into();

    let get_cardinal_neighbours = |p: &Point| -> Vec<(Point, usize)> {
        Direction::iter()
            .filter_map(|dir| {
                if let Some((next_pos, next_tile)) = grid.get_in_direction_indexed((*p).into(), dir)
                {
                    if !matches!(next_tile, Tile::Forest) {
                        Some((next_pos.into(), 1))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect_vec()
    };

    let mut junctions = vec![start];

    junctions.append(
        &mut grid
            .indexed_iter()
            .filter(|(_, tile)| Tile::is_path(tile))
            .filter_map(|((row, col), _)| {
                let available_moves = get_cardinal_neighbours(&Point::from((row, col))).len();

                if available_moves >= 3 {
                    Some(Point::from((row, col)))
                } else {
                    None
                }
            })
            .collect_vec(),
    );

    junctions.push(end);

    println!("{junctions:?}");

    let mut links = HashMap::new();

    for (start, end) in junctions.iter().tuple_combinations() {
        let success = |p: &Point| -> bool { p == end };

        if links
            .get(start)
            .is_some_and(|linked_to: &Vec<(Point, usize)>| linked_to.iter().any(|(p, _)| p == end))
        {
            continue;
        }

        // Don't let us go through another junction to find this one
        let successors = |p: &Point| -> Vec<(Point, usize)> {
            get_cardinal_neighbours(p)
                .iter()
                .filter(|&next| next.0 == *end || !junctions.contains(&next.0))
                .cloned()
                .collect_vec()
        };

        if let Some((_, steps)) = dijkstra(start, successors, success) {
            links
                .entry(start)
                .and_modify(|v: &mut Vec<(Point, usize)>| {
                    v.push((*end, steps));
                })
                .or_insert(vec![(*end, steps)]);

            links
                .entry(end)
                .and_modify(|v: &mut Vec<(Point, usize)>| {
                    v.push((*start, steps));
                })
                .or_insert(vec![(*start, steps)]);
        }
    }

    //println!("{links:#?}");

    Ok(0)
}

pub fn find_paths(
    grid: &ParsedInput,
    start: Point,
    start_dir: Direction,
    end: Point,
    steep_slopes: bool,
) -> Option<Vec<Vec<Point>>> {
    let mut path: Vec<((usize, usize), Direction)> = vec![(start.into(), start_dir)];

    loop {
        let mut next_moves = vec![];
        for next_dir in Direction::iter() {
            let &(cur_pos, cur_dir) = path.last().unwrap(); // Handle better

            // Don't go back on ourselves
            if next_dir.is_opposite(&cur_dir) {
                continue;
            }

            if let Some((next_pos, next_tile)) = grid.get_in_direction_indexed(cur_pos, next_dir) {
                match next_tile {
                    Tile::Path => next_moves.push((next_pos, next_dir)),
                    Tile::Forest => (),
                    Tile::SlopeNorth => {
                        if !steep_slopes || !matches!(next_dir, Direction::South) {
                            next_moves.push((next_pos, next_dir))
                        }
                    }
                    Tile::SlopeEast => {
                        if !steep_slopes || !matches!(next_dir, Direction::West) {
                            next_moves.push((next_pos, next_dir))
                        }
                    }
                    Tile::SlopeSouth => {
                        if !steep_slopes || !matches!(next_dir, Direction::North) {
                            next_moves.push((next_pos, next_dir))
                        }
                    }
                    Tile::SlopeWest => {
                        if !steep_slopes || !matches!(next_dir, Direction::East) {
                            next_moves.push((next_pos, next_dir))
                        }
                    }
                }
            }
        }

        if next_moves.iter().any(|(pos, _)| *pos == end.into()) {
            let mut path = path
                .iter()
                .map(|(point, _)| Point::from(*point))
                .collect_vec();
            path.push(end);
            return Some(vec![path]);
        } else if next_moves.len() == 1 {
            // continue the same path
            path.append(&mut next_moves);
        } else if next_moves.len() > 1 {
            // split paths

            let mut new_paths = vec![];
            let (path, _): (Vec<_>, Vec<_>) = path.into_iter().unzip();
            let path = path.iter().map(Point::from).collect_vec();

            for (next_start, next_dir) in next_moves {
                if let Some(mut rem_paths) =
                    find_paths(grid, Point::from(next_start), next_dir, end, steep_slopes)
                {
                    for rem_path in rem_paths.iter_mut() {
                        let mut new_path = path.clone();
                        new_path.append(rem_path);
                        new_paths.push(new_path.clone());
                    }
                }
            }

            if new_paths.is_empty() {
                // None of the remaining paths found the end
                return None;
            }
            return Some(new_paths);
        } else {
            // this path has run out of path to search
            return None;
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, strum_macros::Display)]
pub enum Tile {
    Path,
    Forest,
    SlopeNorth,
    SlopeEast,
    SlopeSouth,
    SlopeWest,
}

impl Tile {
    fn is_path(tile: &&Tile) -> bool {
        **tile == Tile::Path
    }
}

impl Default for Tile {
    fn default() -> Self {
        Self::Path
    }
}

impl TryFrom<char> for Tile {
    type Error = AdventError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Tile::Path),
            '#' => Ok(Tile::Forest),
            '^' => Ok(Tile::SlopeNorth),
            '>' => Ok(Tile::SlopeEast),
            'v' => Ok(Tile::SlopeSouth),
            '<' => Ok(Tile::SlopeWest),
            _ => Err(AdventError::UnknownPattern(value.into())),
        }
    }
}
