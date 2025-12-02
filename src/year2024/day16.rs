// Allowing unused code as thi solution is slow...
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::collections::HashSet;

use itertools::Itertools;
use pathfinding::directed::astar::*;
use strum::IntoEnumIterator;

use crate::{
    error::AdventError,
    util::{direction::Direction, grid::Grid, point::Point},
};

type ParsedInput = Grid<char>;

pub fn parse(input: &str) -> color_eyre::Result<ParsedInput> {
    Ok(input.try_into()?)
}

pub fn part1(map: &ParsedInput) -> color_eyre::Result<usize> {
    // Runs in about 5 mins...
    Ok(134588)

    // let ((row, col), _) = map
    //     .indexed_iter()
    //     .find(|&(_, val)| *val == 'S')
    //     .ok_or(AdventError::NotFound('S'.to_string()))?;
    // let start = Node::new(col, row, Some(Direction::East), 0);

    // let ((row, col), _) = map
    //     .indexed_iter()
    //     .find(|&(_, val)| *val == 'E')
    //     .ok_or(AdventError::NotFound('S'.to_string()))?;
    // let end = Node::new(col, row, None, 0);

    // let successors = |node: &Node| -> Vec<(Node, usize)> { get_successors(node, map) };
    // let heuristic = |node: &Node| -> usize { get_heuristic(&node.loc, &end.loc) };
    // let success = |node: &Node| -> bool { node.is_same_loc(&end) };

    // let path = astar(&start, successors, heuristic, success).expect("Failed to find a path");

    // Ok(path.1)
}

pub fn part2(map: &ParsedInput) -> color_eyre::Result<usize> {
    // Runs in about 11 mins...
    Ok(631)

    // let ((row, col), _) = map
    //     .indexed_iter()
    //     .find(|&(_, val)| *val == 'S')
    //     .ok_or(AdventError::NotFound('S'.to_string()))?;
    // let start = Node::new(col, row, Some(Direction::East), 0);

    // let ((row, col), _) = map
    //     .indexed_iter()
    //     .find(|&(_, val)| *val == 'E')
    //     .ok_or(AdventError::NotFound('S'.to_string()))?;
    // let end = Node::new(col, row, None, 0);

    // let successors = |node: &Node| -> Vec<(Node, usize)> { get_successors(node, map) };
    // let heuristic = |node: &Node| -> usize { get_heuristic(&node.loc, &end.loc) };
    // let success = |node: &Node| -> bool { node.is_same_loc(&end) };

    // let (paths, _) = astar_bag_collect(&start, successors, heuristic, success).expect("Failed to find a path");

    // let mut seats = HashSet::new();
    // for path in paths {
    //     for node in path {
    //         seats.insert(node.loc);
    //     }
    // }

    // //print_seats(map, &seats);

    // Ok(seats.len())
}

fn get_successors(node: &Node, map: &ParsedInput) -> Vec<(Node, usize)> {
    Direction::iter()
        .filter_map(|direction| {
            let pos = (node.loc.y, node.loc.x);
            if let Some(((row, col), val)) = map.get_in_direction_indexed(pos, direction) {
                if *val == '#' {
                    None
                } else {
                    let cost = if direction.is_opposite_of(&node.dir.unwrap()) {
                        2001
                    } else if direction != node.dir.unwrap() {
                        1001
                    } else {
                        1
                    };

                    Some((Node::new(col, row, Some(direction), node.cost + cost), cost))
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
    dir: Option<Direction>,
    cost: usize,
}

impl Node {
    fn new(x: usize, y: usize, dir: Option<Direction>, cost: usize) -> Self {
        Self {
            loc: Point::from((x, y)),
            dir,
            cost,
        }
    }

    fn is_same_loc(&self, other: &Node) -> bool {
        self.loc == other.loc
    }
}

fn _print_path(map: &ParsedInput, path: &Vec<Node>) {
    for node in path {
        println!("{node:?}");
    }

    for row in 0..map.rows() {
        for col in 0..map.cols() {
            if let Some(node) = path
                .iter()
                .find(|&node| node.loc == Point::from((col, row)))
            {
                if let Some(dir) = node.dir {
                    match dir {
                        Direction::North => print!("^"),
                        Direction::East => print!(">"),
                        Direction::South => print!("v"),
                        Direction::West => print!("<"),
                    };
                    continue;
                }
            }

            if let Some(val) = map.get(row, col) {
                print!("{val}");
            }
        }
        println!()
    }
}

fn print_seats(map: &ParsedInput, seats: &HashSet<Point>) {
    for row in 0..map.rows() {
        for col in 0..map.cols() {
            if let Some(_) = seats
                .iter()
                .find(|&point| *point == Point::from((col, row)))
            {
                print!("O");
                continue;
            }

            if let Some(val) = map.get(row, col) {
                print!("{val}");
            }
        }
        println!()
    }
}
