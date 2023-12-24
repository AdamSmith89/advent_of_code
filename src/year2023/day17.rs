use itertools::Itertools;
use pathfinding::directed::dijkstra::*;
use strum::IntoEnumIterator;

use crate::error::AdventError;
use crate::util::grid::{Direction, Grid};

type ParsedInput = Grid<u32>;

pub fn parse(input: &str) -> color_eyre::Result<ParsedInput> {
    // Fix Grid try_from to support any type...
    // let g = Grid::<u32>::try_from(input)?;

    let mut inner = grid::Grid::new(0, 0);

    for line in input.lines() {
        inner.push_row(
            line.chars()
                .map(|ch| {
                    ch.to_digit(10)
                        .ok_or(AdventError::UnexpectedValue("number".into(), ch.into()))
                })
                .try_collect()?,
        );
    }

    Ok(Grid::from(inner))
}

pub fn part1(map: &ParsedInput) -> color_eyre::Result<usize> {
    let successors = |node: &Node| -> Vec<(Node, usize)> { get_cardinal_successors(map, node, 3) };

    let start = Node::new(0, 0);
    let end = Node::new(map.cols() - 1, map.rows() - 1);
    let success = |node: &Node| -> bool { node.is_same_loc(&end) };

    let path = dijkstra(&start, successors, success).expect("Failed to find path");
    // print_path(map, path.0);

    Ok(path.1)
}

pub fn part2(map: &ParsedInput) -> color_eyre::Result<usize> {
    let successors = |node: &Node| -> Vec<(Node, usize)> {
        let grid_loc = (node.loc.1, node.loc.0);

        if node.dir_count < 4 {
            if let Some(from) = node.in_dir {
                if let Some((next_point, _)) = map.get_in_direction_indexed(grid_loc, from) {
                    let mut next_node = Node::from((next_point.1, next_point.0));
                    next_node.in_dir = Some(from);
                    next_node.dir_count = node.dir_count + 1;

                    if let Some(weight) = map.get(next_node.loc.1, next_node.loc.0) {
                        return vec![(next_node, *weight as usize)];
                    }
                } else {
                    return vec![];
                }
            }
        }

        get_cardinal_successors(map, node, 10)
    };

    let start = Node::new(0, 0);
    let end = Node::new(map.cols() - 1, map.rows() - 1);
    let success = |node: &Node| -> bool { node.is_same_loc(&end) && node.dir_count >= 4 };

    let path = dijkstra(&start, successors, success).expect("Failed to find path");
    // print_path(map, path.0);

    Ok(path.1)
}

fn get_cardinal_successors(
    map: &ParsedInput,
    node: &Node,
    max_straight: usize,
) -> Vec<(Node, usize)> {
    let grid_loc = (node.loc.1, node.loc.0);

    Direction::iter()
        .filter_map(|direction| {
            if let Some((next_point, _)) = map.get_in_direction_indexed(grid_loc, direction) {
                if let Some(from) = node.in_dir {
                    // Are we reversing direction?
                    if from.is_opposite(&direction) {
                        None
                    } else {
                        let mut next_node = Node::from((next_point.1, next_point.0));
                        next_node.in_dir = Some(direction);

                        // Are we entering the neighbour in the same direction the node was entered?
                        next_node.dir_count = if from == direction {
                            node.dir_count + 1
                        } else {
                            1
                        };

                        if next_node.dir_count <= max_straight {
                            Some(next_node)
                        } else {
                            None
                        }
                    }
                } else {
                    // Special case for moving from the start node which won't have an in_dir
                    let mut next_node = Node::from((next_point.1, next_point.0));
                    next_node.in_dir = Some(direction);
                    next_node.dir_count = 2;

                    Some(next_node)
                }
            } else {
                None
            }
        })
        .map(|node: Node| {
            let weight = *(map.get(node.loc.1, node.loc.0).unwrap()) as usize;
            (node, weight)
        })
        .collect_vec()
}

fn _print_path(map: &ParsedInput, path: Vec<Node>) {
    for row in 0..map.rows() {
        for col in 0..map.cols() {
            if let Some(node) = path
                .iter()
                .find(|&node| node.loc == Point::from((col, row)))
            {
                if let Some(in_dir) = node.in_dir {
                    match in_dir {
                        Direction::North => print!("^"),
                        Direction::East => print!(">"),
                        Direction::South => print!("v"),
                        Direction::West => print!("<"),
                    };
                    continue;
                }
            }

            if let Some(weight) = map.get(row, col) {
                print!("{weight}");
            }
        }
        println!()
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Node {
    loc: Point,
    in_dir: Option<Direction>,
    dir_count: usize,
}

impl Node {
    fn new(x: usize, y: usize) -> Self {
        Self {
            loc: Point(x, y),
            in_dir: None,
            dir_count: 0,
        }
    }

    fn is_same_loc(&self, other: &Node) -> bool {
        self.loc == other.loc
    }
}

impl From<(usize, usize)> for Node {
    fn from(value: (usize, usize)) -> Self {
        Self::new(value.0, value.1)
    }
}

// Move to Grid?
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Point(usize, usize);

impl From<(usize, usize)> for Point {
    fn from(value: (usize, usize)) -> Self {
        Self(value.0, value.1)
    }
}

impl From<Point> for (usize, usize) {
    fn from(value: Point) -> Self {
        (value.0, value.1)
    }
}
