use std::collections::HashSet;

use grid::*;

type ParsedInput = Grid<char>;

pub fn parse(input: &str) -> color_eyre::Result<ParsedInput> {
    let mut grid: ParsedInput = Grid::new(0, 0);

    for line in input.lines() {
        grid.push_row(line.chars().collect());
    }

    Ok(grid)
}

pub fn part1(grid: &ParsedInput) -> color_eyre::Result<u32> {
    // any number adjacent to a symbol, even diagonally, is a "part number"
    // add up all the part numbers in the engine schematic

    let mut h = HashSet::new();
    let mut first_visit = |(num, points): (u32, Vec<(usize, usize)>)| {
        if points.iter().all(|&point| h.insert(point)) {
            return Some(num);
        }
        None
    };

    let sum: u32 = grid
        .indexed_iter()
        .filter(|((_, _), &value)| !value.is_ascii_digit() && value != '.')
        .map(|((row, col), _)| adjacent_digit_points(grid, row, col))
        .flat_map(reduce_adjacent_digit_points)
        .map(|(row, col)| build_number_at_point(grid, row, col))
        .filter_map(|(num, points)| first_visit((num, points)))
        .sum();

    Ok(sum)
}

pub fn part2(grid: &ParsedInput) -> color_eyre::Result<u32> {
    let sum = grid
        .indexed_iter()
        // Filter to '*' locations - gears
        .filter(|((_, _), &value)| value == '*')
        // for each gear, map to the adjacent points which are digits
        .map(|((row, col), _)| adjacent_digit_points(grid, row, col))
        // for each set of adjacent digit points, remove the ones which are in the same number
        .map(reduce_adjacent_digit_points)
        // for each set of adjacent points, map to the full number
        .map(|points| {
            points
                .iter()
                .map(|(row, col)| build_number_at_point(grid, *row, *col).0)
                .collect::<Vec<_>>()
        })
        // for each set of adjacent numbers, filter to the sets with just 2 (gears only have 2 parts)
        .filter(|gear_parts| gear_parts.len() == 2)
        // for each set of adjacent numbers, multiply them
        .map(|gear_parts| gear_parts.iter().product::<u32>())
        // Sum all the products
        .sum();

    Ok(sum)
}

fn adjacent_digit_points(grid: &Grid<char>, row: usize, col: usize) -> Vec<(usize, usize)> {
    let offsets: Vec<(i32, i32)> = vec![
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
    ];

    offsets
        .iter()
        .filter_map(|(row_offset, col_offset)| {
            let row_n = usize::try_from(row as i32 + row_offset);
            let col_n = usize::try_from(col as i32 + col_offset);

            if row_n.is_ok() && col_n.is_ok() {
                let row_n = row_n.unwrap();
                let col_n = col_n.unwrap();
                if let Some(v) = grid.get(row_n, col_n) {
                    if v.is_ascii_digit() {
                        return Some((row_n, col_n));
                    }
                }
            }

            None
        })
        .collect::<Vec<_>>()
}

fn reduce_adjacent_digit_points(points: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut reduced_pts = Vec::new();
    let mut thrown_away = Vec::new();
    for (row, col) in points {
        if !reduced_pts
            .iter()
            .any(|(row_inner, col_inner)| row == *row_inner && col.abs_diff(*col_inner) == 1)
            && !thrown_away
                .iter()
                .any(|(row_inner, col_inner)| row == *row_inner && col.abs_diff(*col_inner) == 1)
        {
            reduced_pts.push((row, col));
        } else {
            thrown_away.push((row, col));
        }
    }
    reduced_pts
}

fn build_number_at_point(grid: &Grid<char>, row: usize, col: usize) -> (u32, Vec<(usize, usize)>) {
    let mut number = String::from(unsafe { *grid.get_unchecked(row, col) });
    let mut points = vec![(row, col)];

    grid.iter_row(row)
        .enumerate()
        .skip(col + 1)
        .take_while(|(_, ch)| ch.is_ascii_digit())
        .for_each(|(col, &ch)| {
            number.push(ch);
            points.push((row, col));
        });

    grid.iter_row(row)
        .enumerate()
        .rev()
        .skip(grid.cols() - col)
        .take_while(|(_, ch)| ch.is_ascii_digit())
        .for_each(|(col, &ch)| {
            number.insert(0, ch);
            points.push((row, col));
        });

    let number = number
        .parse()
        .unwrap_or_else(|_| panic!("Failed to parse {number} into u32"));

    (number, points)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        let input = "\
012345
6789ab
cdefgh
";
        let grid = parse(input).expect("Error parsing input");
        assert_eq!(grid[(0, 0)], '0');
        assert_eq!(grid[(1, 5)], 'b');
        assert_eq!(grid[(2, 1)], 'd');
        assert_eq!(grid[(2, 4)], 'g');
    }

    const EXAMPLE: &str = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn part1_test() {
        let input = parse(EXAMPLE).expect("Error parsing input");
        let answer = part1(&input).expect("Error solving part 1");

        assert_eq!(answer, 4361);
    }

    #[test]
    fn part2_test() {
        let input = parse(EXAMPLE).expect("Error parsing input");
        let answer = part2(&input).expect("Error solving part 2");

        assert_eq!(answer, 467835);
    }

    #[test]
    fn part1_edge1() {
        let input = parse(
            "\
........
.24..4..
......*.",
        )
        .expect("Error parsing input");

        let answer = part1(&input).expect("Error solving part 1");
        assert_eq!(answer, 4);
    }

    #[test]
    fn part1_edge2() {
        let input = parse(
            "\
........
.24$-4..
......*.",
        )
        .expect("Error parsing input");

        let answer = part1(&input).expect("Error solving part 1");
        assert_eq!(answer, 28);
    }

    #[test]
    fn part1_edge3() {
        let input = parse(
            "\
11....11
..$..$..
11....11",
        )
        .expect("Error parsing input");

        let answer = part1(&input).expect("Error solving part 1");
        assert_eq!(answer, 44);
    }

    #[test]
    fn part1_edge4() {
        let input = parse(
            "\
$......$
.1....1.
.1....1.
$......$",
        )
        .expect("Error parsing input");

        let answer = part1(&input).expect("Error solving part 1");
        assert_eq!(answer, 4);
    }

    #[test]
    fn part1_edge5() {
        let input = parse(
            "\
$......$
.11..11.
.11..11.
$......$",
        )
        .expect("Error parsing input");

        let answer = part1(&input).expect("Error solving part 1");
        assert_eq!(answer, 44);
    }

    #[test]
    fn part1_edge6() {
        let input = parse(
            "\
$11
...
11$
...",
        )
        .expect("Error parsing input");

        let answer = part1(&input).expect("Error solving part 1");
        assert_eq!(answer, 22);
    }

    #[test]
    fn part1_edge7() {
        let input = parse(
            "\
$..
.11
.11
$..
..$
11.
11.
..$",
        )
        .expect("Error parsing input");

        let answer = part1(&input).expect("Error solving part 1");
        assert_eq!(answer, 44);
    }

    #[test]
    fn part1_edge8() {
        let input = parse(
            "\
11.$.",
        )
        .expect("Error parsing input");

        let answer = part1(&input).expect("Error solving part 1");
        assert_eq!(answer, 0);
    }

    #[test]
    fn part2_edge1() {
        let input = parse(
            "\
70*...
...898",
        )
        .expect("Error parsing input");

        let answer = part2(&input).expect("Error solving part 2");
        assert_eq!(answer, 62860);
    }

    #[test]
    fn part2_edge2() {
        let input = parse(
            "\
*11..
11...
...11
..11*",
        )
        .expect("Error parsing input");

        let answer = part2(&input).expect("Error solving part 2");
        assert_eq!(answer, 242);
    }

    #[test]
    fn part2_edge3() {
        let input = parse(
            "\
11...
*....
11...
",
        )
        .expect("Error parsing input");

        let answer = part2(&input).expect("Error solving part 2");
        assert_eq!(answer, 121);
    }

    #[test]
    fn part2_edge4() {
        let input = parse(
            "\
..*..
11.11
..*..
11.11
..*..",
        )
        .expect("Error parsing input");

        let answer = part2(&input).expect("Error solving part 2");
        assert_eq!(answer, 242);
    }

    #[test]
    fn part2_edge5() {
        let input = parse(
            "\
11**11
11..11",
        )
        .expect("Error parsing input");

        let answer = part2(&input).expect("Error solving part 2");
        assert_eq!(answer, 242);
    }

    #[test]
    fn part2_edge6() {
        let input = parse(
            "\
11..11
..**..
..11..",
        )
        .expect("Error parsing input");

        let answer = part2(&input).expect("Error solving part 2");
        assert_eq!(answer, 242);
    }

    #[test]
    fn part2_edge7() {
        let input = parse(
            "\
..11..
..**..
..11..",
        )
        .expect("Error parsing input");

        let answer = part2(&input).expect("Error solving part 2");
        assert_eq!(answer, 242);
    }

    #[test]
    fn part2_edge8() {
        let input = parse(
            "\
11*11",
        )
        .expect("Error parsing input");

        let answer = part2(&input).expect("Error solving part 2");
        assert_eq!(answer, 121);
    }

    #[test]
    fn part2_edge9() {
        let input = parse(
            "\
.481
.*..
228.",
        )
        .expect("Error parsing input");

        let answer = part2(&input).expect("Error solving part 2");
        assert_eq!(answer, 109668);
    }
}
