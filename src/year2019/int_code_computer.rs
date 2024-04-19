use std::{
    collections::VecDeque,
    ops::{Add, Mul},
};

use itertools::Itertools;
use simple_logger::SimpleLogger;

use crate::error::AdventError;

use super::op_code::{Mode, OpCode};

pub type IccProgram = Vec<i64>;

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum IntCodeError {
    #[error("Yielding")]
    Yield,
    #[error("Invalid memory address {0}")]
    InvalidAccess(isize),
    #[error("No input value provided")]
    NoInput,
}

pub struct IntCodeComputer {
    memory: Vec<i64>,
    ip: usize,
    rel_base: usize,

    input_buffer: VecDeque<i64>,
    output_buffer: Vec<i64>,

    yield_for_input: bool,
    halted: bool,
}

impl IntCodeComputer {
    pub fn parse_program(input: &str) -> color_eyre::Result<IccProgram> {
        Ok(input
            .lines()
            .next()
            .ok_or(AdventError::EndOfIterator)?
            .split(',')
            .map(|s| s.parse::<i64>())
            .try_collect()?)
    }

    pub fn load(code: Vec<i64>) -> Self {
        let mut memory = code;
        memory.resize(2056, -1);

        Self {
            memory,
            ip: 0,
            rel_base: 0,
            input_buffer: VecDeque::new(),
            output_buffer: Vec::new(),
            yield_for_input: false,
            halted: false,
        }
    }

    pub fn enable_input_yield(&mut self) {
        self.yield_for_input = true;
    }

    pub fn enable_debug_mode(&mut self) -> color_eyre::Result<()> {
        Ok(SimpleLogger::new().init()?)
    }

    pub fn run(&mut self) -> color_eyre::Result<()> {
        log::debug!("{:?}", self.memory);
        loop {
            match self.parse_instruction()? {
                OpCode::Add(mode1, mode2, mode3) => {
                    let param1 = self.next_with_mode(mode1, AccessType::Read)?;
                    let param2 = self.next_with_mode(mode2, AccessType::Read)?;
                    let write_addr = self.next_with_mode(mode3, AccessType::Write)? as usize;

                    log::debug!("  {param1} + {param2}");
                    self.binary_op(param1, param2, write_addr, Add::add)?
                }
                OpCode::Mul(mode1, mode2, mode3) => {
                    let param1 = self.next_with_mode(mode1, AccessType::Read)?;
                    let param2 = self.next_with_mode(mode2, AccessType::Read)?;
                    let write_addr = self.next_with_mode(mode3, AccessType::Write)? as usize;

                    log::debug!("  {param1} x {param2}");
                    self.binary_op(param1, param2, write_addr, Mul::mul)?
                }
                OpCode::In(mode) => {
                    let addr = self.next_with_mode(mode, AccessType::Write)?;

                    self.input(addr as usize)?
                }
                OpCode::Out(mode) => {
                    let value = self.next_with_mode(mode, AccessType::Read)?;

                    log::debug!("  {value}");
                    self.output(value)?
                }
                OpCode::JNZ(mode1, mode2) => {
                    let value = self.next_with_mode(mode1, AccessType::Read)?;
                    let jmp_addr = self.next_with_mode(mode2, AccessType::Read)?;

                    log::debug!("  if {value} != 0 then jmp {jmp_addr}");
                    self.jmp_op(value, jmp_addr as usize, PartialEq::ne)?
                }
                OpCode::JZ(mode1, mode2) => {
                    let value = self.next_with_mode(mode1, AccessType::Read)?;
                    let jmp_addr = self.next_with_mode(mode2, AccessType::Read)?;

                    log::debug!("  if {value} == 0 then jmp {jmp_addr}");
                    self.jmp_op(value, jmp_addr as usize, PartialEq::eq)?
                }
                OpCode::LT(mode1, mode2, mode3) => {
                    let param1 = self.next_with_mode(mode1, AccessType::Read)?;
                    let param2 = self.next_with_mode(mode2, AccessType::Read)?;
                    let write_addr = self.next_with_mode(mode3, AccessType::Write)? as usize;

                    log::debug!("  {param1} < {param2}");
                    self.cmp_op(param1, param2, write_addr, PartialOrd::lt)?
                }
                OpCode::EQ(mode1, mode2, mode3) => {
                    let param1 = self.next_with_mode(mode1, AccessType::Read)?;
                    let param2 = self.next_with_mode(mode2, AccessType::Read)?;
                    let write_addr = self.next_with_mode(mode3, AccessType::Write)? as usize;

                    log::debug!("  {param1} == {param2}");
                    self.cmp_op(param1, param2, write_addr, PartialEq::eq)?
                }
                OpCode::RBO(mode) => {
                    let param = self.next_with_mode(mode, AccessType::Read)?;

                    log::debug!("  adjust rel_base by {param}");
                    self.adj_rel_base(param)?
                }
                OpCode::End => {
                    self.halted = true;
                    break;
                }
            }
        }

        Ok(())
    }

    pub fn push_input(&mut self, input: i64) {
        self.input_buffer.push_back(input);
    }

    pub fn get_output(&self) -> &Vec<i64> {
        &self.output_buffer
    }

    pub fn get_last_output(&self) -> Option<&i64> {
        self.output_buffer.last()
    }

    pub fn has_halted(&self) -> bool {
        self.halted
    }

    pub fn read(&self, addr: usize) -> Result<i64, IntCodeError> {
        self.memory
            .get(addr)
            .cloned()
            .ok_or(IntCodeError::InvalidAccess(addr as isize))
    }

    // Currently only used for unit testing
    fn _read_block(&self, addr: usize, len: usize) -> Result<&[i64], IntCodeError> {
        let end = addr + len;

        self.memory
            .get(addr..end)
            .ok_or(IntCodeError::InvalidAccess(addr as isize))
    }

    fn write(&mut self, addr: usize, val: i64) -> Result<(), IntCodeError> {
        if let Some(cur_val) = self.memory.get_mut(addr) {
            log::debug!("  Writing {val} to {addr}");
            *cur_val = val;
            Ok(())
        } else {
            Err(IntCodeError::InvalidAccess(addr as isize))
        }
    }

    fn parse_instruction(&mut self) -> color_eyre::Result<OpCode> {
        let raw_val = self.next()?;
        let op = OpCode::try_from(raw_val)?;

        let upper = self.memory.len().min(self.ip + op.num_params());
        let context = &self.memory[self.ip - 1..upper];
        log::debug!("[{}] {op:?} ({context:?})", self.ip - 1);

        Ok(op)
    }

    // Reads value at instruction pointer then advances it one step
    fn next(&mut self) -> Result<i64, IntCodeError> {
        let value = self.read(self.ip);
        self.ip += 1;

        value
    }

    // Read returns the value at the address specified by the next parameter based on the mode
    // Write returns the value of the next parameter as an address based on the mode
    fn next_with_mode(&mut self, mode: Mode, access_type: AccessType) -> Result<i64, IntCodeError> {
        match access_type {
            AccessType::Read => {
                let value = self.next()?;
                match mode {
                    Mode::Pos => self.read(value as usize),
                    Mode::Imm => Ok(value),
                    Mode::Rel => {
                        let addr = self.rel_base.checked_add_signed(value as isize).ok_or(
                            IntCodeError::InvalidAccess(self.rel_base as isize - value as isize),
                        )?;
                        self.read(addr)
                    }
                }
            }
            AccessType::Write => match mode {
                Mode::Pos => self.next(),
                Mode::Imm => {
                    panic!("Parameters an instruction write to will never be in immediate mode")
                }
                Mode::Rel => {
                    let offset = self.next()?;
                    Ok(self.rel_base.checked_add_signed(offset as isize).ok_or(
                        IntCodeError::InvalidAccess(self.rel_base as isize - offset as isize),
                    )? as i64)
                }
            },
        }
    }

    fn binary_op<Op>(
        &mut self,
        param1: i64,
        param2: i64,
        write_addr: usize,
        op: Op,
    ) -> color_eyre::Result<()>
    where
        Op: FnOnce(i64, i64) -> i64,
    {
        // Opcode 1 adds together numbers read from two positions and stores the result in a third position.
        // Opcode 2 multiplies together numbers read from two positions and stores the result in a third position.
        // The three integers immediately after the opcode tell you these three positions -
        // the first two indicate the positions from which you should read the input values,
        // and the third indicates the position at which the output should be stored.
        let result = op(param1, param2);
        self.write(write_addr, result)?;

        Ok(())
    }

    fn jmp_op<Op>(&mut self, value: i64, jmp_addr: usize, op: Op) -> color_eyre::Result<()>
    where
        Op: FnOnce(&i64, &i64) -> bool,
    {
        // Opcode 5 is jump-if-true: if the first parameter is non-zero,
        // Opcode 6 is jump-if-false: if the first parameter is zero,
        // it sets the instruction pointer to the value from the second parameter.
        // Otherwise, it does nothing.

        if op(&value, &0) {
            self.ip = jmp_addr;
        }

        Ok(())
    }

    fn cmp_op<Op>(
        &mut self,
        param1: i64,
        param2: i64,
        write_addr: usize,
        op: Op,
    ) -> color_eyre::Result<()>
    where
        Op: FnOnce(&i64, &i64) -> bool,
    {
        // Opcode 7 is less than: if the first parameter is less than the second parameter,
        // Opcode 8 is equals: if the first parameter is equal to the second parameter
        // it stores 1 in the position given by the third parameter.
        // Otherwise, it stores 0.

        if op(&param1, &param2) {
            self.write(write_addr, 1)?;
        } else {
            self.write(write_addr, 0)?;
        }

        Ok(())
    }

    fn input(&mut self, addr: usize) -> Result<(), IntCodeError> {
        // Opcode 3 takes a single integer as input and saves it to the position given by its only parameter.
        // For example, the instruction 3,50 would take an input value and store it at address 50.

        if let Some(input) = self.input_buffer.pop_front() {
            self.write(addr, input)?;
            Ok(())
        } else if self.yield_for_input {
            // We are yielding so will need to re-run this input instruction
            self.ip -= 2;
            Err(IntCodeError::Yield.into())
        } else {
            Err(IntCodeError::NoInput)
        }
    }

    fn output(&mut self, value: i64) -> color_eyre::Result<()> {
        // Opcode 4 outputs the value of its only parameter.
        // For example, the instruction 4,50 would output the value at address 50.

        // Is this the best way to represent output?
        self.output_buffer.push(value);

        Ok(())
    }

    fn adj_rel_base(&mut self, value: i64) -> color_eyre::Result<()> {
        // Opcode 9 adjusts the relative base address by the value of its only parameter
        // For example, the instruction 109, -10 would dec
        self.rel_base =
            self.rel_base
                .checked_add_signed(value as isize)
                .ok_or(IntCodeError::InvalidAccess(
                    self.rel_base as isize - value as isize,
                ))?;

        Ok(())
    }
}

enum AccessType {
    Read,
    Write,
}

#[cfg(test)]
mod icc_tests {
    use super::IntCodeComputer;

    #[test]
    fn icc_read_block_ok() {
        let icc = IntCodeComputer::load(vec![1, 2, 3, 4, 5]);

        assert_eq!(Ok(&[2, 3, 4][..]), icc._read_block(1, 3));
    }

    #[test]
    fn icc_read_block_err_too_long() {
        // ICC defaults to having 2056 memory slots filled with -1 - it doesn't yet have the
        // concept of unintialized memory
        let icc = IntCodeComputer::load(vec![1, 2, 3, 4, 5]);

        assert!(icc._read_block(1, 10000).is_err());
    }

    #[test]
    fn icc_read_block_err_out_of_bounds() {
        // ICC defaults to having 2056 memory slots filled with -1 - it doesn't yet have the
        // concept of unintialized memory
        let icc = IntCodeComputer::load(vec![1, 2, 3, 4, 5]);

        assert!(icc._read_block(10000, 3).is_err());
    }

    #[test]
    fn icc_end() {
        let mut icc = IntCodeComputer::load(vec![99]);

        assert!(icc.run().is_ok());
        assert_eq!(Ok(&[99][..]), icc._read_block(0, 1));
    }

    #[test]
    fn icc_add() {
        let mut icc = IntCodeComputer::load(vec![1, 0, 0, 0, 99]);

        assert!(icc.run().is_ok());
        assert_eq!(Ok(&[2, 0, 0, 0, 99][..]), icc._read_block(0, 5));
    }

    #[test]
    fn icc_mul() {
        let mut icc = IntCodeComputer::load(vec![2, 3, 0, 3, 99]);

        assert!(icc.run().is_ok());
        assert_eq!(Ok(&[2, 3, 0, 6, 99][..]), icc._read_block(0, 5));
    }

    #[test]
    fn icc_in() {
        let mut icc = IntCodeComputer::load(vec![3, 3, 99, 0]);
        icc.push_input(8);

        assert!(icc.run().is_ok());
        assert_eq!(Ok(&[3, 3, 99, 8][..]), icc._read_block(0, 4));
    }

    #[test]
    fn icc_out() {
        let mut icc = IntCodeComputer::load(vec![4, 3, 104, 8, 99]);

        assert!(icc.run().is_ok());
        assert_eq!(Ok(&[4, 3, 104, 8, 99][..]), icc._read_block(0, 5));
        assert_eq!(&vec![8, 8], icc.get_output());
    }

    #[test]
    fn icc_jnz_true() {
        // 0 is an invalid OpCode, the JNZ should set the ip to [99]
        let mut icc = IntCodeComputer::load(vec![1105, 1, 4, 0, 99]);

        assert!(icc.run().is_ok());
        assert_eq!(Ok(&[1105, 1, 4, 0, 99][..]), icc._read_block(0, 5));
    }

    #[test]
    fn icc_jnz_false() {
        // 0 is an invalid OpCode, the JNZ should not change the ip
        let mut icc = IntCodeComputer::load(vec![1105, 0, 4, 99, 0]);

        assert!(icc.run().is_ok());
        assert_eq!(Ok(&[1105, 0, 4, 99, 0][..]), icc._read_block(0, 5));
    }

    #[test]
    fn icc_jz_true() {
        // 0 is an invalid OpCode, the JZ should set the ip to [99]
        let mut icc = IntCodeComputer::load(vec![1106, 0, 4, 0, 99]);

        assert!(icc.run().is_ok());
        assert_eq!(Ok(&[1106, 0, 4, 0, 99][..]), icc._read_block(0, 5));
    }

    #[test]
    fn icc_jz_false() {
        // 0 is an invalid OpCode, the JZ should not change the ip
        let mut icc = IntCodeComputer::load(vec![1106, 1, 4, 99, 0]);

        assert!(icc.run().is_ok());
        assert_eq!(Ok(&[1106, 1, 4, 99, 0][..]), icc._read_block(0, 5));
    }

    #[test]
    fn icc_lt_true() {
        // Set [6] to 1 as [5](0) < 1
        let mut icc = IntCodeComputer::load(vec![1007, 5, 1, 6, 99, 0, 0]);

        assert!(icc.run().is_ok());
        assert_eq!(Ok(&[1007, 5, 1, 6, 99, 0, 1][..]), icc._read_block(0, 7));
    }

    #[test]
    fn icc_lt_false() {
        // Set [6] to 0 as [5](1) > 0
        let mut icc = IntCodeComputer::load(vec![1007, 5, 0, 6, 99, 1, 1]);

        assert!(icc.run().is_ok());
        assert_eq!(Ok(&[1007, 5, 0, 6, 99, 1, 0][..]), icc._read_block(0, 7));
    }

    #[test]
    fn icc_lt_eq() {
        // Set [6] to 0 as [5](1) == 1
        let mut icc = IntCodeComputer::load(vec![1007, 5, 1, 6, 99, 1, 1]);

        assert!(icc.run().is_ok());
        assert_eq!(Ok(&[1007, 5, 1, 6, 99, 1, 0][..]), icc._read_block(0, 7));
    }

    #[test]
    fn icc_eq_true() {
        // Set [6] to 1 as [5](1) == 1
        let mut icc = IntCodeComputer::load(vec![1008, 5, 1, 6, 99, 1, 0]);

        assert!(icc.run().is_ok());
        assert_eq!(Ok(&[1008, 5, 1, 6, 99, 1, 1][..]), icc._read_block(0, 7));
    }

    #[test]
    fn icc_eq_false() {
        // Set [6] to 0 as [5](1) != 0
        let mut icc = IntCodeComputer::load(vec![1008, 5, 0, 6, 99, 1, 1]);

        assert!(icc.run().is_ok());
        assert_eq!(Ok(&[1008, 5, 0, 6, 99, 1, 0][..]), icc._read_block(0, 7));
    }

    #[test]
    fn icc_add_neg() {
        let mut icc = IntCodeComputer::load(vec![1101, -4, 3, 0, 99]);

        assert!(icc.run().is_ok());
        assert_eq!(Ok(&[-1, -4, 3, 0, 99][..]), icc._read_block(0, 5));
    }

    #[test]
    fn icc_add_imm() {
        let mut icc = IntCodeComputer::load(vec![1101, 2, 4, 5, 99, 0]);

        assert!(icc.run().is_ok());
        assert_eq!(Ok(&[1101, 2, 4, 5, 99, 6][..]), icc._read_block(0, 6));
    }

    #[test]
    fn icc_rbo_imm_inc() {
        let mut icc = IntCodeComputer::load(vec![109, 19, 99]);

        assert!(icc.run().is_ok());
        assert_eq!(19, icc.rel_base);
    }

    #[test]
    fn icc_rbo_pos_dec() {
        let mut icc = IntCodeComputer::load(vec![09, 3, 99, -10]);
        icc.rel_base = 100;

        assert!(icc.run().is_ok());
        assert_eq!(90, icc.rel_base);
    }

    #[test]
    fn icc_rbo_dec_err() {
        let mut icc = IntCodeComputer::load(vec![109, -5, 99]);

        assert!(icc.run().is_err());
    }

    #[test]
    fn icc_add_rel() {
        let mut icc = IntCodeComputer::load(vec![2201, 1, 2, 5, 99, 0, 5, 6]);
        icc.rel_base = 5;

        assert!(icc.run().is_ok());
        assert_eq!(
            Ok(&[2201, 1, 2, 5, 99, 11, 5, 6][..]),
            icc._read_block(0, 8)
        );
    }

    #[test]
    fn icc_day05_part2_example1() {
        // Using position mode, consider whether the input is equal to 8;
        // output 1 (if it is) or 0 (if it is not).
        let mut icc = IntCodeComputer::load(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]);
        //let _ = icc.enable_debug_mode();
        icc.push_input(8);

        assert!(icc.run().is_ok());
        assert_eq!(
            Ok(&[3, 9, 8, 9, 10, 9, 4, 9, 99, 1, 8][..]),
            icc._read_block(0, 11)
        );
        assert_eq!(&vec![1], icc.get_output());
    }

    #[test]
    fn icc_day05_part2_example2() {
        // Using position mode, consider whether the input is less than 8;
        // output 1 (if it is) or 0 (if it is not).
        let mut icc = IntCodeComputer::load(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]);
        icc.push_input(10);

        assert!(icc.run().is_ok());
        assert_eq!(
            Ok(&[3, 9, 7, 9, 10, 9, 4, 9, 99, 0, 8][..]),
            icc._read_block(0, 11)
        );
        assert_eq!(&vec![0], icc.get_output());
    }

    #[test]
    fn icc_day05_part2_example3() {
        // Using immediate mode, consider whether the input is equal to 8;
        // output 1 (if it is) or 0 (if it is not).
        let mut icc = IntCodeComputer::load(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99]);
        icc.push_input(10);

        assert!(icc.run().is_ok());
        assert_eq!(
            Ok(&[3, 3, 1108, 0, 8, 3, 4, 3, 99][..]),
            icc._read_block(0, 9)
        );
        assert_eq!(&vec![0], icc.get_output());
    }

    #[test]
    fn icc_day05_part2_example4() {
        // Using immediate mode, consider whether the input is less than 8;
        // output 1 (if it is) or 0 (if it is not).
        let mut icc = IntCodeComputer::load(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]);
        icc.push_input(5);

        assert!(icc.run().is_ok());
        assert_eq!(
            Ok(&[3, 3, 1107, 1, 8, 3, 4, 3, 99][..]),
            icc._read_block(0, 9)
        );
        assert_eq!(&vec![1], icc.get_output());
    }

    #[test]
    fn icc_day05_part2_example5() {
        // Using position mode, take an input,
        // then output 0 if the input was zero or 1 if the input was non-zero
        let mut icc = IntCodeComputer::load(vec![
            3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9,
        ]);
        icc.push_input(0);

        assert!(icc.run().is_ok());
        assert_eq!(
            Ok(&[3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, 0, 0, 1, 9][..]),
            icc._read_block(0, 16)
        );
        assert_eq!(&vec![0], icc.get_output());
    }

    #[test]
    fn icc_day05_part2_example6() {
        // Using immediate mode, take an input,
        // then output 0 if the input was zero or 1 if the input was non-zero
        let mut icc = IntCodeComputer::load(vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]);
        icc.push_input(5);

        assert!(icc.run().is_ok());
        assert_eq!(
            Ok(&[3, 3, 1105, 5, 9, 1101, 0, 0, 12, 4, 12, 99, 1][..]),
            icc._read_block(0, 13)
        );
        assert_eq!(&vec![1], icc.get_output());
    }

    // year2019::day05 part 2 big example
    // The example program uses an input instruction to ask for a single number.
    // The program will then output 999 if the input value is below 8,
    // ...
    #[test]
    fn icc_day05_part2_big_example_lt() {
        let mut icc = IntCodeComputer::load(vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ]);
        icc.push_input(5);

        assert!(icc.run().is_ok());
        assert_eq!(&vec![999], icc.get_output());
    }

    // ...
    // output 1000 if the input value is equal to 8,
    // ...
    #[test]
    fn icc_day05_part2_big_example_eq() {
        let mut icc = IntCodeComputer::load(vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ]);
        icc.push_input(8);

        assert!(icc.run().is_ok());
        assert_eq!(&vec![1000], icc.get_output());
    }

    // ...
    // or output 1001 if the input value is greater than 8.
    #[test]
    fn icc_day05_part2_big_example_gt() {
        let mut icc = IntCodeComputer::load(vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ]);
        icc.push_input(15);

        assert!(icc.run().is_ok());
        assert_eq!(&vec![1001], icc.get_output());
    }

    #[test]
    fn icc_day09_part1_more_memory() {
        // Takes no input and poduces a copy of itself as output
        let mut icc = IntCodeComputer::load(vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 15, 101, 1006, 101, 0, 99,
        ]);

        assert!(icc.run().is_ok());
        assert_eq!(
            &vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 15, 101, 1006, 101, 0, 99,],
            icc.get_output()
        );
    }

    #[test]
    fn icc_day09_part1_big_number_support_1() {
        let mut icc = IntCodeComputer::load(vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0]);

        assert!(icc.run().is_ok());
        assert_eq!(&vec![1219070632396864], icc.get_output());
    }

    #[test]
    fn icc_day09_part1_big_number_support_2() {
        let mut icc = IntCodeComputer::load(vec![104, 1125899906842624, 99]);

        assert!(icc.run().is_ok());
        assert_eq!(&vec![1125899906842624], icc.get_output());
    }
}
