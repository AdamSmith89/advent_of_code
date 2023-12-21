use std::collections::{HashMap, VecDeque};

use itertools::Itertools;
use num::integer::lcm;

use crate::error::AdventError;

pub type ParsedInput = HashMap<String, Module>;

pub fn parse(input: &str) -> color_eyre::Result<ParsedInput> {
    let mut modules: HashMap<String, Module> = input
        .lines()
        .map(to_module)
        .map_ok(|(id, module)| (id.clone(), module))
        .try_collect()?;

    let conjunctions = modules
        .iter_mut()
        .filter_map(|(key, module)| {
            if let Module::Conjuction(_) = module {
                Some(key.clone())
            } else {
                None
            }
        })
        .collect_vec();

    for conjunction in conjunctions {
        let inputs = modules
            .iter()
            .filter_map(|(id, module)| {
                if module.targets().unwrap().contains(&conjunction) {
                    Some(id.to_owned())
                } else {
                    None
                }
            })
            .collect_vec();

        if let Module::Conjuction(conjunction) = modules.get_mut(&conjunction).unwrap() {
            conjunction.set_inputs(inputs);
        }
    }

    modules.insert("rx".to_string(), Module::Rx(Rx::new()));

    Ok(modules)
}

pub fn part1(modules: &ParsedInput) -> color_eyre::Result<usize> {
    let mut modules = (*modules).clone();
    let mut sent_freq = Vec::new();

    for press in 0..1000 {
        push_button(&mut sent_freq, &mut modules, &press);
    }

    let (low, high): (Vec<Frequency>, Vec<Frequency>) =
        sent_freq.iter().partition(|&freq| *freq == Frequency::Low);

    Ok(low.len() * high.len())
}

pub fn part2(modules: &ParsedInput) -> color_eyre::Result<usize> {
    // Thanks to Reddit, we know that the modules are arranged in a graph like this:
    // https://www.reddit.com/r/adventofcode/comments/18mypla/2023_day_20_input_data_plot/?utm_source=share&utm_medium=web2x&context=3
    // In my input...
    /*
                        rx
                        ^
                        |
                       &zh
                       ^
                _ __ __|__ ____
               |    |    |    |
              &xc  &th  &pd  &bp
               ^    ^    ^    ^
               |    |    |    |
              &ps  &kh  &mk  &ml
               ^    ^    ^    ^
               |    |    |    |
               %*   %*   %*   %*

        where %* is a series of flip-flops
    */
    // Remember that...
    // Conjunction modules (prefix &) remember the type of the most recent pulse received from each o
    // their connected input modules; they initially default to remembering a low pulse for each input.
    // When a pulse is received, the conjunction module first updates its memory for that input. Then,
    // if it remembers high pulses for all inputs, it sends a low pulse; otherwise, it sends a high pulse.
    // So...
    // To get a low-pulse to rx we need a low-pulse from zh
    // To send a low pulse, zh needs a high pulse from all inputs (xc, th, pd, bp)
    // To send a high pulse they need to have a low input from at least 1 of their inputs (ps, kh, mk, ml respectively)
    // To send a low pulse they need to have all high pulses from their inputs
    // So...
    // We iterate until each of ps, kh, mk, and ml have all of their inputs registering high - recording
    // the number of button presses for each one.
    // Then we sum # of presses for each Flip-Flops connected to those Conjunctions to get the number of
    // iterations each one needs to send a low pulse
    // Then we work out the least-common-multiple of those numbers to get the number of presses needed
    // to align them all together - which would flip zh, sending a low pulse to rx

    let mut modules = (*modules).clone();
    let mut sent_freq = Vec::new();
    let mut button_presses = 1u32;
    let mut stored_sum = vec![("ps", 0), ("kh", 0), ("mk", 0), ("ml", 0)];

    loop {
        push_button(&mut sent_freq, &mut modules, &button_presses);

        // For the conjunctions we care about, check the ones which haven't stored a sum yet
        for (id, sum) in stored_sum.iter_mut() {
            if *sum == 0 {
                if let Some(Module::Conjuction(c)) = modules.get(*id) {
                    if c.all_inputs_seen_high() {
                        *sum = c.sum_input_memory();
                    }
                }
            }
        }

        if stored_sum.iter().all(|(_, sum)| *sum != 0) {
            break;
        }

        button_presses += 1;
    }

    let coal = stored_sum
        .into_iter()
        .map(|(_, lcm)| lcm)
        .coalesce(|prev, cur| Ok(lcm(prev, cur)))
        .collect_vec();

    Ok(coal[0])
}

fn push_button(
    sent_freq: &mut Vec<Frequency>,
    modules: &mut HashMap<String, Module>,
    button_presses: &u32,
) {
    let mut pulses = VecDeque::from([Pulse {
        from: "".to_string(),
        freq: Frequency::Low,
        targets: vec!["broadcaster".to_string()],
    }]);

    sent_freq.push(Frequency::Low);

    while let Some(pulse) = pulses.pop_front() {
        for target_id in &pulse.targets {
            if let Some(module) = modules.get_mut(target_id) {
                if let Some(output_freq) =
                    module.process_freq(pulse.freq, &pulse.from, button_presses)
                {
                    if let Some(targets) = module.targets() {
                        for _ in 0..targets.len() {
                            sent_freq.push(output_freq);
                        }

                        let new_pulse = Pulse {
                            from: target_id.clone(),
                            freq: output_freq,
                            targets: targets.clone(),
                        };

                        pulses.push_back(new_pulse);
                    }
                }
            }
        }
    }
}

fn to_module(value: &str) -> color_eyre::Result<(String, Module)> {
    let (type_id, targets) = value
        .split_once(" -> ")
        .ok_or(AdventError::SplitOnce(value.into(), " -> ".into()))?;

    let targets: Vec<String> = targets.split(", ").map_into().collect();

    if let Some(id) = type_id.strip_prefix('%') {
        Ok((id.to_string(), Module::FlipFlop(FlipFlop::new(targets))))
    } else if let Some(id) = type_id.strip_prefix('&') {
        Ok((
            id.to_string(),
            Module::Conjuction(Conjunction::new(targets)),
        ))
    } else {
        Ok((
            type_id.to_string(),
            Module::Broadcaster(Broadcaster::new(targets)),
        ))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Module {
    Broadcaster(Broadcaster),
    FlipFlop(FlipFlop),
    Conjuction(Conjunction),
    Rx(Rx),
}

impl Module {
    pub fn process_freq(
        &mut self,
        freq: Frequency,
        from: &str,
        button_presses: &u32,
    ) -> Option<Frequency> {
        match self {
            Module::Broadcaster(b) => b.process_freq(freq, from),
            Module::FlipFlop(f) => f.process_freq(freq, from),
            Module::Conjuction(c) => c.process_freq(freq, from, button_presses),
            Module::Rx(r) => r.process_freq(freq, from),
        }
    }

    pub fn targets(&self) -> Option<&Vec<String>> {
        match self {
            Module::Broadcaster(b) => Some(&b.targets),
            Module::FlipFlop(f) => Some(&f.targets),
            Module::Conjuction(c) => Some(&c.targets),
            Module::Rx(_) => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Broadcaster {
    pub targets: Vec<String>,
}

impl Broadcaster {
    fn new(targets: Vec<String>) -> Self {
        Broadcaster { targets }
    }

    fn process_freq(&mut self, freq: Frequency, _from: &str) -> Option<Frequency> {
        Some(freq)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FlipFlop {
    pub targets: Vec<String>,
    pub on_off: bool,
}

impl FlipFlop {
    fn new(targets: Vec<String>) -> Self {
        FlipFlop {
            targets,
            on_off: false,
        }
    }

    fn process_freq(&mut self, freq: Frequency, _from: &str) -> Option<Frequency> {
        match freq {
            Frequency::High => None,
            Frequency::Low => {
                self.on_off = !self.on_off;

                if self.on_off {
                    Some(Frequency::High)
                } else {
                    Some(Frequency::Low)
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Conjunction {
    pub targets: Vec<String>,
    pub inputs: Vec<(String, Frequency, Option<u32>)>,
}

impl Conjunction {
    fn new(targets: Vec<String>) -> Self {
        Conjunction {
            targets,
            inputs: Vec::new(),
        }
    }

    fn set_inputs(&mut self, inputs: Vec<String>) {
        self.inputs = inputs
            .iter()
            .map(|id| (id.to_owned(), Frequency::Low, None))
            .collect();
    }

    fn process_freq(
        &mut self,
        freq: Frequency,
        from: &str,
        button_presses: &u32,
    ) -> Option<Frequency> {
        if let Some((_, stored_freq, presses_at_high)) =
            self.inputs.iter_mut().find(|(id, _, _)| id == from)
        {
            *stored_freq = freq;

            if freq == Frequency::High && presses_at_high.is_none() {
                *presses_at_high = Some(*button_presses);
            }
        }

        if self.all_inputs_high() {
            Some(Frequency::Low)
        } else {
            Some(Frequency::High)
        }
    }

    fn all_inputs_high(&self) -> bool {
        self.inputs
            .iter()
            .all(|(_, freq, _)| *freq == Frequency::High)
    }

    fn all_inputs_seen_high(&self) -> bool {
        self.inputs
            .iter()
            .all(|(_, _, presses_at_high)| presses_at_high.is_some())
    }

    fn sum_input_memory(&self) -> usize {
        self.inputs
            .iter()
            .map(|(_, _, presses_at_high)| presses_at_high.unwrap() as usize)
            .sum()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Rx {
    pub last_input: Option<Frequency>,
}

impl Rx {
    fn new() -> Self {
        Rx { last_input: None }
    }

    fn process_freq(&mut self, freq: Frequency, _from: &str) -> Option<Frequency> {
        self.last_input = Some(freq);
        None
    }
}

#[derive(Debug)]
pub struct Pulse {
    from: String,
    freq: Frequency,
    targets: Vec<String>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Frequency {
    High,
    Low,
}
