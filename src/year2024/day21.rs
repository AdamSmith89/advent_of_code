use itertools::Itertools;

use crate::{error::AdventError, util::point::Point};

type ParsedInput = Vec<String>;

pub fn parse(input: &str) -> color_eyre::Result<ParsedInput> {
    Ok(input.lines().map(String::from).collect_vec())
}

pub fn part1(codes: &ParsedInput) -> color_eyre::Result<u32> {
    /*

    num_pad <- dir_pad_1 <- dir_pad_2 <- my_pad
    num_pad
        ^-  press dir moves arm (robot 1 presses)
                   ^- press dir moves arm (robot 2 presses)
                             ^- press dir moves arm (my presses)

    num_pad_pos = ? A
    dir_pad_1_pos = ? A
    dir_pad_2_pos = ? A

    for digit in code // don't forget the A
      digit_pos = get_digit_pos(digit)
      num_pad_moves = get_moves(num_pad_pos, digit_pos)

      for arrow in num_pad_moves
        arrow_pos = get_arrow_pos(arrow)
        dir_pad_1_moves = get_moves(dir_pad_1_pos, arrow_pos)

        for arrow in dir_pad_1_moves
            arrow_pos = get_arrow_pos(arrow)
            dir_pad_2_moves = get_moves(dir_pad_2_pos, arrow_pos)

     */

    for code in codes {
        let mut num_pad_pos = Point::from((2, 3));
        let mut dir_pad_1_pos = Point::from((2, 0));
        let mut dir_pad_2_pos = Point::from((2, 0));

        let mut num_pad_res = String::new();
        let mut dir_pad_1_res = String::new();
        let mut dir_pad_2_res = String::new();

        for button in code.chars() {
            let button_pos = get_num_pad_pos(button)?;
            let num_pad_moves = get_robot_moves(num_pad_pos, button_pos);

            //dbg!((button, num_pad_pos, button_pos, &num_pad_moves));
            num_pad_res.extend(num_pad_moves.iter());

            for arrow in num_pad_moves {
                let button_pos = get_dir_pad_pos(arrow)?;
                let dir_pad_1_moves = get_robot_moves(dir_pad_1_pos, button_pos);

                //dbg!((arrow, dir_pad_1_pos, button_pos, &dir_pad_1_moves));
                dir_pad_1_res.extend(dir_pad_1_moves.iter());

                for arrow in dir_pad_1_moves {
                    let button_pos = get_dir_pad_pos(arrow)?;
                    let dir_pad_2_moves = get_robot_moves(dir_pad_2_pos, button_pos);

                    //dbg!((arrow, dir_pad_2_pos, button_pos, &dir_pad_2_moves));
                    dir_pad_2_res.extend(dir_pad_2_moves.iter());

                    dir_pad_2_pos = button_pos;
                }

                dir_pad_1_pos = button_pos;
            }

            num_pad_pos = button_pos;
        }
        //dbg!(dir_pad_2_res);
        // dbg!(dir_pad_1_res);
        // dbg!(num_pad_res);

        println!("{code}: {dir_pad_2_res} : {}", dir_pad_2_res.len());
    }

    Ok(0)
}

pub fn part2(_: &ParsedInput) -> color_eyre::Result<u32> {
    Ok(0)
}

// enum Move {
//     Up,
//     Down,
//     Left,
//     Right,
// }

fn get_num_pad_pos(button: char) -> color_eyre::Result<Point> {
    /*
    +---+---+---+
    | 7 | 8 | 9 |
    +---+---+---+
    | 4 | 5 | 6 |
    +---+---+---+
    | 1 | 2 | 3 |
    +---+---+---+
        | 0 | A |
        +---+---+
    */

    match button {
        '7' => Ok((0, 0).into()),
        '8' => Ok((1, 0).into()),
        '9' => Ok((2, 0).into()),
        '4' => Ok((0, 1).into()),
        '5' => Ok((1, 1).into()),
        '6' => Ok((2, 1).into()),
        '1' => Ok((0, 2).into()),
        '2' => Ok((1, 2).into()),
        '3' => Ok((2, 2).into()),
        '0' => Ok((1, 3).into()),
        'A' => Ok((2, 3).into()),
        _ => Err(AdventError::UnknownPattern(button.to_string()).into()),
    }
}

fn get_robot_moves(from: Point, to: Point) -> Vec<char> {
    let x_diff = from.x.abs_diff(to.x);
    // let x_moves = match from.x.cmp(&to.x) {
    //     std::cmp::Ordering::Less => vec!['>'; x_diff],
    //     std::cmp::Ordering::Equal => vec![],
    //     std::cmp::Ordering::Greater => vec!['<'; x_diff],
    // };

    // NEED TO NOT MOVE OVER THE EMPTY SPACE

    let mut moves = Vec::new();

    if from.x > to.x {
        moves.extend(vec!['<'; x_diff].iter());
    }

    let y_diff = from.y.abs_diff(to.y);
    match from.y.cmp(&to.y) {
        std::cmp::Ordering::Less => moves.extend(vec!['v'; y_diff].iter()),
        std::cmp::Ordering::Greater => moves.extend(vec!['^'; y_diff].iter()),
        std::cmp::Ordering::Equal => (),
    }

    if from.x < to.x {
        moves.extend(vec!['>'; x_diff].iter());
    }

    moves.push('A');
    moves

    //[y_moves, x_moves, vec!['A']].concat()
}

fn get_dir_pad_pos(button: char) -> color_eyre::Result<Point> {
    /*
        +---+---+
        | ^ | A |
    +---+---+---+
    | < | v | > |
    +---+---+---+
    */

    match button {
        '^' => Ok((1, 0).into()),
        'A' => Ok((2, 0).into()),
        '<' => Ok((0, 1).into()),
        'v' => Ok((1, 1).into()),
        '>' => Ok((2, 1).into()),
        _ => Err(AdventError::UnknownPattern(button.to_string()).into()),
    }
}
