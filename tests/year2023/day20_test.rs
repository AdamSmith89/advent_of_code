use advent_of_code::year2023::day20::*;

const PUZZLE_INPUT: &str = include_str!("../../input/year2023/day20.txt");
const EXAMPLE_INPUT_1: &str = "\
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
const EXAMPLE_INPUT_2: &str = "\
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

fn assert_module(
    parsed: &ParsedInput,
    name: &str,
    matches_exp_type: fn(&Module) -> bool,
    exp_targets: Option<&Vec<String>>,
) {
    let module = parsed.get(name);
    assert!(module.is_some());

    let module = module.unwrap();
    assert!(matches_exp_type(module));
    assert_eq!(module.targets(), exp_targets);
}

#[test]
fn parse_example_1() {
    let parsed = parse(EXAMPLE_INPUT_1).expect("Error parsing input");

    assert_module(
        &parsed,
        "broadcaster",
        |module| matches!(module, Module::Broadcaster { .. }),
        Some(&vec!["a".to_string(), "b".to_string(), "c".to_string()]),
    );

    assert_module(
        &parsed,
        "a",
        |module| matches!(module, Module::FlipFlop { .. }),
        Some(&vec!["b".to_string()]),
    );

    assert_module(
        &parsed,
        "b",
        |module| matches!(module, Module::FlipFlop { .. }),
        Some(&vec!["c".to_string()]),
    );

    assert_module(
        &parsed,
        "c",
        |module| matches!(module, Module::FlipFlop { .. }),
        Some(&vec!["inv".to_string()]),
    );

    assert_module(
        &parsed,
        "inv",
        |module| matches!(module, Module::Conjuction { .. }),
        Some(&vec!["a".to_string()]),
    );
}

#[test]
fn parse_example_2() {
    let parsed = parse(EXAMPLE_INPUT_2).expect("Error parsing input");

    assert_module(
        &parsed,
        "broadcaster",
        |module| matches!(module, Module::Broadcaster { .. }),
        Some(&vec!["a".to_string()]),
    );

    assert_module(
        &parsed,
        "a",
        |module| matches!(module, Module::FlipFlop { .. }),
        Some(&vec!["inv".to_string(), "con".to_string()]),
    );

    assert_module(
        &parsed,
        "inv",
        |module| matches!(module, Module::Conjuction { .. }),
        Some(&vec!["b".to_string()]),
    );

    assert_module(
        &parsed,
        "b",
        |module| matches!(module, Module::FlipFlop { .. }),
        Some(&vec!["con".to_string()]),
    );

    assert_module(
        &parsed,
        "con",
        |module| matches!(module, Module::Conjuction { .. }),
        Some(&vec!["output".to_string()]),
    );
}

#[test]
fn part1_example_1() {
    let input = parse(EXAMPLE_INPUT_1).expect("Error parsing input");
    let answer = part1(&input).expect("Error solving part 1");

    assert_eq!(answer, 32000000);
}

#[test]
fn part1_example_2() {
    let input = parse(EXAMPLE_INPUT_2).expect("Error parsing input");
    let answer = part1(&input).expect("Error solving part 1");

    assert_eq!(answer, 11687500);
}

#[test]
fn part1_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let answer = part1(&parsed).expect("Error solving part 1");

    assert_eq!(answer, 886701120);
}

#[test]
fn part2_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let answer = part2(&parsed).expect("Error solving part 2");

    assert_eq!(answer, 228134431501037);
}
