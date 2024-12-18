use itertools::Itertools;
use pathfinding::directed::astar::*;
use strum::IntoEnumIterator;

use crate::{
    error::AdventError,
    util::{direction::Direction, grid::Grid, point::Point},
};

type ParsedInput = (Grid<char>, Vec<Point>, usize);

pub fn parse(input: &str) -> color_eyre::Result<ParsedInput> {
    let mut lines = input.lines().peekable();
    // Your memory space is a two-dimensional grid with coordinates that range from 0 to 70
    // both horizontally and vertically. However, for the sake of example, suppose you're on
    // a smaller grid with coordinates that range from 0 to 6
    // Only consider first 12 byte for example, or 1024 for real input
    let (grid, num_bytes) = if lines.peek().is_some_and(|&line| line == "example") {
        lines.next();
        (Grid::init(7, 7, '.'), 12)
    } else {
        (Grid::init(71, 71, '.'), 1024)
    };

    let byte_locs: Vec<Point> = lines.map(|line| line.try_into()).try_collect()?;

    Ok((grid, byte_locs, num_bytes))
}

pub fn part1((grid, byte_locs, num_bytes): &ParsedInput) -> color_eyre::Result<usize> {
    let mut grid = grid.clone();
    corrupt_grid(&mut grid, &byte_locs[..*num_bytes])?;

    let (_, steps) = find_path(&grid).ok_or(AdventError::LogicError(
        "Failed to find path through grid".to_string(),
    ))?;

    Ok(steps)
}

pub fn part2((grid, byte_locs, num_bytes): &ParsedInput) -> color_eyre::Result<String> {
    // Can still corrupt the grid with the first num_bytes as we know a path is still
    // feasible when they are present from part 1
    let mut grid = grid.clone();
    corrupt_grid(&mut grid, &byte_locs[..*num_bytes])?;

    let (mut path, _) = find_path(&grid).ok_or(AdventError::LogicError(
        "Failed to find path through grid".to_string(),
    ))?;

    for byte_loc in &byte_locs[*num_bytes..] {
        let grid_val = grid
            .get_mut(byte_loc.y, byte_loc.x)
            .ok_or(AdventError::LogicError(format!(
                "Byte loc {byte_loc:?} not found in grid"
            )))?;

        *grid_val = '#';

        if path.contains(byte_loc) {
            if let Some((new_path, _)) = find_path(&grid) {
                path = new_path;
            }
            else {
                return Ok(format!("{},{}", byte_loc.x.to_string(), byte_loc.y.to_string()));
            }
        }
    }

    Err(AdventError::LogicError("No byte locs failed to block the path".to_string()).into())
}

fn corrupt_grid(grid: &mut Grid<char>, byte_locs: &[Point]) -> color_eyre::Result<()> {
    for byte_loc in byte_locs {
        let grid_val = grid
            .get_mut(byte_loc.y, byte_loc.x)
            .ok_or(AdventError::LogicError(format!(
                "Byte loc {byte_loc:?} not found in grid"
            )))?;

        *grid_val = '#';
    }

    Ok(())
}

fn find_path(grid: &Grid<char>) -> Option<(Vec<Point>, usize)> {
    let start = Point::from((0, 0)); //Node::new(0, 0, 0);
    let end = Point::from((grid.cols() - 1, grid.rows() - 1)); //Node::new(grid.cols() - 1, grid.rows() - 1, 0);

    let successors = |point: &Point| -> Vec<(Point, usize)> { get_successors(point, &grid) };
    let heuristic = |point: &Point| -> usize { get_heuristic(&point, &end) };
    let success = |point: &Point| -> bool { *point == end };

    astar(&start, successors, heuristic, success)
}

fn get_successors(point: &Point, grid: &Grid<char>) -> Vec<(Point, usize)> {
    Direction::iter()
        .filter_map(|direction| {
            let pos = (point.y, point.x);
            if let Some(((row, col), val)) = grid.get_in_direction_indexed(pos, direction) {
                if *val == '#' {
                    None
                } else {
                    //Some((Node::new(col, row, point.cost + 1), 1))
                    Some((Point::from((col, row)), 1))
                }
            } else {
                None
            }
        })
        .collect_vec()
}

fn get_heuristic(from: &Point, to: &Point) -> usize {
    let x_diff = to.x.abs_diff(from.x) as usize;
    let y_diff = to.y.abs_diff(from.y) as usize;

    x_diff + y_diff
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Node {
    loc: Point,
    // dir: Option<Direction>,
    cost: usize,
}

impl Node {
    fn new(x: usize, y: usize, cost: usize) -> Self {
        Self {
            loc: Point::from((x, y)),
            //dir,
            cost,
        }
    }

    fn is_same_loc(&self, other: &Node) -> bool {
        self.loc == other.loc
    }
}
