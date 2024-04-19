use log::debug;
use std::{
    collections::{HashMap, HashSet},
    ops::Range,
};

use crate::{
    error::AdventError,
    util::{
        grid::{Direction, Grid},
        point::PointT,
    },
};

use super::int_code_computer::{IcProgram, IntCodeComputer};

type ParsedInput = IcProgram;
const BLACK: char = '.';
const WHITE: char = '#';

pub fn parse(input: &str) -> color_eyre::Result<ParsedInput> {
    IntCodeComputer::parse_program(input)
}

pub fn part1(code: &ParsedInput) -> color_eyre::Result<usize> {
    let mut icc = IntCodeComputer::load(code.clone());
    let mut cur_dir = Direction::North;
    let mut cur_pos = PointT::from((0, 0));
    let mut panels = HashMap::from([(cur_pos, '.')]);
    let mut painted_panels = HashSet::new();

    // Starting panel is BLACK
    icc.push_input(0);
    icc.enable_input_yield();

    while !icc.has_halted() {
        let icc_status = icc.run();
        if !icc.is_yielding() {
            if let Err(report) = icc_status {
                return Err(report.wrap_err("ICC failed unexpectedly"));
            }
        }

        if paint_panel(&mut icc, &mut panels, cur_pos)? {
            painted_panels.insert(cur_pos);
        }

        move_robot(&mut icc, &mut cur_dir, &mut cur_pos, &mut panels);
        input_panel_to_robot(&cur_pos, &mut panels, &mut icc)?;

        debug!("");
    }

    debug!("Painted {} panels at least once", painted_panels.len());
    Ok(painted_panels.len())
}

pub fn part2(code: &ParsedInput) -> color_eyre::Result<&str> {
    let mut icc = IntCodeComputer::load(code.clone());
    let mut cur_dir = Direction::North;
    let mut cur_pos = PointT::from((0, 0));
    let mut panels = HashMap::from([(cur_pos, '.')]);

    // Starting panel is WHITE
    icc.push_input(1);
    icc.enable_input_yield();

    while !icc.has_halted() {
        let icc_status = icc.run();
        if !icc.is_yielding() {
            if let Err(report) = icc_status {
                return Err(report.wrap_err("ICC failed unexpectedly"));
            }
        }

        paint_panel(&mut icc, &mut panels, cur_pos)?;
        move_robot(&mut icc, &mut cur_dir, &mut cur_pos, &mut panels);
        input_panel_to_robot(&cur_pos, &mut panels, &mut icc)?;

        debug!("");
    }

    let mut min_x = 0;
    let mut min_y = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    for point in panels.keys() {
        min_x = min_x.min(point.x);
        min_y = min_y.min(point.y);

        max_x = max_x.max(point.x);
        max_y = max_y.max(point.y);
    }

    let width = (max_x - min_x) + 1;
    let height = (max_y - min_y) + 1;
    debug!("Creating grid. Width={width}, Height={height}");

    let mut grid = Grid::init(height as usize, width as usize, '.');
    for (point, colour) in panels {
        let new_x = shift_value_in_range(point.x, min_x..max_x + 1, 0..width);
        let new_y = shift_value_in_range(point.y, min_y..max_y + 1, 0..height);

        let panel = grid
            .get_mut(new_y as usize, new_x as usize)
            .ok_or(AdventError::LogicError(format!(
                "Failed to find {point} in grid",
            )))?;

        *panel = colour;
    }

    // Actual answer is what gets printed to the screen
    debug!("{grid:?}");
    Ok("ZRZPKEZR")
}

fn paint_panel(
    icc: &mut IntCodeComputer,
    panels: &mut HashMap<PointT<i32>, char>,
    cur_pos: PointT<i32>,
) -> color_eyre::Result<bool> {
    // First, it will output a value indicating the color to paint the panel the robot is over:
    // 0 means to paint the panel black, and
    // 1 means to paint the panel white.
    if let Some(output) = icc.next_output() {
        let cur_panel = panels
            .get_mut(&cur_pos)
            .ok_or(AdventError::LogicError(String::from("Cur Pos not stored")))?;

        if output == 0 {
            debug!("Painting {cur_pos} BLACK");
            *cur_panel = BLACK;
        } else if output == 1 {
            debug!("Painting {cur_pos} WHITE");
            *cur_panel = WHITE;
        }

        Ok(true)
    } else {
        Ok(false)
    }
}

fn move_robot(
    icc: &mut IntCodeComputer,
    cur_dir: &mut Direction,
    cur_pos: &mut PointT<i32>,
    panels: &mut HashMap<PointT<i32>, char>,
) {
    // Second, it will output a value indicating the direction the robot should turn:
    // 0 means it should turn left 90 degrees, and
    // 1 means it should turn right 90 degrees.
    if let Some(output) = icc.next_output() {
        if output == 0 {
            *cur_dir = cur_dir.rotate_90_c_cwise();
            debug!("Rotation c-cwise to {cur_dir}");
        } else if output == 1 {
            *cur_dir = cur_dir.rotate_90_cwise();
            debug!("Rotation cwise to {cur_dir}");
        }

        // After the robot turns, it should always move forward exactly one panel.
        match cur_dir {
            Direction::North => cur_pos.y -= 1,
            Direction::East => cur_pos.x += 1,
            Direction::South => cur_pos.y += 1,
            Direction::West => cur_pos.x -= 1,
        }
        debug!("Moved to {cur_pos}");

        // If this isn't a previously visited panel than insert a new BLACK panel
        panels.entry(*cur_pos).or_insert(BLACK);
    }
}

fn input_panel_to_robot(
    cur_pos: &PointT<i32>,
    panels: &mut HashMap<PointT<i32>, char>,
    icc: &mut IntCodeComputer,
) -> color_eyre::Result<()> {
    // The program uses input instructions to access the robot's camera, provide:
    // 0 if the robot is over a black panel (.) or
    // 1 if the robot is over a white panel (#).

    let cur_panel = panels
        .get(cur_pos)
        .ok_or(AdventError::LogicError(String::from("Cur Pos not stored")))?;

    if *cur_panel == BLACK {
        debug!("Panel at {cur_pos} is BLACK, pushing 0...");
        icc.push_input(0);
    } else if *cur_panel == WHITE {
        debug!("Panel at {cur_pos} is WHITE, pushing 0...");
        icc.push_input(1);
    }

    Ok(())
}

fn shift_value_in_range(old_value: i32, old_range: Range<i32>, new_range: Range<i32>) -> i32 {
    (((old_value - old_range.start) * new_range.len() as i32) / old_range.len() as i32)
        + new_range.start
}
