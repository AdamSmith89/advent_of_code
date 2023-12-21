use advent_of_code::year2023::day19::*;

const PUZZLE_INPUT: &str = include_str!("../../input/year2023/day19.txt");
const EXAMPLE_INPUT: &str = "\
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

#[test]
fn parse_example() {
    let (workflows, parts) = parse(EXAMPLE_INPUT).expect("Error parsing input");

    assert_eq!(workflows.len(), 11);

    // px{a<2006:qkq,m>2090:A,rfg}
    assert_eq!(
        workflows.get("px"),
        Some(&Workflow {
            rules: vec![
                Rule {
                    kind: RuleKind::LessThan,
                    field: Field::A,
                    value: 2006,
                    target: "qkq".to_string()
                },
                Rule {
                    kind: RuleKind::GreaterThan,
                    field: Field::M,
                    value: 2090,
                    target: "A".to_string()
                },
            ],
            fallthrough: "rfg".to_string(),
        })
    );

    // crn{x>2662:A,R}
    assert_eq!(
        workflows.get("crn"),
        Some(&Workflow {
            rules: vec![Rule {
                kind: RuleKind::GreaterThan,
                field: Field::X,
                value: 2662,
                target: "A".to_string()
            },],
            fallthrough: "R".to_string(),
        })
    );

    assert_eq!(
        parts,
        vec![
            Part {
                x: 787,
                m: 2655,
                a: 1222,
                s: 2876
            },
            Part {
                x: 1679,
                m: 44,
                a: 2067,
                s: 496
            },
            Part {
                x: 2036,
                m: 264,
                a: 79,
                s: 2244
            },
            Part {
                x: 2461,
                m: 1339,
                a: 466,
                s: 291
            },
            Part {
                x: 2127,
                m: 1623,
                a: 2188,
                s: 1013
            },
        ]
    )
}

#[test]
fn part1_example() {
    let input = parse(EXAMPLE_INPUT).expect("Error parsing input");
    let answer = part1(&input).expect("Error solving part 1");

    assert_eq!(answer, 19114);
}

#[test]
fn part1_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let answer = part1(&parsed).expect("Error solving part 1");

    assert_eq!(answer, 362930);
}

#[test]
fn part2_example() {
    let input = parse(EXAMPLE_INPUT).expect("Error parsing example input");
    let answer = part2(&input).expect("Error solving part 2");

    assert_eq!(answer, 167409079868000);
}

#[test]
fn part2_real() {
    let parsed = parse(PUZZLE_INPUT).expect("Error parsing input");
    let answer = part2(&parsed).expect("Error solving part 2");

    assert_eq!(answer, 116365820987729);
}

#[test]
fn rule_tryfrom_str() {
    let rule = Rule::try_from("a<2006:qkq");
    assert!(rule.is_ok());
    let rule = rule.unwrap();

    assert_eq!(
        rule,
        Rule {
            kind: RuleKind::LessThan,
            field: Field::A,
            value: 2006,
            target: "qkq".to_string()
        }
    )

    // assert!(condition.is_ok());
    // let condition = condition.unwrap();
    // assert_eq!(condition.field, Field::A);
    // assert_eq!(condition.value, 2006);
    // assert_eq!(condition.target, "qkq".to_string());
    // assert!((condition.op)(10, 2006));
    // assert!(!(condition.op)(3000, 2006));
}
