use crate::error::AdventError;

type ParsedInput = Vec<Game>;

pub fn parse(input: &str) -> color_eyre::Result<ParsedInput> {
    Ok(input
        .split("\n\n")
        .map(|game| {
            let mut game_lines = game.lines();
            let button_a = game_lines
                .next()
                .ok_or(AdventError::LogicError("Lost Button A".to_string()))?;
            let button_b = game_lines
                .next()
                .ok_or(AdventError::LogicError("Lost Button B".to_string()))?;
            let prize = game_lines
                .next()
                .ok_or(AdventError::LogicError("Lost prize".to_string()))?;

            let a_move = parse_line(button_a, '+')?;
            let b_move = parse_line(button_b, '+')?;
            let prize_loc = parse_line(prize, '=')?;

            Ok(Game::new(a_move, b_move, prize_loc))
        })
        .collect::<Result<Vec<Game>, AdventError>>()?)
}

fn parse_line(line: &str, delimeter: char) -> Result<(u128, u128), AdventError> {
    let (_, offsets) = line
        .split_once(':')
        .ok_or(AdventError::SplitOnce(line.to_string(), ':'.to_string()))?;

    let (x_offset, y_offset) = offsets
        .split_once(',')
        .ok_or(AdventError::SplitOnce(offsets.to_string(), ','.to_string()))?;

    let (_, x_offset) = x_offset
        .split_once(delimeter)
        .ok_or(AdventError::SplitOnce(
            x_offset.to_string(),
            delimeter.to_string(),
        ))?;

    let (_, y_offset) = y_offset
        .split_once(delimeter)
        .ok_or(AdventError::SplitOnce(
            y_offset.to_string(),
            delimeter.to_string(),
        ))?;

    let x_offset = x_offset.parse::<u128>().map_err(AdventError::ParseInt)?;
    let y_offset = y_offset.parse::<u128>().map_err(AdventError::ParseInt)?;

    Ok((x_offset, y_offset))
}

pub fn part1(games: &ParsedInput) -> color_eyre::Result<i64> {
    Ok(games.iter().filter_map(|game| solve_game(game, 0)).sum())
}

pub fn part2(games: &ParsedInput) -> color_eyre::Result<i64> {
    Ok(games
        .iter()
        .filter_map(|game| solve_game(game, 10000000000000))
        .sum())
}

fn solve_game(game: &Game, prize_offset: i64) -> Option<i64> {
    // Found on https://www.reddit.com/r/adventofcode/comments/1hd7irq/2024_day_13_an_explanation_of_the_mathematics/
    // Cramer's rule
    // A = (p_x*b_y - prize_y*b_x) / (a_x*b_y - a_y*b_x)
    // B = (a_x*p_y - a_y*p_x) / (a_x*b_y - a_y*b_x)

    // Solution taken from https://github.com/voberle/adventofcode/blob/main/2024/day13/src/main.rs
    // as it had a better explanation of the steps

    let ax = game.a_move.0 as i64;
    let ay = game.a_move.1 as i64;
    let bx = game.b_move.0 as i64;
    let by = game.b_move.1 as i64;
    let px = game.prize_loc.0 as i64 + prize_offset;
    let py = game.prize_loc.1 as i64 + prize_offset;

    // We have following equations (a and b are unknown):
    //  a * ax + b * bx = px
    //  a * ay + b * by = py
    // We use the elimination method to get rid of a,
    // multiplying the first by ay and the second by ax:
    //  (a * ax + b * bx) * ay = px * ay
    //  (a * ay + b * by) * ax = py * ax
    // then we substract the second equation from the first:
    //  b * bx * ay - b * by * ax = px * ay - py * ax
    //  b * (bx * ay - by * ax) = px * ay - py * ax
    // giving us b:
    //  b = (px * ay - py * ax) / (bx * ay - by * ax)
    // If this division works without a modulo, we have a b.
    // Then we can get a with:
    //  a = (px - b * bx) / ax
    // and to the same check.

    let num_b = px * ay - py * ax;
    let den_b = bx * ay - by * ax;

    if den_b != 0 && num_b % den_b == 0 {
        let b = num_b / den_b;

        let num_a = px - b * bx;
        let den_a = ax;

        if den_a != 0 && num_a % den_a == 0 {
            let a = num_a / den_a;

            // A presses cost 3 and B presses cost 1
            Some((a * 3) + b)
        } else {
            None
        }
    } else {
        None
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Game {
    a_move: (u128, u128),
    b_move: (u128, u128),
    prize_loc: (u128, u128),
}

impl Game {
    pub fn new(a_move: (u128, u128), b_move: (u128, u128), prize_loc: (u128, u128)) -> Self {
        Self {
            a_move,
            b_move,
            prize_loc,
        }
    }
}
