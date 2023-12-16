use std::collections::HashMap;

use crate::error::AdventError;
use crate::util::grid::{
    Direction::{self, *},
    Grid,
};

type ParsedInput = Grid<char>;

pub fn parse(input: &str) -> color_eyre::Result<ParsedInput> {
    Ok(Grid::try_from(input)?)
}

pub fn part1(grid: &ParsedInput) -> color_eyre::Result<usize> {
    let beam = Beam {
        point: (0, 0),
        direction: East,
    };

    fire_beam(beam, grid)
}

pub fn part2(grid: &ParsedInput) -> color_eyre::Result<usize> {
    let mut max_energized = 0;

    for row in 0..grid.rows() {
        let beam = Beam {
            point: (row, 0),
            direction: East,
        };
        max_energized = max_energized.max(fire_beam(beam, grid)?);

        let beam = Beam {
            point: (row, grid.cols() - 1),
            direction: West,
        };
        max_energized = max_energized.max(fire_beam(beam, grid)?);
    }

    for col in 0..grid.cols() {
        let beam = Beam {
            point: (0, col),
            direction: South,
        };
        max_energized = max_energized.max(fire_beam(beam, grid)?);

        let beam = Beam {
            point: (grid.rows() - 1, col),
            direction: West,
        };
        max_energized = max_energized.max(fire_beam(beam, grid)?);
    }

    Ok(max_energized)
}

fn fire_beam(beam: Beam, grid: &ParsedInput) -> color_eyre::Result<usize> {
    let mut beams = vec![beam];
    let mut nodes = HashMap::<(usize, usize), Node>::new();

    while !beams.is_empty() {
        let mut beam = if let Some(beam) = beams.pop() {
            beam
        } else {
            break;
        };

        let node = nodes.entry(beam.point).or_insert_with(|| {
            let x = grid
                .get(beam.point.0, beam.point.1)
                .ok_or(AdventError::NotFound(format!(
                    "({}, {})",
                    beam.point.0, beam.point.1
                )))
                .unwrap();
            let mut node = Node::try_from(*x).unwrap();
            node.energized = true;
            node
        });

        if node.entered_dirs.contains(&beam.direction) {
            continue;
        }

        node.entered_dirs.push(beam.direction);

        match node.type_ {
            NodeType::Empty => (), // Don't change anyting about the beam
            NodeType::Mirror(mirror) => beam.direction = reflect(mirror, beam.direction)?,
            NodeType::Splitter(splitter) => split(splitter, &mut beam, &mut beams),
        }

        if let Some(next_point) = grid.get_in_direction(beam.point, beam.direction) {
            beam.point = next_point;
            beams.push(beam);
        }
    }

    let energized = nodes.iter().filter(|&(_, node)| node.energized).count();

    //println!("{energized}");

    Ok(energized)
}

fn reflect(mirror: char, incoming: Direction) -> color_eyre::Result<Direction> {
    Ok(match (mirror, incoming) {
        ('\\', North) => West,
        ('\\', East) => South,
        ('\\', South) => East,
        ('\\', West) => North,
        ('/', North) => East,
        ('/', East) => North,
        ('/', South) => West,
        ('/', West) => South,
        _ => {
            return Err(AdventError::UnknownPattern(format!("({},{:?})", mirror, incoming)).into())
        }
    })
}

fn split(splitter: char, incoming: &mut Beam, beams: &mut Vec<Beam>) {
    if splitter == '-' && matches!(incoming.direction, North | South) {
        beams.push(Beam {
            point: incoming.point,
            direction: West,
        });
        incoming.direction = East;
    } else if splitter == '|' && matches!(incoming.direction, East | West) {
        beams.push(Beam {
            point: incoming.point,
            direction: South,
        });
        incoming.direction = North
    }
}

#[derive(Debug)]
struct Beam {
    point: (usize, usize),
    direction: Direction,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum NodeType {
    Empty,
    Mirror(char),
    Splitter(char),
}

impl Default for NodeType {
    fn default() -> Self {
        Self::Empty
    }
}

#[derive(Debug)]
pub struct Node {
    energized: bool,
    type_: NodeType,
    entered_dirs: Vec<Direction>,
}

impl Node {
    pub fn new(type_: NodeType) -> Self {
        Self {
            energized: false,
            type_,
            entered_dirs: vec![],
        }
    }
}

impl TryFrom<char> for Node {
    type Error = AdventError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let type_ = match value {
            '.' => NodeType::Empty,
            '|' | '-' => NodeType::Splitter(value),
            '/' | '\\' => NodeType::Mirror(value),
            _ => return Err(AdventError::UnknownPattern(value.into())),
        };

        Ok(Self::new(type_))
    }
}
