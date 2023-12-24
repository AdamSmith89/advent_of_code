use std::{collections::HashSet, ops::Range};

use itertools::Itertools;

use crate::error::AdventError;

type ParsedInput = Vec<Brick>;

pub fn parse(input: &str) -> color_eyre::Result<ParsedInput> {
    let mut bricks: Vec<Brick> = input.lines().map(Brick::try_from).try_collect()?;
    bricks.sort_unstable_by(Brick::compare_z);

    for i in 0..bricks.len() {
        bricks[i].id = i as u32 + 1;
    }

    // Simulate the bricks falling
    for i in 0..bricks.len() {
        let mut min_z = 1;

        for j in 0..i {
            if bricks[i].overlaps(&bricks[j]) {
                min_z = min_z.max(bricks[j].max_z() + 1);
            }
        }

        let height = bricks[i].height();
        bricks[i].z = min_z..min_z + height;
    }

    // Collect all bricks directly above/below
    for brick_idx in 0..bricks.len() {
        let brick_id = bricks[brick_idx].id;

        for other_idx in 0..brick_idx {
            let other_id = bricks[other_idx].id;

            if bricks[brick_idx].overlaps(&bricks[other_idx])
                && bricks[brick_idx].min_z() == bricks[other_idx].max_z() + 1
            {
                bricks[brick_idx].add_below(other_id);
                bricks[other_idx].add_above(brick_id);
            }
        }
    }

    Ok(bricks)
}

pub fn part1(bricks: &ParsedInput) -> color_eyre::Result<usize> {
    // Critical bricks are those which are the only one supporting a higher brick
    let critical_bricks: HashSet<u32> = bricks
        .iter()
        .filter_map(|brick| {
            if brick.below.len() == 1 {
                Some(brick.below[0])
            } else {
                None
            }
        })
        .collect();

    // Every other brick can be disintegrated
    let removable_bricks = bricks.len() - critical_bricks.len();

    Ok(removable_bricks)
}

pub fn part2(bricks: &ParsedInput) -> color_eyre::Result<usize> {
    let critical_bricks: HashSet<u32> = bricks
        .iter()
        .filter_map(|brick| {
            if brick.below.len() == 1 {
                Some(brick.below[0])
            } else {
                None
            }
        })
        .collect();

    let mut todo = Vec::new();
    let mut removed_bricks = HashSet::<u32>::new();
    let mut total = 0;

    for brick_id in critical_bricks {
        todo.push(brick_id);
        removed_bricks.insert(brick_id);

        while let Some(current) = todo.pop() {
            let brick = bricks
                .iter()
                .find(|brick| brick.id == current)
                .ok_or(AdventError::NotFound(format!("{}", brick_id)))?;

            for above_id in &brick.above {
                let above_brick = bricks
                    .iter()
                    .find(|brick| brick.id == *above_id)
                    .ok_or(AdventError::NotFound(format!("{}", brick_id)))?;

                if above_brick.below.iter().all(|below_id| removed_bricks.contains(below_id)) {
                    todo.push(*above_id);
                    removed_bricks.insert(*above_id);
                }
            }
        }

        total += removed_bricks.len() - 1;
        removed_bricks.clear();
    }

    Ok(total)
}

#[derive(Clone, Debug, PartialEq)]
pub struct Brick {
    pub id: u32,
    pub x: Range<u32>,
    pub y: Range<u32>,
    pub z: Range<u32>,
    pub above: Vec<u32>,
    pub below: Vec<u32>,
}

impl Brick {
    fn add_above(&mut self, brick_id: u32) {
        self.above.push(brick_id);
    }

    fn add_below(&mut self, brick_id: u32) {
        self.below.push(brick_id);
    }

    fn compare_z(a: &Self, b: &Self) -> std::cmp::Ordering {
        a.z.start.cmp(&b.z.start)
    }

    fn height(&self) -> u32 {
        self.z.len() as u32
    }

    fn min_z(&self) -> u32 {
        self.z.start
    }

    fn max_z(&self) -> u32 {
        self.z.end - 1
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.x.overlaps(&other.x) && self.y.overlaps(&other.y)
    }
}

trait RangeEx {
    fn overlaps(&self, other: &Range<u32>) -> bool;
}

impl RangeEx for Range<u32> {
    fn overlaps(&self, other: &Range<u32>) -> bool {
        /*
        (StartA <= EndB) and (EndA >= StartB)

        |--A--|
                |--B--|

                |--A--|
        |--B--|

        |--A--|
            |--B--|

            |--A--|
        |--B--|

        |----A----|
          |--B--|

        |----B----|
          |--A--|

        |--A--|
        |--B--|
         */

        // -1 off the end because this is an exclusive range
        self.start <= (other.end - 1) && (self.end - 1) >= other.start
    }
}

impl TryFrom<&str> for Brick {
    type Error = AdventError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // 1,2,3~4,5,6

        let (start, end) = value
            .split_once('~')
            .ok_or(AdventError::SplitOnce(value.into(), '~'.into()))?;

        let start: Vec<u32> = start.split(',').map(|s| s.parse::<u32>()).try_collect()?;
        let (s_x, s_y, s_z) = start
            .iter()
            .collect_tuple()
            .ok_or(AdventError::NotFound(value.into()))?;

        let end: Vec<u32> = end.split(',').map(|s| s.parse::<u32>()).try_collect()?;
        let (e_x, e_y, e_z) = end
            .iter()
            .collect_tuple()
            .ok_or(AdventError::NotFound(value.into()))?;

        Ok(Brick {
            id: 0,
            x: *s_x..*e_x + 1,
            y: *s_y..*e_y + 1,
            z: *s_z..*e_z + 1,
            above: Vec::new(),
            below: Vec::new(),
        })
    }
}
