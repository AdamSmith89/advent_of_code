use std::collections::HashSet;

use crate::{
    error::AdventError,
    util::{direction::Direction, grid::Grid},
};

type ParsedInput = Grid<char>;

pub fn parse(input: &str) -> color_eyre::Result<ParsedInput> {
    Ok(input.try_into()?)
}

pub fn part1(map: &ParsedInput) -> color_eyre::Result<usize> {
    let regions = build_regions(map);

    Ok(regions
        .iter()
        .map(|region| region.points.len() * region.perimeter)
        .sum())
}

pub fn part2(map: &ParsedInput) -> color_eyre::Result<usize> {
    let regions = build_regions(map);

    let mut total_cost = 0;

    for region in regions {
        let (top_sides, bottom_sides) = get_top_bottom_sides(&region, map)?;
        let (left_sides, right_sides) = get_left_right_sides(&region, map)?;
        let total_sides = top_sides + bottom_sides + left_sides + right_sides;

        total_cost += region.points.len() * total_sides;
    }

    Ok(total_cost)
}

fn build_regions(map: &Grid<char>) -> Vec<Region> {
    let mut visited = HashSet::new();
    let mut map_iter = map.indexed_iter();

    let mut regions = Vec::new();

    while let Some((loc, plant)) = map_iter.find(|(loc, _)| !visited.contains(loc)) {
        let mut region = Region::new(*plant);
        region.add_point(&loc);

        flood_fill(map, loc, *plant, &mut region);

        for loc in &region.points {
            visited.insert(*loc);
        }

        region.points.sort();

        regions.push(region);
    }
    regions
}

fn flood_fill(
    map: &Grid<char>,
    start_loc: (usize, usize),
    plant: char,
    region: &mut Region,
) {
    let mut borders: Vec<(usize, Direction)> = Vec::new();
    let mut this_loc_peri = 4;

    for (direction, neighbour) in map.get_neighbours_of_indexed(start_loc) {
        if neighbour.is_none() {
            match direction {
                Direction::North | Direction::South => borders.push((start_loc.0, direction)),
                Direction::East | Direction::West => borders.push((start_loc.1, direction)),
            }
        }

        if let Some((neighbour_loc, neighbour_plant)) = neighbour {
            if *neighbour_plant == plant {
                this_loc_peri -= 1;
                if !region.points.contains(&neighbour_loc) {
                    region.add_point(&neighbour_loc);

                    flood_fill(map, neighbour_loc, plant, region);
                }
            }
        }
    }

    region.perimeter += this_loc_peri;
}

fn get_top_bottom_sides(region: &Region, map: &Grid<char>) -> color_eyre::Result<(usize, usize)> {
    let mut top_sides = 0;
    let mut bottom_sides = 0;

    let top_most = region
        .top_most
        .ok_or(AdventError::LogicError("No top_most value".to_string()))?;
    let bottom_most = region
        .bottom_most
        .ok_or(AdventError::LogicError("No bottom_most value".to_string()))?;

    for row in top_most..=bottom_most {
        let mut last_top_col = None;
        let mut last_bottom_col = None;

        region
            .points
            .iter()
            .filter(|(reg_row, _)| *reg_row == row)
            .for_each(|(row, col)| {
                if map
                    .get_in_direction((*row, *col), Direction::North)
                    .is_none_or(|north_plant| *north_plant != region.plant)
                {
                    if last_top_col.is_none_or(|last_col| last_col != col - 1) {
                        top_sides += 1;
                    }

                    last_top_col = Some(*col);
                }

                if map
                    .get_in_direction((*row, *col), Direction::South)
                    .is_none_or(|south_plant| *south_plant != region.plant)
                {
                    if last_bottom_col.is_none_or(|last_col| last_col != col - 1) {
                        bottom_sides += 1;
                    }

                    last_bottom_col = Some(*col);
                }
            });
    }

    Ok((top_sides, bottom_sides))
}

fn get_left_right_sides(region: &Region, map: &Grid<char>) -> color_eyre::Result<(usize, usize)> {
    let left_most = region
        .left_most
        .ok_or(AdventError::LogicError("No left_most value".to_string()))?;
    let right_most = region
        .right_most
        .ok_or(AdventError::LogicError("No right_most value".to_string()))?;

    let mut left_sides = 0;
    let mut right_sides = 0;

    for col in left_most..=right_most {
        let mut last_left_row = None;
        let mut last_right_row = None;

        region
            .points
            .iter()
            .filter(|(_, reg_col)| *reg_col == col)
            .for_each(|(row, col)| {
                if map
                    .get_in_direction((*row, *col), Direction::West)
                    .is_none_or(|west_plant| *west_plant != region.plant)
                {
                    if last_left_row.is_none_or(|last_row| last_row != row - 1) {
                        left_sides += 1;
                    }

                    last_left_row = Some(*row);
                }

                if map
                    .get_in_direction((*row, *col), Direction::East)
                    .is_none_or(|east_plant| *east_plant != region.plant)
                {
                    if last_right_row.is_none_or(|last_row| last_row != row - 1) {
                        right_sides += 1;
                    }

                    last_right_row = Some(*row);
                }
            });
    }

    Ok((left_sides, right_sides))
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Region {
    plant: char,
    points: Vec<(usize, usize)>,
    top_most: Option<usize>,
    bottom_most: Option<usize>,
    left_most: Option<usize>,
    right_most: Option<usize>,
    perimeter: usize,
}

impl Region {
    pub fn new(plant: char) -> Self {
        Self {
            plant,
            points: Vec::new(),
            top_most: None,
            bottom_most: None,
            left_most: None,
            right_most: None,
            perimeter: 0,
        }
    }

    fn add_point(&mut self, new_point: &(usize, usize)) {
        self.top_most = Some(
            self.top_most
                .map_or(new_point.0, |top_most| top_most.min(new_point.0)),
        );

        self.bottom_most = Some(
            self.bottom_most
                .map_or(new_point.0, |bottom_most| bottom_most.max(new_point.0)),
        );

        self.left_most = Some(
            self.left_most
                .map_or(new_point.1, |left_most| left_most.min(new_point.1)),
        );

        self.right_most = Some(
            self.right_most
                .map_or(new_point.1, |right_most| right_most.max(new_point.1)),
        );

        self.points.push(*new_point);
    }
}
