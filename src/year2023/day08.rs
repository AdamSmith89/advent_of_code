use itertools::Itertools;
use num::integer::lcm;
use std::collections::HashMap;

type ParsedInput = Docs;
use crate::error::AdventError;

pub fn parse(input: &str) -> color_eyre::Result<ParsedInput> {
    let mut lines = input.lines();
    let steps = lines
        .next()
        .ok_or(AdventError::EndOfIterator)?
        .chars()
        .collect_vec();

    lines.next(); // Skip the blank line

    let mut network = HashMap::new();

    for line in lines {
        let (node, links) = line
            .split_once('=')
            .ok_or(AdventError::SplitOnce(line.into(), '='.into()))?;
        let node = node.trim().to_string();

        let (link_l, link_r) = links
            .split_once(',')
            .ok_or(AdventError::SplitOnce(links.into(), ','.into()))?;
        let link_l = link_l
            .trim()
            .strip_prefix('(')
            .unwrap_or(link_l)
            .to_string();
        let link_r = link_r
            .trim()
            .strip_suffix(')')
            .unwrap_or(link_r)
            .to_string();

        network.insert(node, (link_l, link_r));
    }

    Ok(Docs { steps, network })
}

pub fn part1(docs: &ParsedInput) -> color_eyre::Result<u64> {
    let start_node = "AAA".to_string();

    Ok(solve(docs, &start_node, |node| *node == "ZZZ".to_string())?)
}

pub fn part2(docs: &ParsedInput) -> color_eyre::Result<u64> {
    // Simultaneously start on every node that ends with A. How many steps does it take before you're only on nodes that end with Z?

    let start_nodes = docs
        .network
        .keys()
        .filter(|key| key.ends_with('A'))
        .cloned()
        .collect_vec();

    let at_end = |node: &String| node.ends_with('Z');

    let steps = start_nodes
        .iter()
        .map(|start_node| solve(docs, &start_node, at_end))
        .collect::<Result<Vec<u64>, AdventError>>()?;

    let coal = steps
        .into_iter()
        .coalesce(|prev, cur| Ok(lcm(prev, cur)))
        .collect_vec();

    Ok(coal[0])
}

fn solve<F>(docs: &Docs, start_node: &String, at_end: F) -> Result<u64, AdventError>
where
    F: Fn(&String) -> bool,
{
    let mut cur_node = start_node.clone();
    let mut dir_iter = docs.steps.iter().cycle();
    let mut steps_taken = 0;

    while !at_end(&cur_node) {
        let next = docs
            .network
            .get(&cur_node)
            .ok_or(AdventError::NotFound(cur_node.into()))?;

        let dir = dir_iter.next().unwrap();
        cur_node = match dir {
            'L' => Ok(next.0.clone()),
            'R' => Ok(next.1.clone()),
            _ => Err(AdventError::UnknownPattern(dir.to_string())),
        }?;

        steps_taken += 1;
    }

    Ok(steps_taken)
}

#[derive(Debug, Eq, PartialEq)]
pub struct Docs {
    pub steps: Vec<char>,
    pub network: HashMap<String, (String, String)>,
}
