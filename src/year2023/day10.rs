use itertools::Itertools;
use strum::IntoEnumIterator;

use crate::{
    error::AdventError,
    util::grid::{Direction, Grid},
};

type ParsedInput = Grid<NodeType>;

pub fn parse(input: &str) -> color_eyre::Result<ParsedInput> {
    let mut inner = grid::Grid::new(0, 0);

    for line in input.lines() {
        inner.push_row(line.chars().map(NodeType::try_from).try_collect()?);
    }

    Ok(Grid::from(inner))
}

pub fn part1(grid: &ParsedInput) -> color_eyre::Result<usize> {
    let path = get_path(grid)?;
    if !path.is_empty() {
        Ok(path.len() / 2)
    } else {
        Err(AdventError::NotFound("Path not found".to_string()).into())
    }
}

pub fn part2(grid: &ParsedInput) -> color_eyre::Result<i64> {
    let (start, _) = grid
        .indexed_iter()
        .find(|&(_, node)| *node == NodeType::Start)
        .ok_or(AdventError::NotFound(NodeType::Start.to_string()))?;

    let mut path = get_path(grid)?;
    path.push(start);

    let perimeter = path.len() as i64;
    let mut area = 0;

    for (a, b) in path.iter().tuple_windows() {
        area += calc_determinant(&Point::from(*a), &Point::from(*b));
    }

    // Pick's Theorem
    Ok(area.abs() / 2 - (perimeter / 2 - 1))
}

// Shoelace Theorem
// Area = 0.5 * |(x1*y2 - x2*y1) + (x2*y3 - x3*y2) + ... + (xn*y1 - x1*yn)|
// Determinant = (xn*y1 - x1*yn)
fn calc_determinant(start: &Point, end: &Point) -> i64 {
    (start.0 * end.1) - (start.1 * end.0)
}

fn get_path(grid: &ParsedInput) -> color_eyre::Result<Vec<(usize, usize)>> {
    let (start, _) = grid
        .indexed_iter()
        .find(|&(_, node)| *node == NodeType::Start)
        .ok_or(AdventError::NotFound(NodeType::Start.to_string()))?;

    // Test with start being any of the possible pipe types
    for start_type in NodeType::iter() {
        let start_dir = match start_type {
            NodeType::Ground => continue,
            NodeType::Start => continue,
            NodeType::Vertical => Direction::South,
            NodeType::Horizontal => Direction::East,
            NodeType::BendNorthEast => Direction::East,
            NodeType::BendNorthWest => Direction::West,
            NodeType::BendSouthWest => Direction::West,
            NodeType::BendSouthEast => Direction::East,
        };

        let path = get_path_from(grid, start, start_type, start_dir);

        if !path.is_empty() {
            return Ok(path);
        }
    }

    Err(AdventError::NotFound("Path not found".to_string()).into())
}

fn get_path_from(
    grid: &ParsedInput,
    start: (usize, usize),
    start_type: NodeType,
    start_dir: Direction,
) -> Vec<(usize, usize)> {
    let mut path = vec![(start, start_dir)];

    while let Some(&(cur_node, cur_dir)) = path.last() {
        if let Some((next_node, next_type)) = grid.get_in_direction_indexed(cur_node, cur_dir) {
            if next_node == start {
                if match cur_dir {
                    Direction::North => matches!(
                        start_type,
                        NodeType::Vertical | NodeType::BendSouthEast | NodeType::BendSouthWest
                    ),
                    Direction::East => matches!(
                        start_type,
                        NodeType::Horizontal | NodeType::BendNorthWest | NodeType::BendSouthWest
                    ),
                    Direction::South => matches!(
                        start_type,
                        NodeType::Vertical | NodeType::BendNorthEast | NodeType::BendNorthWest
                    ),
                    Direction::West => matches!(
                        start_type,
                        NodeType::Horizontal | NodeType::BendNorthEast | NodeType::BendSouthEast
                    ),
                } {
                    break;
                }
            }

            if let Some(next_dir) = get_next_direction(*next_type, cur_dir) {
                path.push((next_node, next_dir));
            } else {
                path.clear();
                break;
            }
        } else {
            path.clear();
            break;
        }
    }

    path.iter().map(|(point, _)| point).cloned().collect_vec()
}

fn get_next_direction(next_type: NodeType, cur_dir: Direction) -> Option<Direction> {
    match cur_dir {
        Direction::North => {
            if next_type == NodeType::Vertical {
                Some(cur_dir)
            } else if next_type == NodeType::BendSouthEast {
                Some(Direction::East)
            } else if next_type == NodeType::BendSouthWest {
                Some(Direction::West)
            } else {
                None
            }
        }
        Direction::East => {
            if next_type == NodeType::Horizontal {
                Some(cur_dir)
            } else if next_type == NodeType::BendNorthWest {
                Some(Direction::North)
            } else if next_type == NodeType::BendSouthWest {
                Some(Direction::South)
            } else {
                None
            }
        }
        Direction::South => {
            if next_type == NodeType::Vertical {
                Some(cur_dir)
            } else if next_type == NodeType::BendNorthEast {
                Some(Direction::East)
            } else if next_type == NodeType::BendNorthWest {
                Some(Direction::West)
            } else {
                None
            }
        }
        Direction::West => {
            if next_type == NodeType::Horizontal {
                Some(cur_dir)
            } else if next_type == NodeType::BendNorthEast {
                Some(Direction::North)
            } else if next_type == NodeType::BendSouthEast {
                Some(Direction::South)
            } else {
                None
            }
        }
    }
}

struct Point(i64, i64);

impl From<(usize, usize)> for Point {
    fn from(value: (usize, usize)) -> Self {
        Self(value.0 as i64, value.1 as i64)
    }
}

#[derive(
    Copy, Clone, Debug, strum_macros::Display, PartialEq, Eq, strum_macros::EnumIter, Hash,
)]
pub enum NodeType {
    Ground,
    Start,
    Vertical,
    Horizontal,
    BendNorthEast,
    BendNorthWest,
    BendSouthWest,
    BendSouthEast,
}

impl Default for NodeType {
    fn default() -> Self {
        NodeType::Ground
    }
}

impl TryFrom<char> for NodeType {
    type Error = AdventError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(NodeType::Ground),
            'S' => Ok(NodeType::Start),
            '|' => Ok(NodeType::Vertical),
            '-' => Ok(NodeType::Horizontal),
            'L' => Ok(NodeType::BendNorthEast),
            'J' => Ok(NodeType::BendNorthWest),
            '7' => Ok(NodeType::BendSouthWest),
            'F' => Ok(NodeType::BendSouthEast),
            _ => Err(AdventError::UnknownPattern(value.into())),
        }
    }
}
