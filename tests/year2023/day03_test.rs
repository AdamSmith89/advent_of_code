use advent_of_code::year2023::day03::*;

const PUZZLE_INPUT: &str = include_str!("../../input/year2023/day03.txt");
const EXAMPLE_INPUT: &str = "\
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

#[test]
fn part1_example() {
    let input = parse(EXAMPLE_INPUT).expect("Error parsing input");
    let answer = part1(&input).expect("Error solving part 1");

    assert_eq!(answer, 4361);
}

#[test]
fn part1_real() {
    let input = parse(PUZZLE_INPUT).expect("Error parsing input");
    let answer = part1(&input).expect("Error solving part 1");

    assert_eq!(answer, 557705);
}

#[test]
fn part2_example() {
    let input = parse(EXAMPLE_INPUT).expect("Error parsing input");
    let answer = part2(&input).expect("Error solving part 2");

    assert_eq!(answer, 467835);
}

#[test]
fn part2_real() {
    let input = parse(PUZZLE_INPUT).expect("Error parsing input");
    let answer = part2(&input).expect("Error solving part 2");

    assert_eq!(answer, 84266818);
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
