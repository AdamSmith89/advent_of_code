use strum::IntoEnumIterator;

use crate::util::{direction::DirectionEx, grid::Grid};

type ParsedInput = Grid<char>;

pub fn parse(input: &str) -> color_eyre::Result<ParsedInput> {
    Ok(input.try_into()?)
}

pub fn part1(wordsearch: &ParsedInput) -> color_eyre::Result<u32> {
    let mut xmas_count = 0;

    for (x_loc, x) in wordsearch.indexed_iter() {
        if *x != 'X' {
            continue;
        }

        for dir in DirectionEx::iter() {
            if is_xmas(x_loc, dir, wordsearch) {
                xmas_count += 1;
            }
        }
    }

    Ok(xmas_count)
}

pub fn part2(wordsearch: &ParsedInput) -> color_eyre::Result<u32> {
    let mut x_mas_count = 0;

    // Just X shapes and not + shapes
    let corner_dirs = [
        DirectionEx::NorthEast,
        DirectionEx::SouthEast,
        DirectionEx::SouthWest,
        DirectionEx::NorthWest,
    ];

    for (a_loc, a) in wordsearch.indexed_iter() {
        if *a != 'A' {
            continue;
        }

        for dir in corner_dirs {
            if is_mas(a_loc, dir, wordsearch) {
                if is_mas(a_loc, dir.rotate_90_cwise(), wordsearch)
                    || is_mas(a_loc, dir.rotate_90_c_cwise(), wordsearch)
                {
                    x_mas_count += 1;
                }

                break;
            }
        }
    }

    Ok(x_mas_count)
}

fn is_xmas(x_loc: (usize, usize), dir: DirectionEx, wordsearch: &ParsedInput) -> bool {
    wordsearch
        .get_in_direction_ex_indexed(x_loc, dir)
        .is_some_and(|(m_loc, m)| {
            *m == 'M'
                && wordsearch
                    .get_in_direction_ex_indexed(m_loc, dir)
                    .is_some_and(|(a_loc, a)| {
                        *a == 'A'
                            && wordsearch
                                .get_in_direction_ex_indexed(a_loc, dir)
                                .is_some_and(|(_s_loc, s)| *s == 'S')
                    })
        })
}

fn is_mas(a_loc: (usize, usize), dir: DirectionEx, wordsearch: &ParsedInput) -> bool {
    wordsearch
        .get_in_direction_ex_indexed(a_loc, dir)
        .is_some_and(|(_m_loc, m)| {
            *m == 'M'
                && wordsearch
                    .get_in_direction_ex_indexed(a_loc, dir.opposite())
                    .is_some_and(|(_s_loc, s)| *s == 'S')
        })
}
