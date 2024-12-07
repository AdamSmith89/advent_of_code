use std::{collections::HashMap, num::ParseIntError};

use itertools::Itertools;
use log::debug;

use crate::error::AdventError;

type ParsedInput = (HashMap<u32, Vec<Order>>, Vec<Update>);

pub fn parse(input: &str) -> color_eyre::Result<ParsedInput> {
    let (rules, updates) = input.split_once("\n\n").ok_or(AdventError::SplitOnce(
        "input".to_string(),
        "empty newline".to_string(),
    ))?;

    let raw_rules: Vec<(u32, u32)> = rules
        .lines()
        .map(|line| {
            line.split_once('|')
                .ok_or(AdventError::SplitOnce(line.to_string(), "|".to_string()))
        })
        .map(|r| {
            let (before, after) = r?;
            Ok::<(u32, u32), AdventError>((before.parse::<u32>()?, after.parse::<u32>()?))
        })
        .try_collect()?;

    let mut rules = HashMap::new();
    for (first, second) in raw_rules {
        rules
            .entry(first)
            .and_modify(|orders: &mut Vec<Order>| {
                orders.push(Order::Before(second));
            })
            .or_insert_with(|| vec![Order::Before(second)]);

        rules
            .entry(second)
            .and_modify(|orders: &mut Vec<Order>| {
                orders.push(Order::After(first));
            })
            .or_insert_with(|| vec![Order::After(first)]);
    }

    let updates: Vec<Update> = updates
        .lines()
        .map(|line| line.split(',').collect_vec())
        .map(|pages| pages.iter().map(|page| page.parse::<u32>()).collect_vec())
        .map(|pages| {
            let pages: Result<Vec<u32>, ParseIntError> = pages.into_iter().collect();

            Ok::<Update, ParseIntError>(Update { pages: pages? })
        })
        .try_collect()?;

    debug!("===Rules===\n{rules:?}");
    debug!("===Updates===\n{updates:?}");

    Ok((rules, updates))
}

pub fn part1((rules, updates): &ParsedInput) -> color_eyre::Result<u32> {
    let mut mid_total = 0;

    'update_loop: for update in updates {
        for (idx, page) in update.pages.iter().enumerate() {
            let prev_pages = &update.pages[..idx];
            let next_pages = &update.pages[idx + 1..];

            let page_rules = rules
                .get(page)
                .ok_or(AdventError::NotFound(page.to_string()))?;

            for rule in page_rules {
                let rule_ok = match rule {
                    Order::Before(val) => !prev_pages.contains(val),
                    Order::After(val) => !next_pages.contains(val),
                };

                if !rule_ok {
                    continue 'update_loop;
                }
            }
        }

        // Page must be safe by this point
        let mid = update.pages.len() / 2;
        mid_total += update.pages[mid];
    }

    Ok(mid_total)
}

pub fn part2((rules, updates): &ParsedInput) -> color_eyre::Result<u32> {
    let mut mid_total = 0;

    'update_loop: for update in updates {
        for (idx, page) in update.pages.iter().enumerate() {
            let prev_pages = &update.pages[..idx];
            let next_pages = &update.pages[idx + 1..];

            let page_rules = rules
                .get(page)
                .ok_or(AdventError::NotFound(page.to_string()))?;

            for rule in page_rules {
                let rule_ok = match rule {
                    Order::Before(val) => !prev_pages.contains(val),
                    Order::After(val) => !next_pages.contains(val),
                };

                if !rule_ok {
                    // For each of the incorrectly-ordered updates,
                    // use the page ordering rules to put the page numbers in the right order
                    let fixed_update = rebuild_update(&update.pages, rules);

                    let mid = fixed_update.len() / 2;
                    mid_total += fixed_update[mid];

                    continue 'update_loop;
                }
            }
        }
    }

    Ok(mid_total)
}

fn rebuild_update(pages: &[u32], rules: &HashMap<u32, Vec<Order>>) -> Vec<u32> {
    let mut new = pages.to_owned();

    new.sort_by(|lhs, rhs| {
        if let Some(page_rules) = rules.get(lhs) {
            if let Some(order) = page_rules.iter().find(|o| match o {
                Order::Before(v) => v == rhs,
                Order::After(v) => v == rhs,
            }) {
                match order {
                    Order::Before(_) => std::cmp::Ordering::Less,
                    Order::After(_) => std::cmp::Ordering::Greater,
                }
            } else {
                std::cmp::Ordering::Equal
            }
        } else {
            std::cmp::Ordering::Equal
        }
    });

    new
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Order {
    Before(u32),
    After(u32),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Update {
    pub pages: Vec<u32>,
}
