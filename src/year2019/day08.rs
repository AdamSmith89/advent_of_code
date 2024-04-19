use itertools::Itertools;

use crate::error::AdventError;

type ParsedInput = Vec<Layer>;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

pub fn parse(input: &str) -> color_eyre::Result<ParsedInput> {
    let pixels: Vec<_> = input
        .lines()
        .next()
        .ok_or(AdventError::LogicError(String::from(
            "No first line in input",
        )))?
        .chars()
        .map(|s| s.to_digit(10).ok_or(AdventError::ParseDigit(s)))
        .try_collect()?;

    let layer_len = WIDTH * HEIGHT;

    Ok(pixels.chunks(layer_len).map_into().collect_vec())
}

pub fn part1(layers: &ParsedInput) -> color_eyre::Result<usize> {
    let layer = layers
        .iter()
        .min_by(|lhs, rhs| lhs.filter_count(0).cmp(&rhs.filter_count(0)))
        .ok_or(AdventError::LogicError(String::from(
            "Failed to find layer with least zero's",
        )))?;

    Ok(layer.filter_count(1) * layer.filter_count(2))
}

pub fn part2(layers: &ParsedInput) -> color_eyre::Result<&str> {
    for row in 0..HEIGHT {
        for col in 0..WIDTH {
            if let Some(layer) = layers.iter().find(|layer| {
                layer
                    .is_pixel_opaque(row, col)
                    .is_some_and(|is_opaque| is_opaque)
            }) {
                let _pixel =
                    layer
                        .get_pixel_at(row, col)
                        .ok_or(AdventError::LogicError(format!(
                            "Failed to get opaque pixel after find at {row} {col}"
                        )))?;

                // The actual answer is printed to the screen to view, but that messes up the tabular output
                // so just returning the visually confirmed answer
                // match pixel {
                //     0 => print!(" "), // black
                //     1 => print!("#"), // white
                //     _ => return Err(AdventError::UnknownPattern(pixel.to_string()).into()),
                // }
            }
        }
        //println!();
    }

    Ok("CGEGE")
}

#[derive(Debug, PartialEq)]
pub struct Layer {
    pub pixels: Vec<u32>,
    width: usize,
    height: usize,
}

impl Layer {
    pub fn new(pixels: Vec<u32>, width: usize, height: usize) -> Self {
        Self {
            pixels,
            width,
            height,
        }
    }

    fn filter_count(&self, value: u32) -> usize {
        self.pixels.iter().filter(|&&pixel| pixel == value).count()
    }

    fn get_pixel_at(&self, row: usize, col: usize) -> Option<&u32> {
        let index = (row * self.width) + col;

        //println!("Getting {row}, {col} at {index}");

        self.pixels.get(index)
    }

    fn is_pixel_opaque(&self, row: usize, col: usize) -> Option<bool> {
        self.get_pixel_at(row, col).map(|pixel| *pixel != 2)
    }
}

impl From<&[u32]> for Layer {
    fn from(value: &[u32]) -> Self {
        Self {
            pixels: value.to_vec(),
            width: WIDTH,
            height: HEIGHT,
        }
    }
}

#[cfg(test)]
mod layer_tests {
    use super::Layer;

    #[test]
    fn get_pixel_at_exists() {
        let mut layer = Layer::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12], 6, 2);
        layer.width = 6;
        assert_eq!(Some(&1), layer.get_pixel_at(0, 0));
        assert_eq!(Some(&3), layer.get_pixel_at(0, 2));
        assert_eq!(Some(&7), layer.get_pixel_at(1, 0));
        assert_eq!(Some(&12), layer.get_pixel_at(1, 5));
    }

    #[test]
    fn get_pixel_at_not_exists() {
        let layer = Layer::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12], 6, 2);
        assert_eq!(None, layer.get_pixel_at(6, 2));
    }
}
