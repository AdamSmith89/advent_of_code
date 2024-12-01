use std::{
    collections::{HashMap, HashSet},
    f64::consts::PI,
};

use itertools::Itertools;
use log::{debug, info};

use crate::{
    error::AdventError,
    util::{grid::Grid, point::Point},
};

type ParsedInput = Grid<char>;

pub fn parse(input: &str) -> color_eyre::Result<ParsedInput> {
    let mut grid = Grid::new(0, 0);

    for line in input.lines() {
        grid.push_row(line.chars().collect());
    }

    Ok(grid)
}

pub fn part1(grid: &ParsedInput) -> color_eyre::Result<usize> {
    // This is an Euclid's orchard problem - https://en.wikipedia.org/wiki/Euclid%27s_orchard
    // Should be able to work out the gradient of each line between asteroids
    // and the number of unique gradients from a given asteroid is the number
    // of other asteroids it can see

    let (_, num_visible) = find_asteroid_with_largest_visible_set(grid)?;

    Ok(num_visible)
}

pub fn part2(grid: &ParsedInput) -> color_eyre::Result<usize> {
    // The Elves are placing bets on which will be the 200th asteroid to be vaporized.
    // Win the bet by determining which asteroid that will be;
    // what do you get if you multiply its X coordinate by 100 and then add its Y coordinate?
    // (For example, 8,2 becomes 802.)

    // My approach:
    // Get visible set for origin
    // Order by angle
    // Remove each asteroid from the grid (in-order of angle)
    // When 200th is removed then that is the answer

    // Origin is the asteroid with the most visible asteroids from part 1
    let (origin, _) = find_asteroid_with_largest_visible_set(grid)?;
    let mut grid = grid.clone();
    let mut asteroids_zapped = 0;

    let asteroid_200 = 'outer: loop {
        let asteroids = find_asteroids(&grid);

        // Map of unique angle to (asteroid and distance from origin)
        let mut visible_asteroids: HashMap<u64, (Point, u64, f64)> = HashMap::new();
        for asteroid in asteroids {
            if asteroid == origin {
                continue;
            }

            let angle = calc_angle_between_points(&origin, &asteroid);
            let angle_bits = angle.to_bits(); // Convert to bits so can use as key in HashMap
            let dist = calc_manhattan_distance(&origin, &asteroid);

            visible_asteroids
                .entry(angle_bits)
                .and_modify(|(p, d, a)| {
                    if dist < *d {
                        *p = asteroid;
                        *d = dist;
                        *a = angle;
                    }
                })
                .or_insert((asteroid, dist, angle));
        }

        // Sorting by the bits feels weird but seems to work - can't compare the actual angle as
        // f64 can't be compared
        for (_, (asteroid, _, _)) in visible_asteroids
            .into_iter()
            .sorted_by(|(bits1, _), (bits2, _)| bits1.cmp(bits2))
        {
            let raw_value = grid
                .get_mut(asteroid.y, asteroid.x)
                .ok_or(AdventError::LogicError(String::from(
                    "Couldn't find known asteroid in grid",
                )))?;

            // Remove this asteroid from the grid for next iteration
            *raw_value = '.';
            asteroids_zapped += 1;
            debug!("Zapping {asteroid:?} [{asteroids_zapped}]");

            if asteroids_zapped == 200 {
                break 'outer asteroid;
            }
        }
    };

    Ok((asteroid_200.x * 100) + asteroid_200.y)
}

fn find_asteroid_with_largest_visible_set(grid: &Grid<char>) -> color_eyre::Result<(Point, usize)> {
    let asteroids = find_asteroids(grid);
    let mut asteroid_angles: HashMap<Point, HashSet<u64>> = HashMap::new();

    for permutation in asteroids.iter().permutations(2) {
        let p_one = permutation[0];
        let p_two = permutation[1];
        let angle = calc_angle_between_points(p_one, p_two);

        // Convert to bits so can store in HashSet to remove duplicates
        let angle_bits = angle.to_bits();

        asteroid_angles
            .entry(*p_one)
            .or_default()
            .insert(angle_bits);
    }

    if let Some((asteroid, visible_set)) = asteroid_angles
        .iter()
        .max_by(|&(_, lhs_set), &(_, rhs_set)| lhs_set.len().cmp(&rhs_set.len()))
    {
        info!(
            "Asteroid {asteroid} has the highest visible set with {} other asteroids visible",
            visible_set.len()
        );
        Ok((*asteroid, visible_set.len()))
    } else {
        Err(AdventError::LogicError(String::from("No asteroids in list")).into())
    }
}

fn find_asteroids(grid: &Grid<char>) -> Vec<Point> {
    grid.indexed_iter()
        .filter_map(|((row, col), &object)| {
            if object == '#' {
                Some(Point { x: col, y: row })
            } else {
                None
            }
        })
        .collect_vec()
}

fn calc_angle_between_points(p_one: &Point, p_two: &Point) -> f64 {
    // Angle of the line between two points is:
    // atan2(y2 - y1, x2 - x1) * 180 / PI;
    let y_diff = p_two.y as f64 - p_one.y as f64;
    let x_diff = p_two.x as f64 - p_one.x as f64;
    let angle = y_diff.atan2(x_diff) * 180.0 / PI;

    // Convert from -180-180 into 0-360 range (useful for Part 2)
    (angle + 450f64) % 360f64
}

fn calc_manhattan_distance(from: &Point, to: &Point) -> u64 {
    let x_diff = to.x.abs_diff(from.x) as u64;
    let y_diff = to.y.abs_diff(from.y) as u64;

    x_diff + y_diff
}
