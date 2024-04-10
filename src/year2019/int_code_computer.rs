use std::{
    collections::VecDeque,
    ops::{Add, Mul},
};

use itertools::Itertools;

use crate::error::AdventError;

use super::op_code::{Mode, OpCode};

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum IntCodeError {
    #[error("Yielding")]
    Yield,
}

pub struct IntCodeComputer {
    code: Vec<i32>,
    ip: usize,
    input_buffer: VecDeque<i32>,
    output_buffer: Vec<i32>,
    yield_for_input: bool,
    halted: bool,
}

impl IntCodeComputer {
    pub fn parse_program(input: &str) -> color_eyre::Result<Vec<i32>> {
        Ok(input
            .lines()
            .next()
            .ok_or(AdventError::EndOfIterator)?
            .split(',')
            .map(|s| s.parse::<i32>())
            .try_collect()?)
    }

    pub fn load(code: Vec<i32>) -> Self {
        Self {
            code,
            ip: 0,
            input_buffer: VecDeque::new(),
            output_buffer: Vec::new(),
            yield_for_input: false,
            halted: false,
        }
    }

    pub fn enable_input_yield(&mut self) {
        self.yield_for_input = true;
    }

    pub fn run(&mut self) -> color_eyre::Result<()> {
        //println!("{:?}", self.code);
        loop {
            match self.parse_instruction()? {
                OpCode::Add(mode1, mode2) => {
                    let param1 = self.next_with_mode(mode1)?;
                    let param2 = self.next_with_mode(mode2)?;

                    //println!("  {param1} + {param2}");
                    self.binary_op(param1, param2, Add::add)?
                }
                OpCode::Mul(mode1, mode2) => {
                    let param1 = self.next_with_mode(mode1)?;
                    let param2 = self.next_with_mode(mode2)?;

                    //println!("  {param1} x {param2}");
                    self.binary_op(param1, param2, Mul::mul)?
                }
                OpCode::In => self.input()?,
                OpCode::Out(mode) => {
                    let value = self.next_with_mode(mode)?;

                    //println!("  {value}");
                    self.output(value)?
                }
                OpCode::JNZ(mode1, mode2) => {
                    let value = self.next_with_mode(mode1)?;
                    let jmp_addr = self.next_with_mode(mode2)?;

                    //println!("  if {value} != 0 then jmp {jmp_addr}");
                    self.jmp_op(value, jmp_addr as usize, PartialEq::ne)?
                }
                OpCode::JZ(mode1, mode2) => {
                    let value = self.next_with_mode(mode1)?;
                    let jmp_addr = self.next_with_mode(mode2)?;

                    //println!("  if {value} == 0 then jmp {jmp_addr}");
                    self.jmp_op(value, jmp_addr as usize, PartialEq::eq)?
                }
                OpCode::LT(mode1, mode2) => {
                    let param1 = self.next_with_mode(mode1)?;
                    let param2 = self.next_with_mode(mode2)?;

                    //println!("  {param1} < {param2}");
                    self.cmp_op(param1, param2, PartialOrd::lt)?
                }
                OpCode::EQ(mode1, mode2) => {
                    let param1 = self.next_with_mode(mode1)?;
                    let param2 = self.next_with_mode(mode2)?;

                    //println!("  {param1} == {param2}");
                    self.cmp_op(param1, param2, PartialEq::eq)?
                }
                OpCode::End => {
                    self.halted = true;
                    break;
                }
            }
        }

        Ok(())
    }

    pub fn push_input(&mut self, input: i32) {
        self.input_buffer.push_back(input);
    }

    pub fn get_output(&self) -> &Vec<i32> {
        &self.output_buffer
    }

    pub fn get_last_output(&self) -> color_eyre::Result<i32> {
        self.output_buffer
            .last()
            .cloned()
            .ok_or(AdventError::EmptySlice.into())
    }

    pub fn read(&self, addr: usize) -> color_eyre::Result<i32> {
        self.code
            .get(addr)
            .cloned()
            .ok_or(AdventError::NotFound(addr.to_string()).into())
    }

    pub fn has_halted(&self) -> bool {
        self.halted
    }

    fn write(&mut self, addr: usize, val: i32) -> color_eyre::Result<()> {
        if let Some(cur_val) = self.code.get_mut(addr) {
            //println!("  Writing {val} to {addr}");
            *cur_val = val;
            Ok(())
        } else {
            Err(AdventError::NotFound(addr.to_string()).into())
        }
    }

    fn parse_instruction(&mut self) -> color_eyre::Result<OpCode> {
        //print!("[{}] ", self.ip);

        let raw_val = self.next()?;
        let op = OpCode::try_from(raw_val)?;

        //let upper = self.code.len().min(self.ip + op.num_params());
        //let context = &self.code[self.ip - 1..upper];
        //println!("{op:?} ({context:?})");

        Ok(op)
    }

    // Reads value at instruction pointer then advances it one step
    fn next(&mut self) -> color_eyre::Result<i32> {
        let value = self.read(self.ip);
        self.ip += 1;

        value
    }

    fn next_with_mode(&mut self, mode: Mode) -> color_eyre::Result<i32> {
        let value = self.next()?;
        match mode {
            Mode::Pos => self.read(value as usize),
            Mode::Imm => Ok(value),
        }
    }

    fn binary_op<Op>(&mut self, param1: i32, param2: i32, op: Op) -> color_eyre::Result<()>
    where
        Op: FnOnce(i32, i32) -> i32,
    {
        // Opcode 1 adds together numbers read from two positions and stores the result in a third position.
        // Opcode 2 multiplies together numbers read from two positions and stores the result in a third position.
        // The three integers immediately after the opcode tell you these three positions -
        // the first two indicate the positions from which you should read the input values,
        // and the third indicates the position at which the output should be stored.

        // Parameters that an instruction writes to will always be in Position Mode
        let addr = self.next()? as usize;
        let result = op(param1, param2);
        self.write(addr, result)?;

        Ok(())
    }

    fn jmp_op<Op>(&mut self, value: i32, jmp_addr: usize, op: Op) -> color_eyre::Result<()>
    where
        Op: FnOnce(&i32, &i32) -> bool,
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

    fn cmp_op<Op>(&mut self, param1: i32, param2: i32, op: Op) -> color_eyre::Result<()>
    where
        Op: FnOnce(&i32, &i32) -> bool,
    {
        // Opcode 7 is less than: if the first parameter is less than the second parameter,
        // Opcode 8 is equals: if the first parameter is equal to the second parameter
        // it stores 1 in the position given by the third parameter.
        // Otherwise, it stores 0.

        // Parameters that an instruction writes to will always be in Position Mode
        let addr = self.next()? as usize;
        if op(&param1, &param2) {
            self.write(addr, 1)?;
        } else {
            self.write(addr, 0)?;
        }

        Ok(())
    }

    fn input(&mut self) -> color_eyre::Result<()> {
        // Opcode 3 takes a single integer as input and saves it to the position given by its only parameter.
        // For example, the instruction 3,50 would take an input value and store it at address 50.

        if let Some(input) = self.input_buffer.pop_front() {
            // Parameters that an instruction writes to will always be in Position Mode
            let addr = self.next()?;
            self.write(addr as usize, input)?;

            Ok(())
        } else if self.yield_for_input {
            // We are yielding so will need to re-run this input instruction
            self.ip -= 1;
            Err(IntCodeError::Yield.into())
        } else {
            Err(AdventError::NotFound(String::from("Input value")).into())
        }
    }

    fn output(&mut self, value: i32) -> color_eyre::Result<()> {
        // Opcode 4 outputs the value of its only parameter.
        // For example, the instruction 4,50 would output the value at address 50.

        // Is this the best way to represent output?
        self.output_buffer.push(value);

        Ok(())
    }
}

#[cfg(test)]
mod icc_tests {
    use super::IntCodeComputer;

    #[test]
    fn icc_end() {
        let mut icc = IntCodeComputer::load(vec![99]);

        assert!(icc.run().is_ok());
        assert_eq!(vec![99], icc.code);
    }

    #[test]
    fn icc_add() {
        let mut icc = IntCodeComputer::load(vec![1, 0, 0, 0, 99]);

        assert!(icc.run().is_ok());
        assert_eq!(vec![2, 0, 0, 0, 99], icc.code);
    }

    #[test]
    fn icc_mul() {
        let mut icc = IntCodeComputer::load(vec![2, 3, 0, 3, 99]);

        assert!(icc.run().is_ok());
        assert_eq!(vec![2, 3, 0, 6, 99], icc.code);
    }

    #[test]
    fn icc_in() {
        let mut icc = IntCodeComputer::load(vec![3, 3, 99, 0]);
        icc.push_input(8);

        assert!(icc.run().is_ok());
        assert_eq!(vec![3, 3, 99, 8], icc.code);
    }

    #[test]
    fn icc_out() {
        let mut icc = IntCodeComputer::load(vec![4, 3, 104, 8, 99]);

        assert!(icc.run().is_ok());
        assert_eq!(vec![4, 3, 104, 8, 99], icc.code);
        assert_eq!(&vec![8, 8], icc.get_output());
    }

    #[test]
    fn icc_jnz_true() {
        // 0 is an invalid OpCode, the JNZ should set the ip to [99]
        let mut icc = IntCodeComputer::load(vec![1105, 1, 4, 0, 99]);

        assert!(icc.run().is_ok());
        assert_eq!(vec![1105, 1, 4, 0, 99], icc.code);
    }

    #[test]
    fn icc_jnz_false() {
        // 0 is an invalid OpCode, the JNZ should not change the ip
        let mut icc = IntCodeComputer::load(vec![1105, 0, 4, 99, 0]);

        assert!(icc.run().is_ok());
        assert_eq!(vec![1105, 0, 4, 99, 0], icc.code);
    }

    #[test]
    fn icc_jz_true() {
        // 0 is an invalid OpCode, the JZ should set the ip to [99]
        let mut icc = IntCodeComputer::load(vec![1106, 0, 4, 0, 99]);

        assert!(icc.run().is_ok());
        assert_eq!(vec![1106, 0, 4, 0, 99], icc.code);
    }

    #[test]
    fn icc_jz_false() {
        // 0 is an invalid OpCode, the JZ should not change the ip
        let mut icc = IntCodeComputer::load(vec![1106, 1, 4, 99, 0]);

        assert!(icc.run().is_ok());
        assert_eq!(vec![1106, 1, 4, 99, 0], icc.code);
    }

    #[test]
    fn icc_lt_true() {
        // Set [6] to 1 as [5](0) < 1
        let mut icc = IntCodeComputer::load(vec![1007, 5, 1, 6, 99, 0, 0]);

        assert!(icc.run().is_ok());
        assert_eq!(vec![1007, 5, 1, 6, 99, 0, 1], icc.code);
    }

    #[test]
    fn icc_lt_false() {
        // Set [6] to 0 as [5](1) > 0
        let mut icc = IntCodeComputer::load(vec![1007, 5, 0, 6, 99, 1, 1]);

        assert!(icc.run().is_ok());
        assert_eq!(vec![1007, 5, 0, 6, 99, 1, 0], icc.code);
    }

    #[test]
    fn icc_lt_eq() {
        // Set [6] to 0 as [5](1) == 1
        let mut icc = IntCodeComputer::load(vec![1007, 5, 1, 6, 99, 1, 1]);

        assert!(icc.run().is_ok());
        assert_eq!(vec![1007, 5, 1, 6, 99, 1, 0], icc.code);
    }

    #[test]
    fn icc_eq_true() {
        // Set [6] to 1 as [5](1) == 1
        let mut icc = IntCodeComputer::load(vec![1008, 5, 1, 6, 99, 1, 0]);

        assert!(icc.run().is_ok());
        assert_eq!(vec![1008, 5, 1, 6, 99, 1, 1], icc.code);
    }

    #[test]
    fn icc_eq_false() {
        // Set [6] to 0 as [5](1) != 0
        let mut icc = IntCodeComputer::load(vec![1008, 5, 0, 6, 99, 1, 1]);

        assert!(icc.run().is_ok());
        assert_eq!(vec![1008, 5, 0, 6, 99, 1, 0], icc.code);
    }

    #[test]
    fn icc_add_neg() {
        let mut icc = IntCodeComputer::load(vec![1101, -4, 3, 0, 99]);

        assert!(icc.run().is_ok());
        assert_eq!(vec![-1, -4, 3, 0, 99], icc.code);
    }

    #[test]
    fn icc_add_imm() {
        let mut icc = IntCodeComputer::load(vec![1101, 2, 4, 5, 99, 0]);

        assert!(icc.run().is_ok());
        assert_eq!(vec![1101, 2, 4, 5, 99, 6], icc.code);
    }

    #[test]
    fn icc_day05_part2_example1() {
        // Using position mode, consider whether the input is equal to 8;
        // output 1 (if it is) or 0 (if it is not).
        let mut icc = IntCodeComputer::load(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]);
        icc.push_input(8);

        assert!(icc.run().is_ok());
        assert_eq!(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, 1, 8], icc.code);
        assert_eq!(&vec![1], icc.get_output());
    }

    #[test]
    fn icc_day05_part2_example2() {
        // Using position mode, consider whether the input is less than 8;
        // output 1 (if it is) or 0 (if it is not).
        let mut icc = IntCodeComputer::load(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]);
        icc.push_input(10);

        assert!(icc.run().is_ok());
        assert_eq!(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, 0, 8], icc.code);
        assert_eq!(&vec![0], icc.get_output());
    }

    #[test]
    fn icc_day05_part2_example3() {
        // Using immediate mode, consider whether the input is equal to 8;
        // output 1 (if it is) or 0 (if it is not).
        let mut icc = IntCodeComputer::load(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99]);
        icc.push_input(10);

        assert!(icc.run().is_ok());
        assert_eq!(vec![3, 3, 1108, 0, 8, 3, 4, 3, 99], icc.code);
        assert_eq!(&vec![0], icc.get_output());
    }

    #[test]
    fn icc_day05_part2_example4() {
        // Using immediate mode, consider whether the input is less than 8;
        // output 1 (if it is) or 0 (if it is not).
        let mut icc = IntCodeComputer::load(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]);
        icc.push_input(5);

        assert!(icc.run().is_ok());
        assert_eq!(vec![3, 3, 1107, 1, 8, 3, 4, 3, 99], icc.code);
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
            vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, 0, 0, 1, 9],
            icc.code
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
            vec![3, 3, 1105, 5, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
            icc.code
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
}
