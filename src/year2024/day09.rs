use std::fmt::Debug;

use crate::error::AdventError;

type ParsedInput = Vec<File>;

pub fn parse(input: &str) -> color_eyre::Result<ParsedInput> {
    let mut files = Vec::new();
    let mut file_id = 0;
    let mut is_file = true;

    for ch in input.chars() {
        // Ignores the new-line
        if let Some(num) = ch.to_digit(10) {
            if is_file {
                files.push(File {
                    id: Some(file_id),
                    size: num,
                });

                file_id += 1;
            } else {
                files.push(File {
                    id: None,
                    size: num,
                });
            }

            is_file = !is_file;
        }
    }

    Ok(files)
}

pub fn part1(disk_map: &ParsedInput) -> color_eyre::Result<u128> {
    let mut new_disk_map = Vec::new();

    let mut rev_iter = disk_map.iter().rev().enumerate();
    let mut end_opt = rev_iter.next();
    let mut end_file_rem = None;

    let mut space_to_fill: u32 = disk_map
        .iter()
        .filter_map(|file| {
            if file.id.is_none() {
                Some(file.size)
            } else {
                None
            }
        })
        .sum();

    let mut file_sizes = 0;
    for file in disk_map.iter().rev() {
        if file.id.is_some() {
            file_sizes += file.size;
        } else {
            space_to_fill -= file.size;
        }

        if file_sizes >= space_to_fill {
            break;
        }
    }

    let mut space_filled = 0;

    for file in disk_map.iter() {
        if file.id.is_some() {
            new_disk_map.push(file.clone());
            continue;
        }

        let mut space_size = file.size;

        loop {
            let (_, end_file) =
                end_opt.ok_or(AdventError::LogicError("Hit end of rev iter".to_string()))?;

            if end_file.id.is_none() {
                end_opt = rev_iter.next();
                continue;
            }

            let end_file_size = end_file_rem.unwrap_or(end_file.size);

            if end_file_size == space_size {
                let new_file = File {
                    id: end_file.id,
                    size: end_file_size,
                };
                new_disk_map.push(new_file);
                end_file_rem = None;
                end_opt = rev_iter.next();
                space_filled += end_file_size;

                break;
            } else if end_file_size < space_size {
                let new_file = File {
                    id: end_file.id,
                    size: end_file_size,
                };
                new_disk_map.push(new_file);
                end_file_rem = None;
                end_opt = rev_iter.next();
                space_filled += end_file_size;

                space_size -= end_file_size;
            } else if end_file_size > space_size {
                let new_file = File {
                    id: end_file.id,
                    size: space_size,
                };
                new_disk_map.push(new_file);
                end_file_rem = Some(end_file_size - space_size);

                space_filled += space_size;

                break;
            }
        }

        if space_filled >= space_to_fill {
            if end_file_rem.is_some_and(|n| n > 0) {
                let (_, end_file) =
                    end_opt.ok_or(AdventError::LogicError("Hit end of rev iter".to_string()))?;

                let new_file = File {
                    id: end_file.id,
                    size: end_file_rem.unwrap(),
                };
                new_disk_map.push(new_file);
            }

            break;
        }
    }

    Ok(calc_checksum(new_disk_map))
}

pub fn part2(disk_map: &ParsedInput) -> color_eyre::Result<u128> {
    let mut new_disk_map = disk_map.clone();

    for (rev_idx, file_to_move) in disk_map.iter().rev().enumerate() {
        if file_to_move.id.is_none() {
            continue;
        }

        // Used to make sure the space we've found is before the item we are processing
        let fwd_idx = (new_disk_map.len() - 1) - rev_idx;

        if let Some((space_idx, space)) =
            new_disk_map
                .iter_mut()
                .enumerate()
                .find(|(space_idx, file)| {
                    *space_idx <= fwd_idx && file.id.is_none() && file.size >= file_to_move.size
                })
        {
            let rem_space = space.size - file_to_move.size;
            *space = file_to_move.clone();

            if rem_space > 0 {
                new_disk_map.insert(
                    space_idx + 1,
                    File {
                        id: None,
                        size: rem_space,
                    },
                );
            }

            if let Some(moved) = new_disk_map
                .iter_mut()
                .rev()
                .find(|file| file.id == file_to_move.id)
            {
                moved.id = None;
            }
        }
    }

    Ok(calc_checksum(new_disk_map))
}

fn calc_checksum(new_disk_map: Vec<File>) -> u128 {
    let mut pos = 0;
    let mut checksum: u128 = 0;
    for file in new_disk_map {
        if let Some(id) = file.id {
            for _ in 0..file.size {
                checksum += pos * id as u128;
                pos += 1;
            }
        } else {
            pos += file.size as u128;
        }
    }
    checksum
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct File {
    id: Option<u32>, // A "None" file represents empty space
    size: u32,
}

impl File {
    pub fn new(id: Option<u32>, size: u32) -> Self {
        File { id, size }
    }
}

impl Debug for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let id = if let Some(id) = self.id {
            id.to_string()
        } else {
            ".".to_string()
        };

        for _ in 0..self.size {
            f.write_str(id.as_str())?;
        }
        Ok(())

        //f.write_fmt(format_args!("('{}', {})", id, self.size))
    }
}
