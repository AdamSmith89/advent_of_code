use itertools::Itertools;

use std::collections::HashSet;

use crate::{
    error::AdventError,
    util::{direction::Direction, grid::Grid},
};

type ParsedInput = (Grid<char>, Vec<Direction>);

pub fn parse(input: &str) -> color_eyre::Result<ParsedInput> {
    let (map, directions) = input.split_once("\n\n").ok_or(AdventError::SplitOnce(
        input.to_string(),
        "double-newline".to_string(),
    ))?;

    let map = map.try_into()?;

    let directions = directions
        .chars()
        .filter_map(|ch| match ch {
            '^' => Some(Direction::North),
            '>' => Some(Direction::East),
            'v' => Some(Direction::South),
            '<' => Some(Direction::West),
            _ => None,
        })
        .collect_vec();

    Ok((map, directions))
}

pub fn part1((map, directions): &ParsedInput) -> color_eyre::Result<usize> {
    let mut map = map.clone();
    let (mut robot_pos, _) =
        map.indexed_iter()
            .find(|&(_, ch)| *ch == '@')
            .ok_or(AdventError::LogicError(
                "Failed to find robot in map".to_string(),
            ))?;

    for direction in directions {
        if let Some((next_pos, next_val)) = map.get_in_direction_indexed(robot_pos, *direction) {
            let can_move = match *next_val {
                '#' => false,
                '.' => true,
                'O' => move_small_box(next_pos, *direction, &mut map)?,
                _ => {
                    return Err(AdventError::LogicError(format!(
                        "Value at {next_pos:?} is {next_val} unexpectedly"
                    ))
                    .into())
                }
            };

            if can_move {
                // We know this is safe because we got
                unsafe {
                    let cur_val = map.get_unchecked_mut(robot_pos.0, robot_pos.1);
                    *cur_val = '.';
                    let next_val = map.get_unchecked_mut(next_pos.0, next_pos.1);
                    *next_val = '@';
                }

                robot_pos = next_pos;
            }
        } else {
            return Err(AdventError::LogicError(format!(
                "{direction} of {robot_pos:?} is out of bounds"
            ))
            .into());
        }
    }

    // The GPS coordinate of a box is equal to 100 times its distance from the top edge of the map
    // plus its distance from the left edge of the map.
    // What is the sum of all boxes' GPS coordinates?

    Ok(map
        .indexed_iter()
        .filter_map(|(pos, val)| {
            if *val == 'O' {
                Some(100 * pos.0 + pos.1)
            } else {
                None
            }
        })
        .sum())
}

pub fn part2((map, directions): &ParsedInput) -> color_eyre::Result<usize> {
    let mut map = expand_map(map)?;
    let (mut robot_pos, _) =
        map.indexed_iter()
            .find(|&(_, ch)| *ch == '@')
            .ok_or(AdventError::LogicError(
                "Failed to find robot in map".to_string(),
            ))?;

    for direction in directions {
        if let Some((next_pos, next_val)) = map.get_in_direction_indexed(robot_pos, *direction) {
            let move_robot = match *next_val {
                '#' => false,
                '.' => true,
                '[' | ']' => {
                    let (box_lhs, box_rhs) = if *next_val == '[' {
                        (next_pos, (next_pos.0, next_pos.1 + 1))
                    } else {
                        ((next_pos.0, next_pos.1 - 1), next_pos)
                    };

                    if let Some(boxes) = can_move_big_box(box_lhs, box_rhs, *direction, &mut map)? {
                        move_big_boxes(boxes, *direction, &mut map);
                        true
                    } else {
                        false
                    }
                }
                _ => {
                    return Err(AdventError::LogicError(format!(
                        "Value at {next_pos:?} is {next_val} unexpectedly"
                    ))
                    .into())
                }
            };

            if move_robot {
                // We know this is safe because we got the positions already above
                unsafe {
                    let cur_val = map.get_unchecked_mut(robot_pos.0, robot_pos.1);
                    *cur_val = '.';
                    let next_val = map.get_unchecked_mut(next_pos.0, next_pos.1);
                    *next_val = '@';
                }

                robot_pos = next_pos;
            }
        } else {
            return Err(AdventError::LogicError(format!(
                "{direction} of {robot_pos:?} is out of bounds"
            ))
            .into());
        }
    }

    // The GPS coordinate of a box is equal to 100 times its distance from the top edge of the map
    // plus its distance from the left edge of the map.
    // What is the sum of all boxes' GPS coordinates?

    Ok(map
        .indexed_iter()
        .filter_map(|(pos, val)| {
            if *val == '[' {
                Some(100 * pos.0 + pos.1)
            } else {
                None
            }
        })
        .sum())
}

fn move_small_box(
    box_pos: (usize, usize),
    direction: Direction,
    map: &mut Grid<char>,
) -> Result<bool, AdventError> {
    if let Some((next_pos, next_val)) = map.get_in_direction_indexed(box_pos, direction) {
        let can_move = match *next_val {
            '#' => Ok(false),
            '.' => Ok(true),
            'O' => move_small_box(next_pos, direction, map),
            _ => Err(AdventError::LogicError(format!(
                "Value at {next_pos:?} is {next_val} unexpectedly"
            ))),
        }?;

        if can_move {
            // We know this is safe because we got the positions already above
            unsafe {
                let cur_val = map.get_unchecked_mut(box_pos.0, box_pos.1);
                *cur_val = '.';
                let next_val = map.get_unchecked_mut(next_pos.0, next_pos.1);
                *next_val = 'O';
            }
        }

        return Ok(can_move);
    } else {
        return Err(AdventError::LogicError(format!(
            "{direction} of {box_pos:?} is out of bounds"
        )));
    }
}

fn can_move_big_box(
    box_lhs: (usize, usize),
    box_rhs: (usize, usize),
    direction: Direction,
    map: &mut Grid<char>,
) -> Result<Option<HashSet<((usize, usize), (usize, usize))>>, AdventError> {
    let (next_lhs_pos, next_lhs_val) =
        map.get_in_direction_indexed(box_lhs, direction)
            .ok_or(AdventError::LogicError(format!(
                "Failed to move {direction:?} from {box_lhs:?}"
            )))?;
    let next_lhs_val = next_lhs_val.clone();

    let (next_rhs_pos, next_rhs_val) =
        map.get_in_direction_indexed(box_rhs, direction)
            .ok_or(AdventError::LogicError(format!(
                "Failed to move {direction:?} from {box_rhs:?}"
            )))?;
    let next_rhs_val = next_rhs_val.clone();

    use Direction::*;

    match (next_lhs_val, next_rhs_val, direction) {
        (lhs, rhs, _) if lhs == '#' || rhs == '#' => Ok(None),
        ('.', '.', _) | (']', '.', East) | ('.', '[', West) => {
            Ok(Some(vec![(box_lhs, next_lhs_pos)].into_iter().collect()))
        }
        ('[', ']', _) => {
            let boxes = can_move_big_box(next_lhs_pos, next_rhs_pos, direction, map)?;
            Ok(boxes.map(|boxes| {
                let mut boxes = boxes.clone();
                boxes.insert((box_lhs, next_lhs_pos));
                boxes
            }))
        }
        (']', '[', East) => {
            let next_lhs_box = next_rhs_pos;
            let next_rhs_box = (next_rhs_pos.0, next_rhs_pos.1 + 1);

            let boxes = can_move_big_box(next_lhs_box, next_rhs_box, direction, map)?;
            Ok(boxes.map(|boxes| {
                let mut boxes = boxes.clone();
                boxes.insert((box_lhs, next_lhs_pos));
                boxes
            }))
        }
        (']', '[', West) => {
            let next_lhs_box = (next_lhs_pos.0, next_lhs_pos.1 - 1);
            let next_rhs_box = next_lhs_pos;

            let boxes = can_move_big_box(next_lhs_box, next_rhs_box, direction, map)?;
            Ok(boxes.map(|boxes| {
                let mut boxes = boxes.clone();
                boxes.insert((box_lhs, next_lhs_pos));
                boxes
            }))
        }
        (']', '.', North | South) | ('.', '[', North | South) | (']', '[', North | South) => {
            let lhs_boxes = if next_lhs_val == ']' {
                let next_rhs_box = next_lhs_pos;
                let next_lhs_box = (next_rhs_box.0, next_rhs_box.1 - 1);

                can_move_big_box(next_lhs_box, next_rhs_box, direction, map)?
            } else {
                Some(HashSet::new())
            };

            let rhs_boxes = if next_rhs_val == '[' {
                let next_lhs_box = next_rhs_pos;
                let next_rhs_box = (next_lhs_box.0, next_lhs_box.1 + 1);

                can_move_big_box(next_lhs_box, next_rhs_box, direction, map)?
            } else {
                Some(HashSet::new())
            };

            Ok(
                if let Some(rhs_boxes) = rhs_boxes.map(|rhs_boxes| {
                    let mut rhs_boxes = rhs_boxes.clone();
                    rhs_boxes.insert((box_lhs, next_lhs_pos));
                    rhs_boxes
                }) {
                    lhs_boxes.map(|lhs_boxes| {
                        let mut lhs_boxes = lhs_boxes.clone();
                        lhs_boxes.extend(rhs_boxes);
                        lhs_boxes
                    })
                } else {
                    None
                },
            )
        }
        next => Err(AdventError::UnknownPattern(format!("{next:?}"))),
    }
}

fn move_big_boxes(
    boxes: HashSet<((usize, usize), (usize, usize))>,
    direction: Direction,
    map: &mut Grid<char>,
) {
    use Direction::*;
    let mut boxes = boxes.iter().collect_vec();
    match direction {
        North | South => {
            boxes.sort_by(|lhs, rhs| lhs.0.cmp(&rhs.0));
            if direction == South {
                boxes.reverse();
            }
        }
        East | West => {
            boxes.sort_by(|lhs, rhs| lhs.1.cmp(&rhs.1));
            if direction == East {
                boxes.reverse();
            }
        }
    }

    for (box_from, box_to) in boxes {
        let box_from_lhs = box_from;
        let box_from_rhs = (box_from.0, box_from.1 + 1);

        let box_to_lhs = box_to;
        let box_to_rhs = (box_to.0, box_to.1 + 1);

        // We know this is safe because we got the positions already above
        unsafe {
            let cur_lhs_val = map.get_unchecked_mut(box_from_lhs.0, box_from_lhs.1);
            if *cur_lhs_val == '[' {
                *cur_lhs_val = '.';
            }
            let next_lhs_val = map.get_unchecked_mut(box_to_lhs.0, box_to_lhs.1);
            *next_lhs_val = '[';

            let cur_rhs_val = map.get_unchecked_mut(box_from_rhs.0, box_from_rhs.1);
            if *cur_rhs_val == ']' {
                *cur_rhs_val = '.';
            }
            let next_rhs_val = map.get_unchecked_mut(box_to_rhs.0, box_to_rhs.1);
            *next_rhs_val = ']';
        }
    }
}

fn expand_map(map: &Grid<char>) -> Result<Grid<char>, AdventError> {
    let mut new_map = Grid::init(map.rows(), map.cols() * 2, '.');

    for row_idx in 0..map.rows() {
        let mut new_row_iter = new_map.iter_row_mut(row_idx);

        for row in map.iter_row(row_idx) {
            match *row {
                '#' => {
                    let next = new_row_iter.next().ok_or(AdventError::EndOfIterator)?;
                    *next = '#';
                    let next = new_row_iter.next().ok_or(AdventError::EndOfIterator)?;
                    *next = '#';
                }
                '.' => {
                    new_row_iter.next().ok_or(AdventError::EndOfIterator)?;
                    new_row_iter.next().ok_or(AdventError::EndOfIterator)?;
                }
                '@' => {
                    let next = new_row_iter.next().ok_or(AdventError::EndOfIterator)?;
                    *next = '@';
                    new_row_iter.next().ok_or(AdventError::EndOfIterator)?;
                }
                'O' => {
                    let next = new_row_iter.next().ok_or(AdventError::EndOfIterator)?;
                    *next = '[';
                    let next = new_row_iter.next().ok_or(AdventError::EndOfIterator)?;
                    *next = ']';
                }
                val => {
                    return Err(AdventError::UnexpectedValue(
                        "#|.|@|O".to_string(),
                        val.to_string(),
                    ))
                }
            }
        }
    }

    Ok(new_map)
}
