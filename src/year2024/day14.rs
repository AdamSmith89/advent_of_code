use itertools::Itertools;
use log::debug;
use strum::IntoEnumIterator;

use crate::{
    error::AdventError,
    util::{direction::DirectionEx, grid::Grid, point::PointSig},
};

type ParsedInput = (Vec<Guard>, isize, isize);

pub fn parse(input: &str) -> color_eyre::Result<ParsedInput> {
    let mut lines = input.lines().peekable();

    // The robots outside the actual bathroom are in a space which is 101 tiles wide and 103 tiles tall (when viewed from above).
    // However, in this example, the robots are in a space which is only 11 tiles wide and 7 tiles tall.
    let (width, height) = if lines.peek().is_some_and(|&line| line == "example") {
        lines.next();
        (11, 7)
    } else {
        (101, 103)
    };

    Ok((lines.map(Guard::try_from).try_collect()?, width, height))
}

pub fn part1((guards, width, height): &ParsedInput) -> color_eyre::Result<u32> {
    // Where will the robots be after 100 seconds?
    // count the number of robots in each quadrant
    // Robots that are exactly in the middle (horizontally or vertically) don't count as being in any quadrant

    let final_locs = guards
        .iter()
        .map(|guard| project_guard(guard, 100, *width, *height))
        .collect_vec();

    let mid_x = width / 2;
    let mid_y = height / 2;

    let mut top_left = 0;
    let mut top_right = 0;
    let mut bottom_left = 0;
    let mut bottom_right = 0;

    for loc in final_locs {
        if (0..mid_x).contains(&loc.x) && (0..mid_y).contains(&loc.y) {
            top_left += 1;
        } else if (0..mid_x).contains(&loc.x) && (mid_y + 1..*height).contains(&loc.y) {
            bottom_left += 1;
        } else if (mid_x + 1..*width).contains(&loc.x) && (0..mid_y).contains(&loc.y) {
            top_right += 1;
        } else if (mid_x + 1..*width).contains(&loc.x) && (mid_y + 1..*height).contains(&loc.y) {
            bottom_right += 1;
        }
    }

    Ok(top_left * top_right * bottom_left * bottom_right)
}

pub fn part2((guards, width, height): &ParsedInput) -> color_eyre::Result<u32> {
    let mut guards = guards.clone();
    let mut grid = Grid::init(*height as usize, *width as usize, 0);
    let mut found_after = 0;

    'seconds: for seconds in 1..=10000 {
        for guard in guards.iter_mut() {
            if let Some(n) = grid.get_mut(guard.pos.y as usize, guard.pos.x as usize) {
                if *n > 0 {
                    *n -= 1;
                }
            }

            guard.pos = project_guard(&guard, 1, *width, *height);

            if let Some(n) = grid.get_mut(guard.pos.y as usize, guard.pos.x as usize) {
                *n += 1;
            }
        }

        'guard: for guard in &guards {
            // If a guard is completely surrounded by other guards then we've hit
            // the christmas tree shape

            for direction in DirectionEx::iter() {
                let point = (guard.pos.y as usize, guard.pos.x as usize);
                if grid
                    .get_in_direction_ex(point, direction)
                    .is_none_or(|n| *n == 0)
                {
                    continue 'guard;
                }
            }

            debug!("At {seconds} - {guard:?}");
            debug!("{grid:?}");

            found_after = seconds;
            break 'seconds;
        }
    }

    Ok(found_after)
}

fn project_guard(guard: &Guard, num_steps: isize, width: isize, height: isize) -> PointSig {
    // Linear algorithm: Pn = P0 + n * v
    //  Pn -> position of point after n steps
    //  P0 -> start position
    //  v -> vector of movement
    //  n -> number of steps
    // Apply the modulo operation using rem_euclid to wrap around the grid.
    //  rem_euclid method is used instead of % because it handles negative values correctly

    let final_x = (guard.pos.x + num_steps * guard.vel.x).rem_euclid(width);
    let final_y = (guard.pos.y + num_steps * guard.vel.y).rem_euclid(height);

    (final_x, final_y).into()
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Guard {
    pos: PointSig,
    vel: PointSig,
}

impl Guard {
    pub fn new(pos: PointSig, vel: PointSig) -> Self {
        Self { pos, vel }
    }
}

impl TryFrom<&str> for Guard {
    type Error = AdventError;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let (pos, vel) = line
            .split_once(' ')
            .ok_or(AdventError::SplitOnce(line.to_string(), ' '.to_string()))?;

        let pos = &pos[2..];
        let (pos_x, pos_y) = pos
            .split_once(',')
            .ok_or(AdventError::SplitOnce(pos.to_string(), ','.to_string()))?;
        let pos = (pos_x.parse::<isize>()?, pos_y.parse::<isize>()?).into();

        let vel = &vel[2..];
        let (vel_x, vel_y) = vel
            .split_once(',')
            .ok_or(AdventError::SplitOnce(vel.to_string(), ','.to_string()))?;
        let vel = (vel_x.parse::<isize>()?, vel_y.parse::<isize>()?).into();

        Ok(Guard { pos, vel })
    }
}
