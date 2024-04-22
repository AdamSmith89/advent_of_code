use itertools::Itertools;

use crate::error::AdventError;

use super::int_code_computer::{IcProgram, IntCodeComputer};

type ParsedInput = IcProgram;

pub fn parse(input: &str) -> color_eyre::Result<ParsedInput> {
    IntCodeComputer::parse_program(input)
}

pub fn part1(code: &ParsedInput) -> color_eyre::Result<i64> {
    let signals: Vec<_> = (0..5)
        .permutations(5)
        .map(|p| calc_thruster_signal(p, code))
        .try_collect()?;

    Ok(*signals.iter().max().ok_or(AdventError::EmptySlice)?)
}

fn calc_thruster_signal(phase_settings: Vec<i64>, code: &ParsedInput) -> color_eyre::Result<i64> {
    let mut io = 0;
    for phase in phase_settings {
        let mut icc = IntCodeComputer::load(code.clone());
        icc.push_input(phase);
        icc.push_input(io);

        icc.run()?;

        io = *icc
            .last_output()
            .ok_or(AdventError::LogicError(String::from(
                "No output from amplifier",
            )))?;
    }

    Ok(io)
}

pub fn part2(code: &ParsedInput) -> color_eyre::Result<i64> {
    let signals: Vec<_> = (5..10)
        .permutations(5)
        .map(|p| calc_thruster_signal_with_feedback(p, code))
        .try_collect()?;

    Ok(*signals.iter().max().ok_or(AdventError::EmptySlice)?)
}

fn calc_thruster_signal_with_feedback(
    phase_settings: Vec<i64>,
    code: &ParsedInput,
) -> color_eyre::Result<i64> {
    let mut amp_a = new_amplifier(code, phase_settings[0]);
    let mut amp_b = new_amplifier(code, phase_settings[1]);
    let mut amp_c = new_amplifier(code, phase_settings[2]);
    let mut amp_d = new_amplifier(code, phase_settings[3]);
    let mut amp_e = new_amplifier(code, phase_settings[4]);

    let mut amp_a_input = 0;
    loop {
        let a_to_b = run_amplifier(&mut amp_a, amp_a_input)?;
        let b_to_c = run_amplifier(&mut amp_b, a_to_b)?;
        let c_to_d = run_amplifier(&mut amp_c, b_to_c)?;
        let d_to_e = run_amplifier(&mut amp_d, c_to_d)?;
        amp_a_input = run_amplifier(&mut amp_e, d_to_e)?;

        if amp_e.has_halted() {
            return amp_e
                .last_output()
                .cloned()
                .ok_or(AdventError::LogicError(String::from("No output from amplifier")).into());
        }
    }
}

fn new_amplifier(code: &ParsedInput, phase_setting: i64) -> IntCodeComputer {
    let mut amp = IntCodeComputer::load(code.clone());
    amp.enable_input_yield();
    amp.push_input(phase_setting);

    amp
}

fn run_amplifier(amp: &mut IntCodeComputer, input: i64) -> color_eyre::Result<i64> {
    amp.push_input(input);
    amp.run()?;
    amp.last_output()
        .cloned()
        .ok_or(AdventError::LogicError(String::from("No output from amplifier")).into())
}
