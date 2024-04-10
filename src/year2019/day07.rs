use itertools::Itertools;

use crate::error::AdventError;

use super::int_code_computer::{IntCodeComputer, IntCodeError};

type ParsedInput = Vec<i32>;

pub fn parse(input: &str) -> color_eyre::Result<ParsedInput> {
    IntCodeComputer::parse_program(input)
}

pub fn part1(code: &ParsedInput) -> color_eyre::Result<i32> {
    let signals: Vec<_> = (0..5)
        .permutations(5)
        .map(|p| calc_thruster_signal(p, code))
        .try_collect()?;

    Ok(signals.iter().max().ok_or(AdventError::EmptySlice)?.clone())
}

fn calc_thruster_signal(phase_settings: Vec<i32>, code: &ParsedInput) -> color_eyre::Result<i32> {
    let mut io = 0;
    for phase in phase_settings {
        let mut icc = IntCodeComputer::load(code.clone());
        icc.push_input(phase);
        icc.push_input(io);

        icc.run()?;

        io = icc.get_last_output()?;
    }

    Ok(io)
}

pub fn part2(code: &ParsedInput) -> color_eyre::Result<i32> {
    let signals: Vec<_> = (5..10)
        .permutations(5)
        .map(|p| calc_thruster_signal_with_feedback(p, code))
        .try_collect()?;

    Ok(signals.iter().max().ok_or(AdventError::EmptySlice)?.clone())
}

fn calc_thruster_signal_with_feedback(
    phase_settings: Vec<i32>,
    code: &ParsedInput,
) -> color_eyre::Result<i32> {
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
            return Ok(amp_e.get_last_output()?);
        }
    }
}

fn new_amplifier(code: &ParsedInput, phase_setting: i32) -> IntCodeComputer {
    let mut amp = IntCodeComputer::load(code.clone());
    amp.enable_input_yield();
    amp.push_input(phase_setting);

    amp
}

fn run_amplifier(amp: &mut IntCodeComputer, input: i32) -> color_eyre::Result<i32> {
    amp.push_input(input);

    if let Err(report) = amp.run() {
        let root_cause = report.root_cause();

        if let Some(IntCodeError::Yield) = root_cause.downcast_ref() {
            amp.get_last_output()
        } else {
            Err(report.wrap_err("Failed to run Amplifier A"))
        }
    } else {
        amp.get_last_output()
    }
}
