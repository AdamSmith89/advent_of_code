use itertools::Itertools;
use std::collections::HashMap;

use crate::error::AdventError;

type ParsedInput = Vec<String>;

pub fn parse(input: &str) -> color_eyre::Result<ParsedInput> {
    Ok(input.split(',').map_into().collect_vec())
}

pub fn part1(steps: &ParsedInput) -> color_eyre::Result<u32> {
    // Determine the ASCII code for the current character of the string.
    // Increase the current value by the ASCII code you just determined.
    // Set the current value to itself multiplied by 17.
    // Set the current value to the remainder of dividing itself by 256.

    let mut total: u32 = 0;
    for step in steps {
        total += hash(step);
    }

    Ok(total)
}

pub fn part2(steps: &ParsedInput) -> color_eyre::Result<u32> {
    let mut boxes = HashMap::<u32, Vec<Lens>>::new();

    for step in steps {
        let mut label = "";
        let mut op = ' ';
        let mut focal_length = None;
        if let Some((s, o)) = step.split_once('=') {
            label = s;
            op = '=';
            focal_length = Some(o.parse::<u32>().map_err(AdventError::ParseInt)?);
        } else if let Some((s, _)) = step.split_once('-') {
            label = s;
            op = '-';
        }

        let box_num = hash(label);
        if op == '-' {
            if let Some(lenses) = boxes.get_mut(&box_num) {
                if let Some(idx) = lenses.iter().position(|lens| lens.label == label) {
                    lenses.remove(idx);
                }
            }
        } else if op == '=' {
            let lens = Lens {
                label: label.into(),
                focal_length: focal_length.ok_or(AdventError::NotFound(step.into()))?,
            };

            boxes
                .entry(box_num)
                .and_modify(|lenses| {
                    if let Some(idx) = lenses.iter().position(|lens| lens.label == label) {
                        lenses[idx] = lens.clone();
                    } else {
                        lenses.push(lens.clone());
                    }
                })
                .or_insert(vec![lens]);
        }
    }

    let focusing_power = boxes.iter().map(box_focusing_power).sum::<u32>();

    Ok(focusing_power)
}

fn box_focusing_power((box_num, lenses): (&u32, &Vec<Lens>)) -> u32 {
    let lens_focusing_power = |(lens_idx, lens): (usize, &Lens)| {
        // One plus the box number of the lens in question.
        // The slot number of the lens within the box: 1 for the first lens, 2 for the second lens, and so on.
        // The focal length of the lens.
        (1 + box_num) * (lens_idx as u32 + 1) * lens.focal_length
    };

    lenses
        .iter()
        .enumerate()
        .map(lens_focusing_power)
        .sum::<u32>()
}

fn hash(s: &str) -> u32 {
    let mut cur_value = 0;
    for ch in s.chars() {
        cur_value += ch as u32;
        cur_value *= 17;
        cur_value %= 256;
    }
    cur_value
}

#[derive(Clone, Debug)]
struct Lens {
    label: String,
    focal_length: u32,
}
