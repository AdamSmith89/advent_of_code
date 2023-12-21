use std::{collections::HashMap, ops::Range};

use itertools::Itertools;

use crate::error::AdventError;

type Workflows = HashMap<String, Workflow>;
type Parts = Vec<Part>;

type ParsedInput = (Workflows, Parts);

pub fn parse(input: &str) -> color_eyre::Result<ParsedInput> {
    let input = input.replace("\r\n", "\n");

    let (workflows, parts) = input.split_once("\n\n").ok_or(AdventError::SplitOnce(
        input.to_string(),
        "double-newline".into(),
    ))?;

    // px{a<2006:qkq,m>2090:A,rfg}
    let workflows: Workflows = workflows
        .lines()
        .map(|line| -> Result<_, AdventError> {
            let (name, rules) = line
                .split_once('{')
                .ok_or(AdventError::SplitOnce(line.into(), '{'.into()))?;

            let rules = rules.trim_end_matches('}');
            let rules = rules.split(',').collect_vec();

            let fallthrough = rules.last().ok_or(AdventError::EmptySlice)?.to_string();
            let rules = rules
                .iter()
                .take(rules.len() - 1)
                .copied()
                .map(Rule::try_from)
                .try_collect()?;

            Ok((name.to_string(), Workflow { rules, fallthrough }))
        })
        .try_collect()?;

    // {x=787,m=2655,a=1222,s=2876}
    let parts = parts.lines().map(Part::try_from).try_collect()?;

    Ok((workflows, parts))
}

pub fn part1(input: &ParsedInput) -> color_eyre::Result<u64> {
    let workflows = &input.0;
    let parts = &input.1;

    let mut accepted_parts = Vec::new();

    for part in parts {
        let mut cur_workflow = "in";

        loop {
            let workflow = workflows
                .get(cur_workflow)
                .ok_or(AdventError::NotFound("in".into()))?;
            let next = workflow.eval_part(part);

            if next == "A" {
                accepted_parts.push(part);
                break;
            } else if next == "R" {
                // rejected
                break;
            } else {
                cur_workflow = next;
            }
        }
    }

    Ok(accepted_parts.iter().map(|&part| part.total_rating()).sum())
}

pub fn part2(input: &ParsedInput) -> color_eyre::Result<u64> {
    let workflows = &input.0;

    let initial_parts = PartRanged {
        x: 1..4001,
        m: 1..4001,
        a: 1..4001,
        s: 1..4001,
    };

    let mut combinations = vec![Combination {
        workflow_id: "in".to_string(),
        rule_index: 0,
        parts: initial_parts,
    }];

    let mut total = 0;

    while let Some(combination) = combinations.pop() {
        if combination.is_accepted() {
            total += combination.parts.total();
            continue;
        } else if combination.is_rejected() {
            continue;
        }

        let workflow = workflows.get(&combination.workflow_id).unwrap();

        if workflow.should_fallthrough(combination.rule_index) {
            combinations.push(Combination {
                workflow_id: workflow.fallthrough.clone(),
                rule_index: 0,
                parts: combination.parts.clone(),
            });
            continue;
        }

        let rule = &workflow.rules[combination.rule_index];
        let parts = &combination.parts;

        let field_range = match rule.field {
            Field::X => &parts.x,
            Field::M => &parts.m,
            Field::A => &parts.a,
            Field::S => &parts.s,
        };

        match rule.kind {
            RuleKind::LessThan => {
                let r = field_range.start..rule.value;
                let mut lt_combination = combination.clone();
                lt_combination.workflow_id = rule.target.clone();
                match rule.field {
                    Field::X => lt_combination.parts.x = r,
                    Field::M => lt_combination.parts.m = r,
                    Field::A => lt_combination.parts.a = r,
                    Field::S => lt_combination.parts.s = r,
                }
                lt_combination.rule_index = 0;

                combinations.push(lt_combination);

                let r = rule.value..field_range.end;
                let mut gt_combination = combination.clone();
                match rule.field {
                    Field::X => gt_combination.parts.x = r,
                    Field::M => gt_combination.parts.m = r,
                    Field::A => gt_combination.parts.a = r,
                    Field::S => gt_combination.parts.s = r,
                }
                gt_combination.rule_index += 1; // Staying in this workflow, but the next rule

                combinations.push(gt_combination);
            }
            RuleKind::GreaterThan => {
                let r = field_range.start..rule.value + 1;
                let mut lt_combination = combination.clone();
                match rule.field {
                    Field::X => lt_combination.parts.x = r,
                    Field::M => lt_combination.parts.m = r,
                    Field::A => lt_combination.parts.a = r,
                    Field::S => lt_combination.parts.s = r,
                }
                lt_combination.rule_index += 1; // Staying in this workflow, but the next rule

                combinations.push(lt_combination);

                let r = rule.value + 1..field_range.end;
                let mut gt_combination = combination.clone();
                gt_combination.workflow_id = rule.target.clone();
                match rule.field {
                    Field::X => gt_combination.parts.x = r,
                    Field::M => gt_combination.parts.m = r,
                    Field::A => gt_combination.parts.a = r,
                    Field::S => gt_combination.parts.s = r,
                }
                gt_combination.rule_index = 0;

                combinations.push(gt_combination);
            }
        }
    }

    Ok(total)
}

#[derive(Clone, Debug)]
struct Combination {
    workflow_id: String,
    rule_index: usize,
    parts: PartRanged,
}

impl Combination {
    fn is_accepted(&self) -> bool {
        self.workflow_id == "A"
    }

    fn is_rejected(&self) -> bool {
        self.workflow_id == "R"
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Workflow {
    pub rules: Vec<Rule>,
    pub fallthrough: String,
}

impl Workflow {
    pub fn eval_part(&self, part: &Part) -> &String {
        for rule in &self.rules {
            if rule.matches(part) {
                return &rule.target;
            }
        }

        &self.fallthrough
    }

    fn should_fallthrough(&self, rule_index: usize) -> bool {
        rule_index >= self.rules.len()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Rule {
    pub field: Field,
    pub kind: RuleKind,
    pub value: u64,
    pub target: String,
}

impl Rule {
    pub fn matches(&self, part: &Part) -> bool {
        match self.kind {
            RuleKind::LessThan => part.get_field(&self.field) < self.value,
            RuleKind::GreaterThan => part.get_field(&self.field) > self.value,
        }
    }
}

impl TryFrom<&str> for Rule {
    type Error = AdventError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // a<2006:qkq

        let (rule, target) = value
            .split_once(':')
            .ok_or(AdventError::SplitOnce(value.into(), ':'.into()))?;

        let mut rule_iter = rule.chars();

        let field = Field::try_from(rule_iter.next().ok_or(AdventError::EndOfIterator)?)?;

        let kind = rule_iter.next().ok_or(AdventError::EndOfIterator)?;
        let kind = match kind {
            '<' => Ok(RuleKind::LessThan),
            '>' => Ok(RuleKind::GreaterThan),
            _ => Err(AdventError::UnknownPattern(kind.into())),
        }?;

        let value = rule_iter.collect::<String>().parse::<u64>()?;

        Ok(Rule {
            //op: Box::new(op),
            kind,
            field,
            value,
            target: target.into(),
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum RuleKind {
    LessThan,
    GreaterThan,
}

#[derive(Debug, PartialEq)]
pub struct Part {
    pub x: u64,
    pub m: u64,
    pub a: u64,
    pub s: u64,
}

impl Part {
    pub fn get_field(&self, field: &Field) -> u64 {
        match field {
            Field::X => self.x,
            Field::M => self.m,
            Field::A => self.a,
            Field::S => self.s,
        }
    }

    pub fn total_rating(&self) -> u64 {
        self.x + self.m + self.a + self.s
    }
}

impl TryFrom<&str> for Part {
    type Error = AdventError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // {x=787,m=2655,a=1222,s=2876}
        let brackets: &[_] = &['{', '}'];
        let ratings = value.trim_matches(brackets);

        let mut part = Part {
            x: 0,
            m: 0,
            a: 0,
            s: 0,
        };

        for rating in ratings.split(',') {
            let (name, value) = rating
                .split_once('=')
                .ok_or(AdventError::SplitOnce(rating.into(), '='.into()))?;
            let value = value.parse::<u64>()?;

            match name {
                "x" => part.x = value,
                "m" => part.m = value,
                "a" => part.a = value,
                "s" => part.s = value,
                s => return Err(AdventError::UnknownPattern(s.into())),
            }
        }

        Ok(part)
    }
}

#[derive(Clone, Debug)]
struct PartRanged {
    x: Range<u64>,
    m: Range<u64>,
    a: Range<u64>,
    s: Range<u64>,
}

impl PartRanged {
    fn total(self) -> u64 {
        let total_x = self.x.count() as u64;
        let total_m = self.m.count() as u64;
        let total_a = self.a.count() as u64;
        let total_s = self.s.count() as u64;

        total_x * total_m * total_a * total_s
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Field {
    X,
    M,
    A,
    S,
}

impl TryFrom<char> for Field {
    type Error = AdventError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'x' => Ok(Field::X),
            'm' => Ok(Field::M),
            'a' => Ok(Field::A),
            's' => Ok(Field::S),
            _ => Err(AdventError::UnknownPattern(value.into())),
        }
    }
}
